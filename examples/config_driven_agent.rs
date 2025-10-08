//! Config-Driven Agent Demo
//!
//! This example demonstrates using a single generic Agent
//! configured via YAML templates and runtime tool registration.

use task_runner::models::MockModel;
use task_runner::planning::PlanningEngine;
use task_runner::prompts::PromptTemplate;
use task_runner::tools::{ListFilesTool, ReadFileTool, RunCommandTool, ToolCall, ToolArgs, ToolRegistry, WriteFileTool};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("=== Config-Driven Agent Demo ===\n");

    // 1) Load prompt template from YAML (fallback to default)
    let template = match PromptTemplate::from_file("prompts/optimized-template.yaml") {
        Ok(t) => t,
        Err(_) => {
            println!("Note: optimized-template.yaml not found, using default template");
            PromptTemplate::default()
        }
    };

    // 2) Create planning engine with the template
    // MockModel returns a pre-canned understanding/approach for demonstration
    let model = Arc::new(MockModel::new(
        "UNDERSTANDING: Implement a safe file reader\n\
         APPROACH: Use PathValidator and ToolRegistry\n\
         COMPLEXITY: Simple\n\
         REQUIREMENTS: serde, tokio"
            .to_string(),
    ));
    let engine = PlanningEngine::with_template(model, template);

    // 3) Analyze a task via the config-driven prompt system
    let plan = engine
        .analyze_task("读取 Cargo.toml 并打印前 200 个字符")
        .await?;

    println!("Understanding: {}", plan.understanding);
    println!("Approach: {}", plan.approach);
    println!("Complexity: {:?}", plan.complexity);
    println!("Requirements: {:?}\n", plan.requirements);

    // 4) Register tools dynamically (extensible via ToolRegistry)
    let registry = ToolRegistry::new();
    registry.register(ReadFileTool).await;
    registry.register(WriteFileTool).await;
    registry.register(ListFilesTool).await;
    registry.register(RunCommandTool).await;

    println!("Available tools: {}\n", registry.count());

    // 5) Execute a simple tool call (read_file) as a demo
    let mut args = HashMap::new();
    args.insert("path".to_string(), serde_json::json!("Cargo.toml"));
    let call = ToolCall {
        name: "read_file".to_string(),
        args: ToolArgs::from_map(args),
    };
    match registry.execute(&call).await {
        Ok(result) => {
            println!("Tool result summary:\n{}\n", result.summary);
            if let Some(output) = result.output {
                let preview = output.chars().take(200).collect::<String>();
                println!("File preview (first 200 chars):\n{}\n", preview);
            }
        }
        Err(e) => println!("Tool execution error: {}", e),
    }

    println!("\nDone.");
    Ok(())
}