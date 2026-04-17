use crate::api::middleware::auth::AuthenticatedUser;
use crate::domain::identity::{
    group::Group,
    organization::Organization,
    user::{User, UserRole},
};
use crate::error::{AppError, AppResult};
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::database::identity::IdentityRepository;
use crate::infrastructure::database::DatabasePool;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct InviteUserRequest {
    pub email: String,
    pub role: UserRole,
}

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
}

pub async fn register_organization(
    State(db): State<DatabasePool>,
    Json(payload): Json<RegisterOrgRequest>,
) -> AppResult<Json<RegisterOrgResponse>> {
    let org_domain = Organization::new(&payload.name, &payload.locale)
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user_domain = User::new(&org_domain, &payload.admin_email, UserRole::Owner)
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let mut tx = db.get_pool().begin().await?;

    // RLS on `organizations` / `users` requires `app.current_org_id` to match the new org.
    DatabasePool::set_rls_context(&mut tx, org_domain.id).await?;

    sqlx::query("INSERT INTO organizations (id, name, default_locale) VALUES ($1, $2, $3)")
        .bind(org_domain.id)
        .bind(&org_domain.name)
        .bind(&org_domain.default_locale)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO users (id, organization_id, email, role) VALUES ($1, $2, $3, $4::user_role)",
    )
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

pub async fn login(
    State(db): State<DatabasePool>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let user: (Uuid, Uuid, String) =
        sqlx::query_as("SELECT user_id, organization_id, role_name FROM lookup_user_for_login($1)")
            .bind(&payload.email)
            .fetch_optional(db.get_pool())
            .await?
            .ok_or(AppError::Unauthorized)?;

    let role = match user.2.as_str() {
        "Owner" => UserRole::Owner,
        "Admin" => UserRole::Admin,
        _ => UserRole::User,
    };

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let jwt_service = JwtService::new(&jwt_secret);

    let token = jwt_service
        .create_token(user.0, user.1, role)
        .map_err(|_| AppError::Internal)?;

    Ok(Json(LoginResponse { token }))
}

pub async fn invite_user(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(payload): Json<InviteUserRequest>,
) -> AppResult<Json<String>> {
    // 1. Authorization check
    if claims.role == UserRole::User {
        return Err(AppError::Unauthorized);
    }

    // 2. Domain logic
    // We mock the org for validation purposes
    let mock_org = Organization {
        id: claims.org_id,
        name: "N/A".into(),
        default_locale: "en_US".into(),
    };
    let user_domain = User::new(&mock_org, &payload.email, payload.role)
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // 3. Persist
    let mut tx = db.get_pool().begin().await?;
    IdentityRepository::create_user(&mut tx, claims.org_id, &user_domain).await?;
    tx.commit().await?;

    Ok(Json(user_domain.id.to_string()))
}

pub async fn create_group(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(payload): Json<CreateGroupRequest>,
) -> AppResult<Json<String>> {
    if claims.role == UserRole::User {
        return Err(AppError::Unauthorized);
    }

    let group_domain = Group::new(claims.org_id, &payload.name, payload.description)
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let mut tx = db.get_pool().begin().await?;
    IdentityRepository::create_group(&mut tx, claims.org_id, &group_domain).await?;
    tx.commit().await?;

    Ok(Json(group_domain.id.to_string()))
}
