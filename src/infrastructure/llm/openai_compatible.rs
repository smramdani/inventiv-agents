//! OpenAI-compatible `POST /v1/chat/completions` client (OVH, OpenRouter, Azure-style hosts).

use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::domain::engine::{
    ChatRole, LlmCompletion, LlmCompletionError, LlmCompletionPort, LlmCompletionRequest,
    LlmMessage,
};

const CHAT_COMPLETIONS_PATH: &str = "/v1/chat/completions";

/// HTTP client for providers exposing an OpenAI-style chat completions API.
#[derive(Debug, Clone)]
pub struct OpenAiCompatibleClient {
    http: reqwest::Client,
    base_url: String,
    api_key: Secret<String>,
}

impl OpenAiCompatibleClient {
    /// Builds a client with a default 120s timeout and TLS from `reqwest`.
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self, LlmCompletionError> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| LlmCompletionError::Provider(format!("http client: {e}")))?;
        Self::with_http_client(http, base_url, api_key)
    }

    /// For tests or custom timeouts / proxies.
    pub fn with_http_client(
        http: reqwest::Client,
        base_url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self, LlmCompletionError> {
        let base_url = base_url.into().trim_end_matches('/').to_string();
        if base_url.is_empty() {
            return Err(LlmCompletionError::InvalidRequest(
                "base_url must not be empty".into(),
            ));
        }
        Ok(Self {
            http,
            base_url,
            api_key: Secret::new(api_key.into()),
        })
    }

    fn completions_url(&self) -> String {
        format!("{}{}", self.base_url, CHAT_COMPLETIONS_PATH)
    }
}

#[derive(Serialize)]
struct OaiChatRequest<'a> {
    model: &'a str,
    messages: Vec<OaiMessage<'a>>,
    max_tokens: u32,
}

#[derive(Serialize)]
struct OaiMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OaiChatResponse {
    choices: Vec<OaiChoice>,
    usage: Option<OaiUsage>,
}

#[derive(Deserialize)]
struct OaiChoice {
    message: OaiMsgBody,
}

#[derive(Deserialize)]
struct OaiMsgBody {
    content: Option<String>,
}

#[derive(Deserialize)]
struct OaiUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
}

#[derive(Deserialize)]
struct OaiErrorBody {
    error: Option<OaiErrorMsg>,
}

#[derive(Deserialize)]
struct OaiErrorMsg {
    message: Option<String>,
}

fn role_str(r: &ChatRole) -> &'static str {
    match r {
        ChatRole::System => "system",
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
        ChatRole::Tool => "tool",
    }
}

fn map_messages(messages: &[LlmMessage]) -> Vec<OaiMessage<'_>> {
    messages
        .iter()
        .map(|m| OaiMessage {
            role: role_str(&m.role),
            content: m.content.as_str(),
        })
        .collect()
}

#[async_trait]
impl LlmCompletionPort for OpenAiCompatibleClient {
    async fn complete(
        &self,
        req: LlmCompletionRequest,
    ) -> Result<LlmCompletion, LlmCompletionError> {
        if req.model.trim().is_empty() {
            return Err(LlmCompletionError::EmptyModel);
        }
        if req.messages.is_empty() {
            return Err(LlmCompletionError::InvalidRequest(
                "messages must not be empty".into(),
            ));
        }

        let body = OaiChatRequest {
            model: req.model.as_str(),
            messages: map_messages(&req.messages),
            max_tokens: req.max_tokens,
        };

        let url = self.completions_url();
        let response = self
            .http
            .post(&url)
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.api_key.expose_secret()),
            )
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmCompletionError::Provider(format!("request failed: {e}")))?;

        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|e| LlmCompletionError::Provider(format!("read body: {e}")))?;

        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(LlmCompletionError::RateLimited);
        }

        if !status.is_success() {
            let msg = serde_json::from_slice::<OaiErrorBody>(&bytes)
                .ok()
                .and_then(|e| e.error)
                .and_then(|e| e.message)
                .unwrap_or_else(|| format!("HTTP {status}"));
            if status.is_client_error() {
                return Err(LlmCompletionError::InvalidRequest(msg));
            }
            return Err(LlmCompletionError::Provider(msg));
        }

        let parsed: OaiChatResponse = serde_json::from_slice(&bytes)
            .map_err(|e| LlmCompletionError::Provider(format!("invalid completion JSON: {e}")))?;

        let choice = parsed
            .choices
            .first()
            .ok_or_else(|| LlmCompletionError::Provider("no choices in response".into()))?;

        let content = choice.message.content.clone().unwrap_or_default();

        let input_tokens = parsed
            .usage
            .as_ref()
            .and_then(|u| u.prompt_tokens)
            .unwrap_or(0);
        let output_tokens = parsed
            .usage
            .as_ref()
            .and_then(|u| u.completion_tokens)
            .unwrap_or(0);

        Ok(LlmCompletion {
            content,
            input_tokens,
            output_tokens,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::engine::{ChatRole, LlmMessage};
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn completes_parses_usage_and_content() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .and(header("authorization", "Bearer sk-test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "choices": [{ "message": { "role": "assistant", "content": "hello" } }],
                "usage": { "prompt_tokens": 3, "completion_tokens": 5 }
            })))
            .mount(&server)
            .await;

        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let client =
            OpenAiCompatibleClient::with_http_client(http, server.uri(), "sk-test").unwrap();

        let out = client
            .complete(LlmCompletionRequest {
                messages: vec![LlmMessage {
                    role: ChatRole::User,
                    content: "hi".into(),
                }],
                model: "test-model".into(),
                max_tokens: 32,
            })
            .await
            .unwrap();

        assert_eq!(out.content, "hello");
        assert_eq!(out.input_tokens, 3);
        assert_eq!(out.output_tokens, 5);
    }

    #[tokio::test]
    async fn maps_429_to_rate_limited() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(429).set_body_string("slow down"))
            .mount(&server)
            .await;

        let http = reqwest::Client::new();
        let client = OpenAiCompatibleClient::with_http_client(http, server.uri(), "k").unwrap();
        let err = client
            .complete(LlmCompletionRequest {
                messages: vec![LlmMessage {
                    role: ChatRole::User,
                    content: "x".into(),
                }],
                model: "m".into(),
                max_tokens: 1,
            })
            .await
            .unwrap_err();
        assert!(matches!(err, LlmCompletionError::RateLimited));
    }
}
