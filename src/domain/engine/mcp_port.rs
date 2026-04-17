//! Port for MCP tool discovery and invocation (infrastructure implements JSON-RPC transport).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpToolDefinition {
    pub name: String,
    pub description: Option<String>,
    /// JSON Schema for arguments, when the server provides one.
    pub input_schema: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpInvokeRequest {
    pub tool_name: String,
    pub arguments: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpInvokeResult {
    /// Normalized structured result from the tool (transport-specific wrapping stripped by adapter).
    pub content: Value,
}

#[derive(Debug, Error)]
pub enum McpInvocationError {
    #[error("transport: {0}")]
    Transport(String),
    #[error("tool error: {0}")]
    Tool(String),
    #[error("timeout")]
    Timeout,
    #[error("unknown tool: {0}")]
    UnknownTool(String),
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),
}

/// MCP access for a single configured skill endpoint (implemented in `infrastructure`).
#[async_trait]
pub trait McpInvocationPort: Send + Sync {
    async fn list_tools(&self) -> Result<Vec<McpToolDefinition>, McpInvocationError>;

    async fn invoke(&self, req: McpInvokeRequest) -> Result<McpInvokeResult, McpInvocationError>;
}

/// Validates an invoke request before transport (T4.12).
pub fn validate_mcp_invoke_request(req: &McpInvokeRequest) -> Result<(), McpInvocationError> {
    let name = req.tool_name.trim();
    if name.is_empty() {
        return Err(McpInvocationError::InvalidArguments(
            "tool_name must not be empty".into(),
        ));
    }
    Ok(())
}

/// Trivial tool-selection slice (M4b / T4.11): if the server exposes exactly one tool, use it.
/// Otherwise returns [`None`] so orchestration must disambiguate.
pub fn select_unique_tool_name(tools: &[McpToolDefinition]) -> Option<String> {
    if tools.len() == 1 {
        Some(tools[0].name.clone())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PanicMcp;

    #[async_trait]
    impl McpInvocationPort for PanicMcp {
        async fn list_tools(&self) -> Result<Vec<McpToolDefinition>, McpInvocationError> {
            Ok(vec![])
        }

        async fn invoke(
            &self,
            _req: McpInvokeRequest,
        ) -> Result<McpInvokeResult, McpInvocationError> {
            Err(McpInvocationError::UnknownTool("none".into()))
        }
    }

    #[test]
    fn validate_rejects_empty_tool_name() {
        let req = McpInvokeRequest {
            tool_name: "   ".into(),
            arguments: serde_json::json!({}),
        };
        assert!(matches!(
            validate_mcp_invoke_request(&req),
            Err(McpInvocationError::InvalidArguments(_))
        ));
    }

    #[test]
    fn select_unique_tool_name_only_when_singleton() {
        assert_eq!(select_unique_tool_name(&[]), None::<String>);
        let one = vec![McpToolDefinition {
            name: "only".into(),
            description: None,
            input_schema: None,
        }];
        assert_eq!(select_unique_tool_name(&one), Some("only".into()));
        let two = vec![
            McpToolDefinition {
                name: "a".into(),
                description: None,
                input_schema: None,
            },
            McpToolDefinition {
                name: "b".into(),
                description: None,
                input_schema: None,
            },
        ];
        assert_eq!(select_unique_tool_name(&two), None);
    }

    #[tokio::test]
    async fn port_invoke_roundtrip_shape() {
        let port: &dyn McpInvocationPort = &PanicMcp;
        let tools = port.list_tools().await.unwrap();
        assert!(tools.is_empty());
        let err = port
            .invoke(McpInvokeRequest {
                tool_name: "x".into(),
                arguments: Value::Null,
            })
            .await
            .unwrap_err();
        assert!(matches!(err, McpInvocationError::UnknownTool(_)));
    }
}
