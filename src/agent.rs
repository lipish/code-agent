use crate::context::{Approval, Sandbox};
use crate::llm::plan_with_llm;
use crate::tools::{registry::ToolRegistry, ToolInvocation, ToolOutcome};
use anyhow::{anyhow, Result};
use serde_json::json;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub model: Option<String>,
    pub yes: bool,
    pub working_dir: PathBuf,
}

pub struct Agent {
    config: Config,
    sandbox: Sandbox,
    approval: Approval,
    registry: ToolRegistry,
}

impl Agent {
    pub fn new(config: Config) -> Result<Self> {
        let sandbox = Sandbox::new(config.working_dir.clone());
        let approval = Approval {
            auto_yes: config.yes,
        };
        let registry = ToolRegistry::default();
        Ok(Self {
            config,
            sandbox,
            approval,
            registry,
        })
    }

    fn tool_schema(&self) -> String {
        // Compact human+LLM oriented schema (sufficient for response_format=json_object)
        let tools = self.registry.schemas();
        serde_json::to_string_pretty(&json!({
            "tools": tools,
            "format": {
                "type": "object",
                "required": ["tool", "args"],
                "properties": {
                    "tool": {"type": "string", "enum": self.registry.tool_names()},
                    "args": {"type": "object"}
                }
            }
        }))
        .unwrap()
    }
    pub async fn plan(&mut self, prompt: &str) -> Result<ToolInvocation> {
        let model = self.config.model.as_ref().ok_or_else(|| anyhow!("No model configured. Please specify a model using --model or by setting the LLM_MODEL environment variable."))?;
        let plan = plan_with_llm(model, prompt, &self.tool_schema()).await?;
        Ok(ToolInvocation {
            tool: plan.tool,
            args: plan.args,
        })
    }

    pub async fn exec(&mut self, prompt: &str) -> Result<()> {
        let invocation = self.plan(prompt).await?;
        let outcome = self
            .registry
            .run(&self.sandbox, &self.approval, &invocation)
            .await?;
        Self::print_outcome(&outcome);
        Ok(())
    }

    pub async fn repl(&mut self) -> Result<()> {
        println!("Agent REPL. Type a goal, or /quit to exit.");
        loop {
            print!("goal> ");
            io::stdout().flush().ok();
            let mut goal = String::new();
            if io::stdin().read_line(&mut goal)? == 0 {
                break;
            }
            let goal = goal.trim();
            if goal.is_empty() {
                continue;
            }
            if goal == "/quit" {
                break;
            }

            match self.plan(goal).await {
                Ok(invocation) => {
                    let outcome = self
                        .registry
                        .run(&self.sandbox, &self.approval, &invocation)
                        .await;
                    match outcome {
                        Ok(out) => Self::print_outcome(&out),
                        Err(e) => eprintln!("error: {e}"),
                    }
                }
                Err(e) => eprintln!("plan error: {e}"),
            }
        }
        Ok(())
    }

    fn print_outcome(outcome: &ToolOutcome) {
        match outcome {
            ToolOutcome::Text(s) => println!("{}", s),
            ToolOutcome::Json(v) => {
                println!("{}", serde_json::to_string_pretty(v).unwrap_or_default())
            }
        }
    }
}
