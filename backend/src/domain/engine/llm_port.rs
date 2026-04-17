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

/// Token accounting for a single completion (persisted in later milestones).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

impl TokenUsage {
    #[must_use]
    pub const fn total(self) -> u64 {
        self.input_tokens.saturating_add(self.output_tokens)
    }
}

impl From<&LlmCompletion> for TokenUsage {
    fn from(c: &LlmCompletion) -> Self {
        Self {
            input_tokens: c.input_tokens,
            output_tokens: c.output_tokens,
        }
    }
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

    #[test]
    fn token_usage_from_completion() {
        let c = LlmCompletion {
            content: "x".into(),
            input_tokens: 10,
            output_tokens: 4,
        };
        let u = TokenUsage::from(&c);
        assert_eq!(u.total(), 14);
    }
}
