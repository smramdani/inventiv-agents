use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::infrastructure::database::DatabasePool;
use crate::domain::identity::{organization::Organization, user::{User, UserRole}};
use crate::error::AppResult;

#[derive(Deserialize)]
pub struct RegisterOrgRequest {
    pub name: String,
    pub admin_email: String,
    pub locale: String,
}

#[derive(Serialize)]
pub struct RegisterOrgResponse {
    pub org_id: String,
    pub admin_id: String,
}

pub async fn register_organization(
    State(db): State<DatabasePool>,
    Json(payload): Json<RegisterOrgRequest>,
) -> AppResult<Json<RegisterOrgResponse>> {
    // 1. Create domain objects and validate
    let org_domain = Organization::new(&payload.name, &payload.locale)
        .map_err(|e| crate::error::AppError::Validation(e.to_string()))?;
    
    let user_domain = User::new(&org_domain, &payload.admin_email, UserRole::Owner)
        .map_err(|e| crate::error::AppError::Validation(e.to_string()))?;

    // 2. Persist in a transaction
    let mut tx = db.get_pool().begin().await?;

    // IMPORTANT: For initial setup, we don't set RLS yet as it's the creation phase
    // but in every other handler, we'll use db.set_rls_context()
    
    sqlx::query("INSERT INTO organizations (id, name, default_locale) VALUES ($1, $2, $3)")
        .bind(org_domain.id)
        .bind(&org_domain.name)
        .bind(&org_domain.default_locale)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO users (id, organization_id, email, role) VALUES ($1, $2, $3, $4::user_role)")
        .bind(user_domain.id)
        .bind(user_domain.organization_id)
        .bind(&user_domain.email)
        .bind("Owner")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(Json(RegisterOrgResponse {
        org_id: org_domain.id.to_string(),
        admin_id: user_domain.id.to_string(),
    }))
}
