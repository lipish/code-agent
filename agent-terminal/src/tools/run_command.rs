use anyhow::Result;
use serde_json::{json, Value};
use tokio::process::Command;
use std::time::Duration;

use crate::context::Approval;
use crate::context::Sandbox;
use super::{Tool, ToolOutcome};

pub struct RunCommand;

#[async_trait::async_trait]
impl Tool for RunCommand {
    fn name(&self) -> &'static str { "run_command" }
    fn schema(&self) -> Value {
        json!({
            "description": "Run a shell command in the working directory (approval required).",
            "type": "object",
            "required": ["cmd"],
            "properties": {
                "cmd": {"type": "string", "description": "Command line, executed via shell"},
                "timeout_secs": {"type": "integer", "default": 60}
            }
        })
    }

    async fn run(&self, sandbox: &Sandbox, approval: &Approval, args: &Value) -> Result<ToolOutcome> {
        let cmd = args.get("cmd").and_then(|v| v.as_str()).unwrap_or("");
        let timeout = args.get("timeout_secs").and_then(|v| v.as_u64()).unwrap_or(60);
        if !approval.confirm(&format!("run command: {}", cmd))? {
            return Ok(ToolOutcome::Text("Denied by user".into()))
        }
        let mut child = Command::new("bash");
        child.arg("-lc").arg(cmd).current_dir(sandbox.root());
        let child = child.spawn()?;
        let wait = tokio::time::timeout(Duration::from_secs(timeout), child.wait_with_output());
        let out = wait.await??;
        Ok(ToolOutcome::Json(json!({
            "status": out.status.code(),
            "stdout": String::from_utf8_lossy(&out.stdout).trim(),
            "stderr": String::from_utf8_lossy(&out.stderr).trim()
        })))
    }
}

