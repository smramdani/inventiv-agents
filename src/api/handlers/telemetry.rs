use axum::{extract::State, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::api::middleware::auth::AuthenticatedUser;
use crate::api::middleware::observability::TraceID;
use crate::infrastructure::database::DatabasePool;
use crate::infrastructure::observability::TelemetryRepository;
use crate::error::AppResult;

#[derive(Deserialize)]
pub struct FrontendTelemetryRequest {
    pub level: String, // DEBUG, INFO, WARN, ERROR
    pub message: String,
    pub context: serde_json::Value,
}

pub async fn handle_frontend_telemetry(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    // We can also extract the TraceID from the middleware if needed
    Json(payloads): Json<Vec<FrontendTelemetryRequest>>,
) -> AppResult<Json<String>> {
    // 1. Trace the incoming telemetry batch
    tracing::debug!("Received {} telemetry items from frontend for org {}", payloads.len(), claims.org_id);

    // 2. Persist each item
    for item in payloads {
        TelemetryRepository::log_trace(
            db.get_pool(),
            claims.org_id,
            Uuid::new_v4(), // FE generates its own or we assign one here
            &item.level,
            "Frontend",
            &item.message,
            item.context,
        ).await.map_err(|e| {
            tracing::error!("Failed to persist FE telemetry: {}", e);
            crate::error::AppError::Internal
        })?;
    }

    Ok(Json("Stored".to_string()))
}
