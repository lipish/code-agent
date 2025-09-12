use anyhow::Result;
use regex::Regex;
use serde_json::{json, Value};
use std::fs;

use crate::context::Sandbox;
use super::{Tool, ToolOutcome};

pub struct SearchFiles;

#[async_trait::async_trait]
impl Tool for SearchFiles {
    fn name(&self) -> &'static str { "search_files" }
    fn schema(&self) -> Value {
        json!({
            "description": "Regex search across text files in the repo respecting ignores.",
            "type": "object",
            "required": ["pattern"],
            "properties": {
                "pattern": {"type": "string"},
                "max_results": {"type": "integer", "default": 200}
            }
        })
    }

    async fn run(&self, sandbox: &Sandbox, _approval: &crate::context::Approval, args: &Value) -> Result<ToolOutcome> {
        let pat = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
        let max = args.get("max_results").and_then(|v| v.as_u64()).unwrap_or(200) as usize;
        let re = Regex::new(pat)?;

        let mut hits = vec![];
        let root = sandbox.root().to_path_buf();
        for dent in sandbox.build_walker().build().flatten() {
            let p = dent.path();
            if p.is_file() {
                // Small binary heuristic: skip if not utf8
                if let Ok(bytes) = fs::read(p) {
                    if let Ok(text) = std::str::from_utf8(&bytes) {
                        if re.is_match(text) {
                            let rel = p.strip_prefix(&root).unwrap_or(p);
                            hits.push(rel.to_string_lossy().to_string());
                            if hits.len() >= max { break; }
                        }
                    }
                }
            }
        }
        Ok(ToolOutcome::Json(json!({"pattern": pat, "hits": hits})))
    }
}

