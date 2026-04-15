//! Integration: authenticated `POST /org/agents/:id/complete/stream` returns SSE (wiremock LLM).

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use dotenvy::dotenv;
use http_body_util::BodyExt;
use inventivagents::api::app_router;
use inventivagents::domain::agents::agent::Agent;
use inventivagents::domain::agents::provider::LlmProvider;
use inventivagents::infrastructure::database::agents::AgentsRepository;
use inventivagents::infrastructure::database::DatabasePool;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;
use uuid::Uuid;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
#[serial(integration_db)]
async fn sse_stream_returns_events_and_trace() -> anyhow::Result<()> {
    dotenv().ok();

    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("authorization", "Bearer sk-sse"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{ "message": { "role": "assistant", "content": "sse-ok" } }],
            "usage": { "prompt_tokens": 1, "completion_tokens": 2 }
        })))
        .mount(&mock)
        .await;

    let raw_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_id = Uuid::new_v4();
    common::insert_org(&raw_pool, org_id, "Org SSE").await?;
    let admin_id = common::insert_admin_user(&raw_pool, org_id).await?;
    let token = common::admin_bearer_token(admin_id, org_id)?;

    let base_url = mock.uri();
    let provider = LlmProvider::new(org_id, "SSE Prov", base_url.trim_end_matches('/'))?;
    let agent = Agent::new(org_id, "Stream Agent", "Mission", Some(provider.id))?;

    let mut tx = raw_pool.begin().await?;
    AgentsRepository::insert_llm_provider(&mut tx, org_id, &provider, Some("sk-sse")).await?;
    AgentsRepository::insert_agent(&mut tx, org_id, &agent).await?;
    tx.commit().await?;

    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    let trace = Uuid::parse_str("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")?;
    let uri = format!("/org/agents/{}/complete/stream", agent.id);
    let body = serde_json::json!({
        "message": "hello",
        "model": "test-model",
        "max_tokens": 64
    });

    let req = Request::builder()
        .method("POST")
        .uri(&uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", token))
        .header("x-trace-id", trace.to_string())
        .body(Body::from(serde_json::to_vec(&body)?))?;

    let res = app.oneshot(req).await?;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok()),
        Some("text/event-stream")
    );
    let resp_trace = res
        .headers()
        .get("x-trace-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert_eq!(resp_trace, trace.to_string());

    let bytes = res.into_body().collect().await?.to_bytes();
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("event: meta"), "body:\n{text}");
    assert!(text.contains("trace_id"));
    assert!(text.contains("event: delta"));
    assert!(text.contains("sse-ok"));
    assert!(text.contains("event: usage"));
    assert!(text.contains("event: done"));

    Ok(())
}
