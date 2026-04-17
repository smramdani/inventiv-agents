//! Agentic engine boundaries: reasoning loop phases and outbound ports (Milestone 4).

mod llm_port;
mod mcp_port;
mod reasoning;

pub use llm_port::{
    ChatRole, LlmCompletion, LlmCompletionError, LlmCompletionPort, LlmCompletionRequest,
    LlmMessage, TokenUsage,
};
pub use mcp_port::{
    select_unique_tool_name, validate_mcp_invoke_request, McpInvocationError, McpInvocationPort,
    McpInvokeRequest, McpInvokeResult, McpToolDefinition,
};
pub use reasoning::{EngineError, ReasoningPhase, TransitionInput};
