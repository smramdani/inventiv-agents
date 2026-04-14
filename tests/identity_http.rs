mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use dotenvy::dotenv;
use http_body_util::BodyExt;
use inventivagents::api::app_router;
use inventivagents::infrastructure::database::DatabasePool;
use serial_test::serial;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
#[serial(integration_db)]
async fn register_login_whoami_smoke() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    let admin_email = format!("owner_{}@example.com", Uuid::new_v4());
    let reg = serde_json::json!({
        "name": "HTTP Org",
        "admin_email": admin_email,
        "locale": "en_US"
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/org/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&reg)?))?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&serde_json::json!({
                    "email": admin_email
                }))?))?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await?.to_bytes();
    let login: serde_json::Value = serde_json::from_slice(&bytes)?;
    let token = login["token"].as_str().unwrap();

    let res = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/auth/whoami")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
#[serial(integration_db)]
async fn protected_identity_routes_require_auth() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    for (method, uri, body) in [
        (
            "POST",
            "/org/users",
            Body::from(serde_json::to_vec(&serde_json::json!({
                "email": "u@example.com",
                "role": "User"
            }))?),
        ),
        (
            "POST",
            "/org/groups",
            Body::from(serde_json::to_vec(&serde_json::json!({
                "name": "G"
            }))?),
        ),
        (
            "POST",
            "/telemetry/frontend",
            Body::from(serde_json::to_vec(&serde_json::json!([]))?),
        ),
    ] {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(body)?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(
            res.status(),
            StatusCode::UNAUTHORIZED,
            "expected 401 for {method} {uri}"
        );
    }

    Ok(())
}
