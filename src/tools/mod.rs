use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::context::{Approval, Sandbox};

pub mod registry;
pub mod read_file;
pub mod list_files;
pub mod search_files;
pub mod run_command;
pub mod write_file;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolInvocation {
    pub tool: String,
    pub args: Value,
}

#[derive(Debug, Clone)]
pub enum ToolOutcome {
    Text(String),
    Json(Value),
}

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn schema(&self) -> Value; // JSON schema snippet for args and a short description
    async fn run(&self, sandbox: &Sandbox, approval: &Approval, args: &Value) -> Result<ToolOutcome>;
}

pub fn parse_required<'a>(args: &'a Value, key: &str) -> Result<&'a Value> {
    args.get(key).ok_or_else(|| anyhow!("missing arg: {}", key))
}

