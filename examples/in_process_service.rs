//! In-process service example

use code_agent::{
    service::{CodeAgentService, CodeAgentClient, ApiClientBuilder},
    config::AgentConfig,
    ServiceConfig,
};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🏭 AI Agent In-Process Service Example");
    println!("=====================================");

    // Load configuration
    let service_config = ServiceConfig::default();
    let agent_config = AgentConfig::load_with_fallback("config.toml")?;

    // Create service instance
    println!("🔧 Creating Code Agent Service...");
    let service = Arc::new(CodeAgentService::new(service_config, agent_config).await?);
    println!("✅ Service created successfully");

    // Create in-process client
    let client = CodeAgentClient::new(ApiClientBuilder::in_process(service.clone()));

    // Example 1: Basic usage
    println!("\n📝 Example 1: Basic task execution");
    println!("---------------------------------");

    match client.execute_simple_task("Write a hello world program in Rust").await {
        Ok(response) => {
            println!("✅ Task completed");
            println!("📋 Summary: {}", response.result.unwrap_or_default().summary);
        }
        Err(e) => {
            println!("❌ Task failed: {}", e);
        }
    }

    // Example 2: Multiple tasks
    println!("\n🔄 Example 2: Multiple concurrent tasks");
    println!("-------------------------------------");

    let tasks = vec![
        "List all .rs files in the src directory",
        "Read the Cargo.toml file and show dependencies",
        "Create a simple test file",
    ];

    let futures = tasks.into_iter()
        .map(|task| client.execute_simple_task(task))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(futures).await;

    println!("📊 Results:");
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(response) => {
                println!("  ✅ Task {}: {}", i + 1, response.result.as_ref().unwrap().summary);
            }
            Err(e) => {
                println!("  ❌ Task {}: {}", i + 1, e);
            }
        }
    }

    // Example 3: Service monitoring
    println!("\n📊 Example 3: Service monitoring");
    println!("-------------------------------");

    // Get service status
    let status = client.get_service_status().await?;
    println!("🏥 Service Health: {:?}", status.status);
    println!("⏱️  Uptime: {}s", status.uptime_seconds);
    println!("📈 Active tasks: {}", status.active_tasks);

    // Get metrics
    let metrics = client.get_metrics().await?;
    println!("📊 Total tasks processed: {}", metrics.total_tasks);
    println!("✅ Success rate: {:.1}%",
        if metrics.total_tasks > 0 {
            (metrics.completed_tasks as f64 / metrics.total_tasks as f64) * 100.0
        } else {
            0.0
        }
    );

    println!("\n🎉 In-process service example completed!");
    println!("======================================");

    Ok(())
}