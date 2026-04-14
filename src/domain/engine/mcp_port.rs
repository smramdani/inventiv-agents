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
