//! Task Execution Module
//!
//! This module provides specialized execution capabilities for different types of operations.

pub mod file_ops;
pub mod command_ops;
pub mod sequential;
pub mod guardrails;

// Re-export commonly used items
pub use file_ops::{read_file, write_file, list_files};
pub use command_ops::run_command;

// Re-export sequential execution types
pub use sequential::{
    SequentialExecutor,
    SequentialExecutionPlan,
    ExecutionConfig,
    ExecutionPhase,
    PhaseResult,
    PhaseStatus,
    ValidationResult,
    UnderstandingOutput,
    ApproachOutput,
    DetailedPlan,
    ExecutionStep,
    StepType,
};

// Re-export guardrail types
pub use guardrails::{
    GuardrailEngine,
    GuardrailConfig,
    OperationGuard,
    OperationRiskLevel,
    OperationType,
    OperationTarget,
    OperationImpact,
    DangerousPattern,
    DangerousPatternDetector,
    ConfirmationRequest,
    ConfirmationResponse,
    ConfirmationOption,
    RollbackPlan,
    RollbackStep,
    RollbackAction,
    DryRunResult,
};

