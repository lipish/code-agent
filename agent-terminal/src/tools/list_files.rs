use anyhow::Result;
use serde_json::{json, Value};

use crate::context::Sandbox;
use super::{Tool, ToolOutcome};

pub struct ListFiles;

#[async_trait::async_trait]
impl Tool for ListFiles {
    fn name(&self) -> &'static str { "list_files" }
    fn schema(&self) -> Value {
        json!({
            "description": "List files under working directory respecting .gitignore/.rooignore.",
            "type": "object",
            "properties": {
                "max": {"type": "integer", "minimum": 1, "default": 2000},
                "glob": {"type": "string", "description": "Optional glob like **/*.rs"}
            }
        })
    }

    async fn run(&self, sandbox: &Sandbox, _approval: &crate::context::Approval, args: &Value) -> Result<ToolOutcome> {
        let max = args.get("max").and_then(|v| v.as_u64()).unwrap_or(2000) as usize;
        let glob = args.get("glob").and_then(|v| v.as_str());

        let mut hits = vec![];
        let mut builder = sandbox.build_walker();
        if let Some(pat) = glob { builder.add_custom_ignore_filename(""); /* no-op: ignore can't filter by glob here */ }
        let root = sandbox.root().to_path_buf();
        for dent in builder.build().flatten() {
            let p = dent.path();
            if p.is_file() {
                let rel = p.strip_prefix(&root).unwrap_or(p);
                let s = rel.to_string_lossy().to_string();
                if let Some(pat) = glob {
                    if !globset::Glob::new(pat).ok().and_then(|g| globset::GlobSetBuilder::new().add(g).build().ok()).map(|gs| gs.is_match(&s)).unwrap_or(true) {
                        continue;
                    }
                }
                hits.push(s);
                if hits.len() >= max { break; }
            }
        }
        Ok(ToolOutcome::Json(json!({"files": hits})))
    }
}

