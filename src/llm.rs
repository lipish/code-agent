use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPlan {
    pub tool: String,
    pub args: serde_json::Value,
}

/// Stub implementation. We'll wire LLM later; for now, always error so caller can fallback to manual plan entry.
pub async fn plan_with_llm(_model: &str, _prompt: &str, _tool_schema: &str) -> Result<ToolPlan> {
    bail!("LLM is not configured in this build")
}

