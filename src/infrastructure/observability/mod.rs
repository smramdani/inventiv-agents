use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::infrastructure::database::DatabasePool;

pub struct TelemetryRepository;

impl TelemetryRepository {
    pub async fn log_trace(
        pool: &PgPool,
        org_id: Uuid,
        trace_id: Uuid,
        level: &str,
        source: &str,
        message: &str,
        context: serde_json::Value,
    ) -> anyhow::Result<()> {
        // We use a separate pool or raw query here to avoid RLS 
        // issues during the logging of the logging system itself.
        sqlx::query(
            "INSERT INTO telemetry_logs (organization_id, trace_id, level, source, message, context) 
             VALUES ($1, $2, $3::log_level, $4::log_source, $5, $6)"
        )
        .bind(org_id)
        .bind(trace_id)
        .bind(level)
        .bind(source)
        .bind(message)
        .bind(context)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}
