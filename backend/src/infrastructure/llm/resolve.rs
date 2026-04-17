//! Resolve org-scoped LLM credentials from the database for an agent.

use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::infrastructure::database::agents::AgentsRepository;
use crate::infrastructure::database::DatabasePool;

use super::OpenAiCompatibleClient;

/// Builds an [`OpenAiCompatibleClient`] for the agent’s configured provider (RLS-scoped).
///
/// Never logs the API key. Keys are read from `api_key_encrypted` as stored (plaintext placeholder
/// until a real KMS is wired).
pub async fn openai_compatible_client_for_agent(
    db: &DatabasePool,
    org_id: Uuid,
    agent_id: Uuid,
) -> AppResult<OpenAiCompatibleClient> {
    let mut tx = db.get_pool().begin().await?;

    let agent = AgentsRepository::get_agent_by_id(&mut tx, org_id, agent_id)
        .await?
        .ok_or_else(|| AppError::NotFound("agent not found".into()))?;

    let provider_id = agent.llm_provider_id.ok_or_else(|| {
        AppError::Validation(
            "Agent has no LLM provider; assign llm_provider_id before calling the model".into(),
        )
    })?;

    let (provider, api_key_opt) =
        AgentsRepository::get_llm_provider_with_key(&mut tx, org_id, provider_id)
            .await?
            .ok_or_else(|| AppError::NotFound("LLM provider not found".into()))?;

    tx.commit().await?;

    let api_key = api_key_opt.ok_or_else(|| {
        AppError::Validation(
            "LLM provider has no API key configured; register the key when creating the provider"
                .into(),
        )
    })?;

    OpenAiCompatibleClient::new(provider.base_url, api_key)
        .map_err(|e| AppError::Validation(format!("LLM client configuration: {e}")))
}
