//! JSON-RPC 2.0 MCP client over HTTP POST (`tools/list`, `tools/call`).
//!
//! Transport: single URL (e.g. skill `endpoint_url`); timeouts and body size limits are strict.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::domain::engine::{
    validate_mcp_invoke_request, McpInvocationError, McpInvocationPort, McpInvokeRequest,
    McpInvokeResult, McpToolDefinition,
};

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const MAX_RESPONSE_BYTES: usize = 2 * 1024 * 1024;

#[derive(Debug, Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcErrorObj {
    message: String,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    result: Option<Value>,
    error: Option<JsonRpcErrorObj>,
}

/// MCP client for one HTTP endpoint (typically an MCP skill `endpoint_url`).
#[derive(Debug)]
pub struct McpHttpJsonRpcClient {
    http: reqwest::Client,
    endpoint: String,
    next_id: AtomicU64,
}

impl McpHttpJsonRpcClient {
    /// Builds a client with default timeout and TLS from `reqwest`.
    pub fn new(endpoint: impl Into<String>) -> Result<Self, McpInvocationError> {
        let endpoint = endpoint.into();
        if endpoint.trim().is_empty() {
            return Err(McpInvocationError::InvalidArguments(
                "MCP endpoint URL must not be empty".into(),
            ));
        }
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .map_err(|e| McpInvocationError::Transport(format!("http client: {e}")))?;
        Ok(Self {
            http,
            endpoint: endpoint.trim_end_matches('/').to_string(),
            next_id: AtomicU64::new(1),
        })
    }

    fn next_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    async fn post_rpc(
        &self,
        method: &str,
        params: Option<Value>,
    ) -> Result<Value, McpInvocationError> {
        let id = self.next_id();
        let body = JsonRpcRequest {
            jsonrpc: "2.0",
            id,
            method,
            params,
        };

        let response = self
            .http
            .post(&self.endpoint)
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    McpInvocationError::Timeout
                } else {
                    McpInvocationError::Transport(format!("request failed: {e}"))
                }
            })?;

        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|e| McpInvocationError::Transport(format!("read body: {e}")))?;

        if bytes.len() > MAX_RESPONSE_BYTES {
            return Err(McpInvocationError::Transport(format!(
                "MCP response too large: {} bytes (max {MAX_RESPONSE_BYTES})",
                bytes.len()
            )));
        }

        if status == StatusCode::REQUEST_TIMEOUT || status == StatusCode::GATEWAY_TIMEOUT {
            return Err(McpInvocationError::Timeout);
        }

        if !status.is_success() {
            let msg = String::from_utf8_lossy(&bytes).into_owned();
            return Err(McpInvocationError::Transport(format!(
                "HTTP {status}: {msg}"
            )));
        }

        let parsed: JsonRpcResponse = serde_json::from_slice(&bytes).map_err(|e| {
            McpInvocationError::Transport(format!("invalid JSON-RPC response: {e}"))
        })?;

        if let Some(err) = parsed.error {
            return Err(McpInvocationError::Tool(err.message));
        }

        parsed
            .result
            .ok_or_else(|| McpInvocationError::Transport("JSON-RPC result missing".into()))
    }
}

#[async_trait]
impl McpInvocationPort for McpHttpJsonRpcClient {
    async fn list_tools(&self) -> Result<Vec<McpToolDefinition>, McpInvocationError> {
        let result = self.post_rpc("tools/list", Some(json!({}))).await?;
        let tools_val = result.get("tools").ok_or_else(|| {
            McpInvocationError::Transport("tools/list: missing tools array".into())
        })?;
        let arr = tools_val.as_array().ok_or_else(|| {
            McpInvocationError::Transport("tools/list: tools must be array".into())
        })?;

        let mut out = Vec::with_capacity(arr.len());
        for t in arr {
            let name = t
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| McpInvocationError::Transport("tool entry missing name".into()))?
                .to_string();
            let description = t
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let input_schema = t.get("inputSchema").cloned();
            out.push(McpToolDefinition {
                name,
                description,
                input_schema,
            });
        }
        Ok(out)
    }

    async fn invoke(&self, req: McpInvokeRequest) -> Result<McpInvokeResult, McpInvocationError> {
        validate_mcp_invoke_request(&req)?;
        let params = json!({
            "name": req.tool_name,
            "arguments": req.arguments,
        });
        let result = self.post_rpc("tools/call", Some(params)).await?;

        if result.get("isError").and_then(|v| v.as_bool()) == Some(true) {
            let msg = result
                .pointer("/content/0/text")
                .and_then(|v| v.as_str())
                .unwrap_or("tool returned isError=true")
                .to_string();
            return Err(McpInvocationError::Tool(msg));
        }

        Ok(McpInvokeResult { content: result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_string_contains, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn list_tools_parses_mcp_result() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_string_contains("tools/list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "tools": [
                        {
                            "name": "echo",
                            "description": "Echo",
                            "inputSchema": { "type": "object" }
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let client = McpHttpJsonRpcClient::new(format!("{}/", server.uri())).unwrap();
        let tools = client.list_tools().await.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "echo");
        assert_eq!(tools[0].description.as_deref(), Some("Echo"));
    }

    #[tokio::test]
    async fn invoke_returns_result_content() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_string_contains("tools/call"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": 2,
                "result": {
                    "content": [{ "type": "text", "text": "ok" }],
                    "isError": false
                }
            })))
            .mount(&server)
            .await;

        let client = McpHttpJsonRpcClient::new(server.uri()).unwrap();
        let out = client
            .invoke(McpInvokeRequest {
                tool_name: "echo".into(),
                arguments: json!({"x": 1}),
            })
            .await
            .unwrap();
        assert_eq!(out.content["content"][0]["text"], "ok");
    }

    #[tokio::test]
    async fn jsonrpc_error_maps_to_tool() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": { "code": -32601, "message": "unknown method" }
            })))
            .mount(&server)
            .await;

        let client = McpHttpJsonRpcClient::new(server.uri()).unwrap();
        let err = client.list_tools().await.unwrap_err();
        assert!(matches!(err, McpInvocationError::Tool(ref m) if m == "unknown method"));
    }
}
