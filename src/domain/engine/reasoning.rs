//! Canonical reasoning loop phases (Constitution XIII).
//!
//! State transitions are explicit so orchestration code cannot skip mandatory
//! steps without going through a documented branch (e.g. no-tool shortcut).

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Ordered phases for a single user turn when tools may be involved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPhase {
    /// Model reasoning over the user message and context.
    Reasoning,
    /// Choose zero or one tool calls to execute (M4 may restrict to one).
    ToolSelection,
    /// Run MCP / native tool.
    ToolExecution,
    /// Validate tool output before feeding back to the model.
    Validation,
    /// Produce the final model response (may stream at the API layer).
    Response,
    /// Terminal: this turn is finished successfully.
    Completed,
}

/// Inputs that influence legal transitions (e.g. empty toolbelt).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionInput {
    /// When false after `Reasoning`, the loop jumps directly to `Response`
    /// (no tool selection / execution / validation for this turn).
    pub tool_execution_required: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EngineError {
    #[error("cannot advance: phase is already terminal (completed)")]
    TerminalPhase,
    #[error("invalid transition: {message}")]
    InvalidTransition { message: &'static str },
}

impl ReasoningPhase {
    /// Starting phase for a new user message.
    pub const fn initial() -> Self {
        Self::Reasoning
    }

    /// Returns true when no further `step` calls are valid.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Advance the loop by one logical step.
    ///
    /// - From `Reasoning`: either `ToolSelection` (tools needed) or `Response` (direct answer).
    /// - From `ToolSelection`: must go to `ToolExecution` when `tool_execution_required` is true.
    /// - After `ToolExecution` → `Validation` → `Response` → `Completed`.
    pub fn step(self, input: &TransitionInput) -> Result<Self, EngineError> {
        match self {
            Self::Reasoning => {
                if input.tool_execution_required {
                    Ok(Self::ToolSelection)
                } else {
                    Ok(Self::Response)
                }
            }
            Self::ToolSelection => {
                if input.tool_execution_required {
                    Ok(Self::ToolExecution)
                } else {
                    Err(EngineError::InvalidTransition {
                        message:
                            "tool_execution_required must be true after reaching ToolSelection",
                    })
                }
            }
            Self::ToolExecution => Ok(Self::Validation),
            Self::Validation => Ok(Self::Response),
            Self::Response => Ok(Self::Completed),
            Self::Completed => Err(EngineError::TerminalPhase),
        }
    }

    /// Run `step` repeatedly until `Completed` or an error.
    pub fn run_to_completion(mut self, input: &TransitionInput) -> Result<Self, EngineError> {
        while !self.is_terminal() {
            self = self.step(input)?;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_tools_path_reasoning_to_response_to_completed() {
        let input = TransitionInput {
            tool_execution_required: false,
        };
        let p0 = ReasoningPhase::initial();
        let p1 = p0.step(&input).unwrap();
        assert_eq!(p1, ReasoningPhase::Response);
        let p2 = p1.step(&input).unwrap();
        assert_eq!(p2, ReasoningPhase::Completed);
        assert!(p2.is_terminal());
    }

    #[test]
    fn with_tools_full_chain() {
        let input = TransitionInput {
            tool_execution_required: true,
        };
        let end = ReasoningPhase::initial()
            .step(&input)
            .unwrap()
            .step(&input)
            .unwrap()
            .step(&input)
            .unwrap()
            .step(&input)
            .unwrap()
            .step(&input)
            .unwrap();
        assert_eq!(end, ReasoningPhase::Completed);
    }

    #[test]
    fn completed_cannot_advance() {
        let input = TransitionInput {
            tool_execution_required: false,
        };
        let err = ReasoningPhase::Completed.step(&input).unwrap_err();
        assert_eq!(err, EngineError::TerminalPhase);
    }

    #[test]
    fn tool_selection_with_tools_flag_false_is_invalid() {
        let input = TransitionInput {
            tool_execution_required: false,
        };
        let err = ReasoningPhase::ToolSelection.step(&input).unwrap_err();
        assert!(matches!(err, EngineError::InvalidTransition { .. }));
    }

    #[test]
    fn run_to_completion_no_tools() {
        let input = TransitionInput {
            tool_execution_required: false,
        };
        let end = ReasoningPhase::initial().run_to_completion(&input).unwrap();
        assert_eq!(end, ReasoningPhase::Completed);
    }

    #[test]
    fn run_to_completion_with_tools() {
        let input = TransitionInput {
            tool_execution_required: true,
        };
        let end = ReasoningPhase::initial().run_to_completion(&input).unwrap();
        assert_eq!(end, ReasoningPhase::Completed);
    }
}
