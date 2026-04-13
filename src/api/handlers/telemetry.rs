use axum::extract::{Extension, State};
use axum::Json;
use serde::Deserialize;

use crate::api::middleware::auth::AuthenticatedUser;
use crate::api::middleware::observability::TraceID;
use crate::error::AppResult;
use crate::infrastructure::database::DatabasePool;
use crate::infrastructure::observability::TelemetryRepository;

#[derive(Deserialize)]
pub struct FrontendTelemetryRequest {
    pub level: String, // DEBUG, INFO, WARN, ERROR
    pub message: String,
    pub context: serde_json::Value,
}

pub async fn handle_frontend_telemetry(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Extension(trace): Extension<TraceID>,
    Json(payloads): Json<Vec<FrontendTelemetryRequest>>,
) -> AppResult<Json<String>> {
    // 1. Trace the incoming telemetry batch
    tracing::debug!(
        "Received {} telemetry items from frontend for org {}",
        payloads.len(),
        claims.org_id
    );

    // 2. Persist each item
    for item in payloads {
        TelemetryRepository::log_trace(
            db.get_pool(),
            claims.org_id,
            trace.0,
            &item.level,
            "Frontend",
            &item.message,
            item.context,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to persist FE telemetry: {}", e);
            crate::error::AppError::Internal
        })?;
    }

    Ok(Json("Stored".to_string()))
}
