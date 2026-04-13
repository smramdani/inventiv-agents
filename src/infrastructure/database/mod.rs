use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use uuid::Uuid;
use crate::error::AppResult;

#[derive(Clone)]
pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        
        Ok(Self { pool })
    }

    /// Returns a reference to the underlying pool.
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    /// Sets the RLS context for a transaction.
    /// This must be called inside every transaction that interacts with organization-isolated data.
    pub async fn set_rls_context(tx: &mut Transaction<'_, Postgres>, org_id: Uuid) -> AppResult<()> {
        sqlx::query("SELECT set_config('app.current_org_id', $1, true)")
            .bind(org_id.to_string())
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}
