//! Milestone 4 — agent completion over **Server-Sent Events** (SSE).

use std::convert::Infallible;

use axum::extract::{Path, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::api::middleware::auth::AuthenticatedUser;
use crate::api::middleware::observability::TraceID;
use crate::domain::engine::{ChatRole, LlmCompletionPort, LlmCompletionRequest, LlmMessage};
use crate::infrastructure::database::DatabasePool;
use crate::infrastructure::llm::openai_compatible_client_for_agent;

/// Request body for `POST /org/agents/:agent_id/complete/stream`.
#[derive(Debug, Deserialize)]
pub struct AgentStreamCompleteRequest {
    /// User message for this turn.
    pub message: String,
    /// Model id passed to the OpenAI-compatible provider (e.g. `gpt-4o-mini`).
    pub model: String,
    /// Optional cap; defaults to 1024.
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

/// SSE contract (Milestone 4, Spec Kit T4.7):
///
/// - **`Content-Type`**: `text/event-stream`
/// - **`event: meta`** — first frame. JSON: `{ "trace_id": "<uuid>" }` (mirrors `X-Trace-ID`).
/// - **`event: delta`** — model text. JSON: `{ "content": "<assistant text>" }`.
/// - **`event: usage`** — token counts. JSON: `{ "input_tokens": n, "output_tokens": m }`.
/// - **`event: error`** — non-fatal stream error (validation, provider, etc.). JSON: `{ "message": "..." }`.
/// - **`event: done`** — terminal frame (empty JSON object `{}`). Clients should stop reading after this.
///
/// All frames use standard SSE `data:` lines; payloads are JSON strings.
pub async fn post_agent_complete_stream(
    State(db): State<DatabasePool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Extension(trace): Extension<TraceID>,
    Path(agent_id): Path<Uuid>,
    Json(payload): Json<AgentStreamCompleteRequest>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>> + Send> {
    let trace_id = trace.0;
    let org_id = claims.org_id;
    let user_id = claims.sub;
    let max_tokens = payload.max_tokens.unwrap_or(1024).min(32_768);
    let message = payload.message;
    let model = payload.model;

    let stream = async_stream::stream! {
        let meta = json!({ "trace_id": trace_id.to_string() });
        yield Ok(Event::default().event("meta").data(meta.to_string()));

        tracing::info!(
            trace_id = %trace_id,
            %org_id,
            %agent_id,
            %user_id,
            "sse agent completion stream opened"
        );

        let client = match openai_compatible_client_for_agent(&db, org_id, agent_id).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(trace_id = %trace_id, error = %e, "failed to resolve llm client");
                let err = json!({ "message": e.to_string() }).to_string();
                yield Ok(Event::default().event("error").data(err));
                yield Ok(Event::default().event("done").data("{}"));
                return;
            }
        };

        let req = LlmCompletionRequest {
            messages: vec![LlmMessage {
                role: ChatRole::User,
                content: message,
            }],
            model,
            max_tokens,
        };

        tracing::info!(trace_id = %trace_id, %agent_id, "llm completion request started");

        match client.complete(req).await {
            Ok(completion) => {
                tracing::info!(
                    trace_id = %trace_id,
                    input_tokens = completion.input_tokens,
                    output_tokens = completion.output_tokens,
                    "llm completion finished"
                );
                let delta = json!({ "content": completion.content }).to_string();
                yield Ok(Event::default().event("delta").data(delta));
                let usage = json!({
                    "input_tokens": completion.input_tokens,
                    "output_tokens": completion.output_tokens,
                })
                .to_string();
                yield Ok(Event::default().event("usage").data(usage));
            }
            Err(e) => {
                tracing::warn!(trace_id = %trace_id, error = %e, "llm completion failed");
                let err = json!({ "message": e.to_string() }).to_string();
                yield Ok(Event::default().event("error").data(err));
            }
        }

        yield Ok(Event::default().event("done").data("{}"));
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}
