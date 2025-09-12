use anyhow::{bail, Result};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::context::{read_text, Sandbox};
use super::{parse_required, Tool, ToolOutcome};

pub struct ReadFile;

#[async_trait::async_trait]
impl Tool for ReadFile {
    fn name(&self) -> &'static str { "read_file" }
    fn schema(&self) -> Value {
        json!({
            "description": "Read a text file. Optionally with 1-based line range.",
            "type": "object",
            "required": ["path"],
            "properties": {
                "path": {"type": "string"},
                "start_line": {"type": "integer", "minimum": 1},
                "end_line": {"type": "integer", "minimum": 1}
            }
        })
    }

    async fn run(&self, sandbox: &Sandbox, _approval: &crate::context::Approval, args: &Value) -> Result<ToolOutcome> {
        let path = parse_required(args, "path")?.as_str().ok_or_else(|| anyhow::anyhow!("path must be string"))?;
        let start = args.get("start_line").and_then(|v| v.as_u64());
        let end = args.get("end_line").and_then(|v| v.as_u64());

        let mut full = PathBuf::from(path);
        if !full.is_absolute() { full = sandbox.root().join(full); }
        sandbox.assert_inside(&full)?;

        let text = read_text(&full)?;
        let content = if let (Some(s), Some(e)) = (start, end) {
            if e < s { bail!("end_line < start_line"); }
            let s1 = (s as usize).saturating_sub(1);
            let e1 = e as usize;
            text.lines().skip(s1).take(e1 - s1).collect::<Vec<_>>().join("\n")
        } else { text };

        Ok(ToolOutcome::Json(json!({
            "path": path,
            "content": content
        })))
    }
}

