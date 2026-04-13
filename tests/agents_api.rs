mod common;

use axum::body::Body;
use serial_test::serial;
use axum::http::{Request, StatusCode};
use axum::Router;
use dotenvy::dotenv;
use http_body_util::BodyExt;
use inventivagents::api::app_router;
use inventivagents::domain::identity::user::UserRole;
use inventivagents::infrastructure::auth::jwt::JwtService;
use inventivagents::infrastructure::database::DatabasePool;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;
use uuid::Uuid;

async fn insert_org(pool: &sqlx::PgPool, org_id: Uuid, label: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
        .bind(org_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO organizations (id, name, default_locale) VALUES ($1, $2, $3)")
        .bind(org_id)
        .bind(label)
        .bind("en_US")
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

#[tokio::test]
#[serial(integration_db)]
async fn test_create_provider_requires_auth() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    let body = serde_json::json!({
        "name": "OVH",
        "base_url": "https://api.ovh.com"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/org/providers")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body)?))?;

    let res = app.oneshot(req).await?;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[tokio::test]
#[serial(integration_db)]
async fn test_admin_can_create_provider_via_http() -> anyhow::Result<()> {
    dotenv().ok();
    let raw_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_id = Uuid::new_v4();
    insert_org(&raw_pool, org_id, "Org API").await?;

    let admin_email = format!("admin_{}@example.com", Uuid::new_v4());
    {
        let mut tx = raw_pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
            .bind(org_id.to_string())
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO users (organization_id, email, role) VALUES ($1, $2, $3::user_role)",
        )
        .bind(org_id)
        .bind(&admin_email)
        .bind("Admin")
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
    }

    let (admin_id,): (Uuid,) = {
        let mut tx = raw_pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
            .bind(org_id.to_string())
            .execute(&mut *tx)
            .await?;
        let row = sqlx::query_as("SELECT id FROM users WHERE email = $1")
            .bind(&admin_email)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        row
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let jwt = JwtService::new(&jwt_secret);
    let token = jwt.create_token(admin_id, org_id, UserRole::Admin)?;

    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    let body = serde_json::json!({
        "name": "Scaleway",
        "base_url": "https://api.scaleway.com"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/org/providers")
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_vec(&body)?))?;

    let res = app.oneshot(req).await?;
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await?.to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
    assert!(v.get("id").and_then(|x| x.as_str()).is_some());
    Ok(())
}
