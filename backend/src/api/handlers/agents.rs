use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::middleware::auth::AuthenticatedUser;
use crate::domain::agents::agent::Agent;
use crate::domain::agents::provider::LlmProvider;
use crate::domain::agents::skill::{Skill, SkillType};
use crate::domain::identity::user::UserRole;
use crate::error::{AppError, AppResult};
use crate::infrastructure::auth::jwt::Claims;
use crate::infrastructure::database::agents::AgentsRepository;
use crate::infrastructure::database::DatabasePool;

fn require_admin_or_owner(claims: &Claims) -> AppResult<()> {
    match claims.role {
        UserRole::Owner | UserRole::Admin => Ok(()),
        UserRole::User => Err(AppError::Unauthorized),
    }
}

#[derive(Deserialize)]
pub struct CreateProviderRequest {
    pub name: String,
    pub base_url: String,
    /// Persisted in `api_key_encrypted`; replace with KMS-backed ciphertext for production.
    pub api_key: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateSkillRequest {
    pub name: String,
    pub skill_type: SkillType,
    pub endpoint_url: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub mission: String,
    pub llm_provider_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct CreateEntityResponse {
    pub id: String,
}

pub async fn create_provider(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(payload): Json<CreateProviderRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    require_admin_or_owner(&claims)?;
    let org_id = claims.org_id;

    let provider = LlmProvider::new(org_id, &payload.name, &payload.base_url)
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let mut tx = db.get_pool().begin().await?;
    AgentsRepository::insert_llm_provider(&mut tx, org_id, &provider, payload.api_key.as_deref())
        .await?;
    tx.commit().await?;

    tracing::info!(org_id = %org_id, provider_id = %provider.id, "LLM provider created");

    Ok(Json(CreateEntityResponse {
        id: provider.id.to_string(),
    }))
}

pub async fn list_providers(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> AppResult<Json<Vec<LlmProvider>>> {
    require_admin_or_owner(&claims)?;
    let org_id = claims.org_id;

    let mut tx = db.get_pool().begin().await?;
    let list = AgentsRepository::list_llm_providers(&mut tx, org_id).await?;
    tx.commit().await?;

    Ok(Json(list))
}

pub async fn create_skill(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(payload): Json<CreateSkillRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    require_admin_or_owner(&claims)?;
    let org_id = claims.org_id;

    let skill = Skill::new(
        org_id,
        &payload.name,
        payload.skill_type,
        payload.endpoint_url,
    )
    .map_err(|e| AppError::Validation(e.to_string()))?;

    let mut tx = db.get_pool().begin().await?;
    AgentsRepository::insert_skill(&mut tx, org_id, &skill).await?;
    tx.commit().await?;

    tracing::info!(org_id = %org_id, skill_id = %skill.id, "Skill created");

    Ok(Json(CreateEntityResponse {
        id: skill.id.to_string(),
    }))
}

pub async fn list_skills(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> AppResult<Json<Vec<Skill>>> {
    require_admin_or_owner(&claims)?;
    let org_id = claims.org_id;

    let mut tx = db.get_pool().begin().await?;
    let list = AgentsRepository::list_skills(&mut tx, org_id).await?;
    tx.commit().await?;

    Ok(Json(list))
}

pub async fn create_agent(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(payload): Json<CreateAgentRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    require_admin_or_owner(&claims)?;
    let org_id = claims.org_id;

    let agent = Agent::new(
        org_id,
        &payload.name,
        &payload.mission,
        payload.llm_provider_id,
    )
    .map_err(|e| AppError::Validation(e.to_string()))?;

    let mut tx = db.get_pool().begin().await?;
    AgentsRepository::insert_agent(&mut tx, org_id, &agent).await?;
    tx.commit().await?;

    tracing::info!(org_id = %org_id, agent_id = %agent.id, "Agent created");

    Ok(Json(CreateEntityResponse {
        id: agent.id.to_string(),
    }))
}

/// Any authenticated org member may list agents (e.g. cockpit chat picker for `User` role).
pub async fn list_agents(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> AppResult<Json<Vec<Agent>>> {
    let org_id = claims.org_id;

    let mut tx = db.get_pool().begin().await?;
    let list = AgentsRepository::list_agents(&mut tx, org_id).await?;
    tx.commit().await?;

    Ok(Json(list))
}

pub async fn link_agent_skill(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path((agent_id, skill_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    require_admin_or_owner(&claims)?;
    let org_id = claims.org_id;

    let mut tx = db.get_pool().begin().await?;
    AgentsRepository::link_agent_skill(&mut tx, org_id, agent_id, skill_id).await?;
    tx.commit().await?;

    tracing::info!(%org_id, %agent_id, %skill_id, "Agent skill linked");

    Ok(Json(serde_json::json!({
        "agent_id": agent_id.to_string(),
        "skill_id": skill_id.to_string(),
        "linked": true
    })))
}
