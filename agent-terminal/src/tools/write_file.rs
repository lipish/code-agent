use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

use crate::context::{write_text, Approval, Sandbox};
use super::{Tool, ToolOutcome};

pub struct WriteFile;

#[async_trait::async_trait]
impl Tool for WriteFile {
    fn name(&self) -> &'static str { "write_file" }
    fn schema(&self) -> Value {
        json!({
            "description": "Write entire file content (creates or overwrites). Makes a .bak backup if overwriting.",
            "type": "object",
            "required": ["path", "content"],
            "properties": {
                "path": {"type": "string"},
                "content": {"type": "string"}
            }
        })
    }

    async fn run(&self, sandbox: &Sandbox, approval: &Approval, args: &Value) -> Result<ToolOutcome> {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
        if !approval.confirm(&format!("write file: {} ({} bytes)", path, content.len()))? {
            return Ok(ToolOutcome::Text("Denied by user".into()));
        }
        let mut full = PathBuf::from(path);
        if !full.is_absolute() { full = sandbox.root().join(full); }
        sandbox.assert_inside(&full)?;
        if full.exists() {
            let bak = full.with_extension("bak");
            fs::copy(&full, &bak).ok();
        }
        write_text(&full, content)?;
        Ok(ToolOutcome::Json(json!({"path": path, "status": "ok"})))
    }
}

