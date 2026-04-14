//! Shared helpers for integration tests (`mod common;` from each `tests/*.rs` crate).

use uuid::Uuid;

pub fn app_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://inventiv_app:inventiv_app_password@127.0.0.1:5432/inventiv_agents".into()
    })
}

/// Insert an organization as superuser-style bootstrap (sets `app.current_org_id` in-tx).
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
