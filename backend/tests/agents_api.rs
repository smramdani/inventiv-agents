mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use dotenvy::dotenv;
use http_body_util::BodyExt;
use inventivagents::api::app_router;
use inventivagents::domain::identity::user::UserRole;
use inventivagents::infrastructure::database::DatabasePool;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;
use uuid::Uuid;

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
    common::insert_org(&raw_pool, org_id, "Org API").await?;
    let admin_id = common::insert_admin_user(&raw_pool, org_id).await?;
    let token = common::admin_bearer_token(admin_id, org_id)?;

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

#[tokio::test]
#[serial(integration_db)]
async fn test_list_providers_requires_auth() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    let req = Request::builder()
        .method("GET")
        .uri("/org/providers")
        .body(Body::empty())?;

    let res = app.oneshot(req).await?;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[tokio::test]
#[serial(integration_db)]
async fn test_registry_http_list_create_agent_link_skill() -> anyhow::Result<()> {
    dotenv().ok();
    let raw_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_id = Uuid::new_v4();
    common::insert_org(&raw_pool, org_id, "Org Registry HTTP").await?;
    let admin_id = common::insert_admin_user(&raw_pool, org_id).await?;
    let token = common::admin_bearer_token(admin_id, org_id)?;

    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    let auth = format!("Bearer {}", token);

    let body = serde_json::json!({
        "name": "ProvA",
        "base_url": "https://api.example.com",
        "api_key": "k"
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/org/providers")
                .header("content-type", "application/json")
                .header("Authorization", &auth)
                .body(Body::from(serde_json::to_vec(&body)?))?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let prov_id: Uuid = {
        let bytes = res.into_body().collect().await?.to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&bytes)?;
        Uuid::parse_str(v.get("id").and_then(|x| x.as_str()).unwrap())?
    };

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/org/providers")
                .header("Authorization", &auth)
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let list_bytes = res.into_body().collect().await?.to_bytes();
    let providers: Vec<serde_json::Value> = serde_json::from_slice(&list_bytes)?;
    assert!(
        providers.iter().any(|p| p["name"] == "ProvA"),
        "list providers should include created row"
    );

    let skill_body = serde_json::json!({
        "name": "NativeSkill",
        "skill_type": "Native"
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/org/skills")
                .header("content-type", "application/json")
                .header("Authorization", &auth)
                .body(Body::from(serde_json::to_vec(&skill_body)?))?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let skill_id: Uuid = {
        let bytes = res.into_body().collect().await?.to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&bytes)?;
        Uuid::parse_str(v.get("id").and_then(|x| x.as_str()).unwrap())?
    };

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/org/skills")
                .header("Authorization", &auth)
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    let agent_body = serde_json::json!({
        "name": "HTTP Agent",
        "mission": "Do things",
        "llm_provider_id": prov_id
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/org/agents")
                .header("content-type", "application/json")
                .header("Authorization", &auth)
                .body(Body::from(serde_json::to_vec(&agent_body)?))?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let agent_id: Uuid = {
        let bytes = res.into_body().collect().await?.to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&bytes)?;
        Uuid::parse_str(v.get("id").and_then(|x| x.as_str()).unwrap())?
    };

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/org/agents")
                .header("Authorization", &auth)
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    let link_uri = format!("/org/agents/{}/skills/{}", agent_id, skill_id);
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&link_uri)
                .header("Authorization", &auth)
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await?.to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
    assert_eq!(v["linked"], true);

    Ok(())
}

#[tokio::test]
#[serial(integration_db)]
async fn test_org_user_can_list_agents_read_only() -> anyhow::Result<()> {
    dotenv().ok();
    let raw_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&common::app_database_url())
        .await?;

    let org_id = Uuid::new_v4();
    common::insert_org(&raw_pool, org_id, "Org User List Agents").await?;
    let admin_id = common::insert_admin_user(&raw_pool, org_id).await?;
    let member_id = common::insert_user_with_role(&raw_pool, org_id, UserRole::User).await?;

    let admin_token = common::admin_bearer_token(admin_id, org_id)?;
    let user_token = common::role_bearer_token(member_id, org_id, UserRole::User)?;

    let pool = DatabasePool::connect(&common::app_database_url()).await?;
    let app: Router = app_router(pool);

    let body = serde_json::json!({
        "name": "P",
        "base_url": "https://api.example.com",
        "api_key": "k"
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/org/providers")
                .header("content-type", "application/json")
                .header("Authorization", format!("Bearer {}", admin_token))
                .body(Body::from(serde_json::to_vec(&body)?))?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let prov_id: Uuid = {
        let bytes = res.into_body().collect().await?.to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&bytes)?;
        Uuid::parse_str(v.get("id").and_then(|x| x.as_str()).unwrap())?
    };

    let agent_body = serde_json::json!({
        "name": "Shared Agent",
        "mission": "Help",
        "llm_provider_id": prov_id
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/org/agents")
                .header("content-type", "application/json")
                .header("Authorization", format!("Bearer {}", admin_token))
                .body(Body::from(serde_json::to_vec(&agent_body)?))?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    let res = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/org/agents")
                .header("Authorization", format!("Bearer {}", user_token))
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await?.to_bytes();
    let list: Vec<serde_json::Value> = serde_json::from_slice(&bytes)?;
    assert!(
        list.iter()
            .any(|a| a["name"].as_str() == Some("Shared Agent")),
        "member should see org agents"
    );

    Ok(())
}
