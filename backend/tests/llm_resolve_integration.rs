//! Integration: DB seed → `openai_compatible_client_for_agent` → HTTP mock (wiremock).
//!
//! Validates `get_agent_by_id`, `get_llm_provider_with_key`, RLS context, and `LlmCompletionPort`
//! without calling a real LLM.

mod common;

use dotenvy::dotenv;
use inventivagents::domain::agents::agent::Agent;
use inventivagents::domain::agents::provider::LlmProvider;
use inventivagents::domain::engine::{
    ChatRole, LlmCompletionPort, LlmCompletionRequest, LlmMessage,
};
use inventivagents::infrastructure::database::agents::AgentsRepository;
use inventivagents::infrastructure::database::DatabasePool;
use inventivagents::infrastructure::llm::openai_compatible_client_for_agent;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
#[serial(integration_db)]
async fn resolve_llm_client_from_db_and_complete_via_mock() -> anyhow::Result<()> {
    dotenv().ok();

    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("authorization", "Bearer sk-from-db"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{ "message": { "role": "assistant", "content": "from-mock" } }],
            "usage": { "prompt_tokens": 2, "completion_tokens": 7 }
        })))
        .mount(&mock)
        .await;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_id = Uuid::new_v4();
    common::insert_org(&pool, org_id, "Org LLM Resolve").await?;

    let base_url = mock.uri();
    let provider = LlmProvider::new(org_id, "Mock LLM", base_url.trim_end_matches('/'))?;
    let agent = Agent::new(org_id, "Test Agent", "Mission", Some(provider.id))?;

    let mut tx = pool.begin().await?;
    AgentsRepository::insert_llm_provider(&mut tx, org_id, &provider, Some("sk-from-db")).await?;
    AgentsRepository::insert_agent(&mut tx, org_id, &agent).await?;
    tx.commit().await?;

    let db = DatabasePool::connect(&common::app_database_url()).await?;
    let client = openai_compatible_client_for_agent(&db, org_id, agent.id).await?;

    let out = client
        .complete(LlmCompletionRequest {
            messages: vec![LlmMessage {
                role: ChatRole::User,
                content: "hello".into(),
            }],
            model: "any-model".into(),
            max_tokens: 50,
        })
        .await?;

    assert_eq!(out.content, "from-mock");
    assert_eq!(out.input_tokens, 2);
    assert_eq!(out.output_tokens, 7);

    Ok(())
}

#[tokio::test]
#[serial(integration_db)]
async fn resolve_fails_when_agent_has_no_provider() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_id = Uuid::new_v4();
    common::insert_org(&pool, org_id, "Org No Provider").await?;

    let agent = Agent::new(org_id, "Lonely", "Mission", None)?;
    let mut tx = pool.begin().await?;
    AgentsRepository::insert_agent(&mut tx, org_id, &agent).await?;
    tx.commit().await?;

    let db = DatabasePool::connect(&common::app_database_url()).await?;
    let err = openai_compatible_client_for_agent(&db, org_id, agent.id)
        .await
        .expect_err("expected validation error");
    let msg = err.to_string();
    assert!(msg.contains("no LLM provider"), "unexpected message: {msg}");

    Ok(())
}
