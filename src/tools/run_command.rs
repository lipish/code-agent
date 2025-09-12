use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

use crate::context::{Approval, Sandbox};
use super::{Tool, ToolOutcome};

pub struct RunCommand;

#[async_trait::async_trait]
impl Tool for RunCommand {
    fn name(&self) -> &'static str { "run_command" }
    fn schema(&self) -> Value {
        json!({
            "description": "Run a shell command (bash -lc). Requires confirmation.",
            "type": "object",
            "required": ["cmd"],
            "properties": {
                "cmd": {"type": "string"},
                "timeout_secs": {"type": "integer", "default": 60}
            }
        })
    }

    async fn run(&self, sandbox: &Sandbox, approval: &Approval, args: &Value) -> Result<ToolOutcome> {
        let cmd = args.get("cmd").and_then(|v| v.as_str()).unwrap_or("");
        let t = args.get("timeout_secs").and_then(|v| v.as_u64()).unwrap_or(60);
        if !approval.confirm(&format!("run shell: {} ({}s timeout)", cmd, t))? {
            return Ok(ToolOutcome::Text("Denied by user".into()));
        }
        let mut c = Command::new("bash");
        c.arg("-lc").arg(cmd);
        c.current_dir(sandbox.root());
        c.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        let fut = c.output();
        let out = timeout(Duration::from_secs(t), fut).await.context("timeout")??;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            bail!("command failed ({}): {}", out.status, stderr);
        }
        Ok(ToolOutcome::Json(json!({"stdout": stdout, "stderr": stderr})))
    }
}

