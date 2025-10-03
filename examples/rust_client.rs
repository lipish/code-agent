//! Rust client example for AI Agent Service

use code_agent::service::{CodeAgentClient, ApiClientBuilder};
use code_agent::{TaskRequest, TaskPriority};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), code_agent::ServiceError> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 AI Agent Service Rust Client Example");
    println!("=====================================");

    // Create client from environment configuration
    let client = match ApiClientBuilder::from_env() {
        Ok(client) => {
            println!("✅ Connected to Code Agent Service");
            CodeAgentClient::new(client)
        }
        Err(e) => {
            println!("❌ Failed to connect to service: {}", e);
            println!("Make sure the service is running and CODE_AGENT_API_URL is set");
            return Err(e);
        }
    };

    // Example 1: Simple task execution
    println!("\n📝 Example 1: Simple task execution");
    println!("---------------------------------");

    match client.execute_simple_task("Hello! Please introduce yourself.").await {
        Ok(response) => {
            println!("✅ Task completed successfully");
            println!("📋 Summary: {}", response.result.unwrap_or_default().summary);
            println!("⏱️  Execution time: {}s", response.metrics.total_execution_time);
        }
        Err(e) => {
            println!("❌ Task failed: {}", e);
        }
    }

    // Example 2: Task with context
    println!("\n📂 Example 2: Task with custom context");
    println!("------------------------------------");

    let mut environment = HashMap::new();
    environment.insert("PATH".to_string(), "/usr/bin:/bin".to_string());

    match client.execute_task_with_context(
        "List files in the current directory and show the first 3 files",
        Some("/tmp"), // working directory
        Some(environment),
    ).await {
        Ok(response) => {
            println!("✅ Task completed successfully");
            if let Some(result) = response.result {
                println!("📋 Summary: {}", result.summary);
                if let Some(details) = result.details {
                    println!("📄 Details: {}", details);
                }
            }
        }
        Err(e) => {
            println!("❌ Task failed: {}", e);
        }
    }

    // Example 3: High priority task
    println!("\n🔥 Example 3: High priority task");
    println!("------------------------------");

    let task_request = TaskRequest {
        task: "Analyze the system resources (CPU, memory, disk usage)".to_string(),
        task_id: None,
        context: None,
        priority: Some(TaskPriority::High),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("requester".to_string(), "system_monitor".into());
            Some(meta)
        },
    };

    match client.api().execute_task(task_request).await {
        Ok(response) => {
            println!("✅ High priority task completed");
            println!("📊 Status: {:?}", response.status);
            println!("📋 Summary: {}", response.result.unwrap_or_default().summary);
        }
        Err(e) => {
            println!("❌ Task failed: {}", e);
        }
    }

    // Example 4: Get service status
    println!("\n🏥 Example 4: Service health check");
    println!("-------------------------------");

    match client.get_service_status().await {
        Ok(status) => {
            println!("✅ Service status: {:?}", status.status);
            println!("📈 Uptime: {}s", status.uptime_seconds);
            println!("📊 Active tasks: {}", status.active_tasks);
            println!("✅ Completed tasks: {}", status.completed_tasks);
            println!("❌ Failed tasks: {}", status.failed_tasks);
            println!("🛠️  Available tools: {}", status.available_tools.join(", "));
        }
        Err(e) => {
            println!("❌ Failed to get service status: {}", e);
        }
    }

    // Example 5: Get metrics
    println!("\n📊 Example 5: Service metrics");
    println!("-------------------------");

    match client.get_metrics().await {
        Ok(metrics) => {
            println!("✅ Metrics retrieved successfully");
            println!("⏱️  Uptime: {}s", metrics.uptime_seconds);
            println!("📈 Total tasks: {}", metrics.total_tasks);
            println!("✅ Completed tasks: {}", metrics.completed_tasks);
            println!("❌ Failed tasks: {}", metrics.failed_tasks);
            println!("⚡ Average execution time: {:.2}s", metrics.average_execution_time_seconds);

            if !metrics.tool_usage.is_empty() {
                println!("🛠️  Tool usage:");
                for (tool, count) in metrics.tool_usage {
                    println!("  • {}: {} uses", tool, count);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to get metrics: {}", e);
        }
    }

    // Example 6: Batch processing (if supported)
    println!("\n📦 Example 6: Batch processing");
    println!("----------------------------");

    use code_agent::{BatchTaskRequest, BatchExecutionMode};

    let batch_request = BatchTaskRequest {
        tasks: vec![
            TaskRequest {
                task: "What is the current date and time?".to_string(),
                task_id: None,
                context: None,
                priority: Some(TaskPriority::Normal),
                metadata: None,
            },
            TaskRequest {
                task: "Show system information".to_string(),
                task_id: None,
                context: None,
                priority: Some(TaskPriority::Normal),
                metadata: None,
            },
        ],
        mode: BatchExecutionMode::Parallel,
        continue_on_error: true,
    };

    match client.execute_batch(batch_request).await {
        Ok(batch_response) => {
            println!("✅ Batch completed successfully");
            println!("📊 Statistics:");
            println!("  • Total tasks: {}", batch_response.statistics.total_tasks);
            println!("  • Completed: {}", batch_response.statistics.completed_tasks);
            println!("  • Failed: {}", batch_response.statistics.failed_tasks);
            println!("  • Total time: {}s", batch_response.statistics.total_execution_time);
            println!("  • Average time: {:.2}s", batch_response.statistics.average_execution_time);
        }
        Err(e) => {
            println!("❌ Batch failed: {}", e);
        }
    }

    println!("\n🎉 All examples completed!");
    println!("========================");

    Ok(())
}