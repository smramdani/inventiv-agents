//! Shared helpers for integration tests (`mod common;` from each `tests/*.rs` crate).

use inventivagents::domain::identity::user::UserRole;
use inventivagents::infrastructure::auth::jwt::JwtService;
use uuid::Uuid;

/// Connection URL for integration tests that must respect **RLS** (`inventiv_app`).
///
/// Local `.env` often sets `DATABASE_URL` to the Postgres superuser (`inventiv_user`) for
/// migrations; that role has **`BYPASSRLS`**, so RLS-focused tests would see all tenants' rows.
/// We rewrite credentials to `inventiv_app` while keeping host, port, database, and query
/// string. Set `INVENTIV_TEST_DATABASE_URL` to force an explicit URL.
pub fn app_database_url() -> String {
    if let Ok(url) = std::env::var("INVENTIV_TEST_DATABASE_URL") {
        return url;
    }

    match std::env::var("DATABASE_URL") {
        Ok(url) if db_url_auth_user(&url) == Some("inventiv_app") => url,
        Ok(url) => rewrite_db_url_to_inventiv_app(&url),
        Err(_) => {
            "postgres://inventiv_app:inventiv_app_password@127.0.0.1:5432/inventiv_agents".into()
        }
    }
}

fn db_url_auth_user(url: &str) -> Option<&str> {
    let rest = url
        .strip_prefix("postgres://")
        .or_else(|| url.strip_prefix("postgresql://"))?;
    let userinfo = rest.split('@').next()?;
    userinfo.split(':').next()
}

fn rewrite_db_url_to_inventiv_app(url: &str) -> String {
    const APP_USER: &str = "inventiv_app";
    const APP_PASS: &str = "inventiv_app_password";

    for prefix in ["postgres://", "postgresql://"] {
        if let Some(rest) = url.strip_prefix(prefix) {
            if let Some(at) = rest.find('@') {
                let host_path = &rest[at..];
                return format!("{prefix}{APP_USER}:{APP_PASS}{host_path}");
            }
        }
    }

    format!("postgres://{APP_USER}:{APP_PASS}@127.0.0.1:5432/inventiv_agents")
}

/// Insert an organization as superuser-style bootstrap (sets `app.current_org_id` in-tx).
#[allow(dead_code)]
pub async fn insert_org(pool: &sqlx::PgPool, org_id: Uuid, label: &str) -> anyhow::Result<()> {
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

/// Inserts an `Admin` user for `org_id` (RLS in-tx) and returns the new user id.
#[allow(dead_code)]
pub async fn insert_admin_user(pool: &sqlx::PgPool, org_id: Uuid) -> anyhow::Result<Uuid> {
    let email = format!("admin_{}@example.com", Uuid::new_v4());
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
        .bind(org_id.to_string())
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO users (organization_id, email, role) VALUES ($1, $2, $3::user_role)")
        .bind(org_id)
        .bind(&email)
        .bind("Admin")
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    let mut tx = pool.begin().await?;
    sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
        .bind(org_id.to_string())
        .execute(&mut *tx)
        .await?;
    let (id,): (Uuid,) = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(&email)
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(id)
}

/// JWT bearer token for an `Admin` in the given org (must match `JWT_SECRET` in env).
#[allow(dead_code)]
pub fn admin_bearer_token(user_id: Uuid, org_id: Uuid) -> anyhow::Result<String> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let jwt = JwtService::new(&jwt_secret);
    jwt.create_token(user_id, org_id, UserRole::Admin)
}
