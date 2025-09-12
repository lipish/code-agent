use super::{list_files::ListFiles, read_file::ReadFile, run_command::RunCommand, search_files::SearchFiles, write_file::WriteFile, Tool, ToolInvocation, ToolOutcome};
use crate::context::{Approval, Sandbox};
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct ToolRegistry {
    tools: HashMap<&'static str, Box<dyn Tool>>, 
}

impl Default for ToolRegistry {
    fn default() -> Self {
        let mut tools: HashMap<&'static str, Box<dyn Tool>> = HashMap::new();
        let add = |m: &mut HashMap<_, _>, t: Box<dyn Tool>| { m.insert(t.name(), t); };
        add(&mut tools, Box::new(ListFiles));
        add(&mut tools, Box::new(ReadFile));
        add(&mut tools, Box::new(SearchFiles));
        add(&mut tools, Box::new(RunCommand));
        add(&mut tools, Box::new(WriteFile));
        Self { tools }
    }
}

impl ToolRegistry {
    pub fn tool_names(&self) -> Vec<&'static str> { self.tools.keys().copied().collect() }

    pub fn schemas(&self) -> Value {
        let mut v: Vec<Value> = Vec::new();
        for (name, t) in &self.tools {
            v.push(json!({
                "name": name,
                "args": t.schema(),
            }));
        }
        Value::Array(v)
    }

    pub async fn run(&self, sandbox: &Sandbox, approval: &Approval, inv: &ToolInvocation) -> Result<ToolOutcome> {
        let tool = self.tools.get(inv.tool.as_str()).ok_or_else(|| anyhow!("unknown tool: {}", inv.tool))?;
        tool.run(sandbox, approval, &inv.args).await
    }
}

