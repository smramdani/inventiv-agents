//! Port for LLM completions (infrastructure implements HTTP to OpenAI-compatible APIs).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Role of a message in a chat-style completion request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmCompletionRequest {
    pub messages: Vec<LlmMessage>,
    pub model: String,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmCompletion {
    pub content: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Error)]
pub enum LlmCompletionError {
    #[error("provider error: {0}")]
    Provider(String),
    #[error("rate limited")]
    RateLimited,
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("empty model")]
    EmptyModel,
}

/// Outbound LLM access (implemented in `infrastructure`, not in domain).
#[async_trait]
pub trait LlmCompletionPort: Send + Sync {
    /// Non-streaming completion; streaming will be added at the HTTP/infrastructure layer (M4).
    async fn complete(
        &self,
        req: LlmCompletionRequest,
    ) -> Result<LlmCompletion, LlmCompletionError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stub used only to prove port object-safety and call shape in tests.
    struct PanicLlm;

    #[async_trait]
    impl LlmCompletionPort for PanicLlm {
        async fn complete(
            &self,
            _req: LlmCompletionRequest,
        ) -> Result<LlmCompletion, LlmCompletionError> {
            Err(LlmCompletionError::Provider("not implemented".into()))
        }
    }

    #[tokio::test]
    async fn port_can_be_called_as_dyn() {
        let port: &dyn LlmCompletionPort = &PanicLlm;
        let req = LlmCompletionRequest {
            messages: vec![LlmMessage {
                role: ChatRole::User,
                content: "hi".into(),
            }],
            model: "gpt-test".into(),
            max_tokens: 16,
        };
        let err = port.complete(req).await.unwrap_err();
        assert!(matches!(err, LlmCompletionError::Provider(_)));
    }
}
