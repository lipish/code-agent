//! Command Line Interface for the AI-Native Code Agent

use clap::{Parser, Subcommand};
use std::io::{self, Write};
use crate::config::AgentConfig;
use crate::models::LanguageModel;

#[derive(Parser)]
#[command(name = "ai-agent")]
#[command(about = "AI-Native Code Assistant")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Process a single task
    Task {
        /// The task description
        task: String,
        /// Configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
        /// Output format (text, json, verbose)
        #[arg(short, long, default_value = "text")]
        output: String,
    },
    /// Start interactive mode
    Interactive {
        /// Configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
    /// List available tools
    Tools {
        /// Configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
    /// Show configuration
    Config {
        /// Configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
}

impl Cli {
    /// Run the CLI command
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Task { task, config, output } => {
                Self::handle_task(task, config, output).await
            }
            Commands::Interactive { config } => {
                Self::handle_interactive(config).await
            }
            Commands::Tools { config } => {
                Self::handle_tools(config).await
            }
            Commands::Config { config } => {
                Self::handle_config(config).await
            }
        }
    }

    async fn handle_task(task: String, config_path: String, output: String) -> anyhow::Result<()> {
        println!("🚀 Starting AI Agent Task Execution");
        println!("====================================");
        println!("📝 Task: {}", task);
        println!("⏰ Started at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        println!();

        tracing::info!("Processing task: {}", task);

        println!("🔧 Loading configuration...");
        let config = AgentConfig::load_with_fallback(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
        println!("✅ Configuration loaded successfully");

        println!("🤖 Initializing AI agent...");
        let mut agent = create_agent(&config).await?;
        let tool_count = agent.tool_count().await;
        println!("✅ Agent initialized with {} tools", tool_count);

        println!("🧠 Processing task with AI model...");
        println!("📋 Creating task plan...");
        let start_time = std::time::Instant::now();
        let result = agent.process_task(&task).await;
        let duration = start_time.elapsed();

        println!("🏁 Task execution completed in {:.2}s", duration.as_secs_f32());
        println!("====================================");

        match result {
            Ok(task_result) => {
                println!("✅ Task Status: SUCCESS");
                println!("⏱️  Execution Time: {}s", task_result.execution_time.unwrap_or(0));

                match output.as_str() {
                    "json" => {
                        println!("📄 Output (JSON format):");
                        println!("{}", serde_json::to_string_pretty(&task_result)?);
                    }
                    "verbose" => {
                        println!("📋 Task Plan:");
                        if let Some(plan) = &task_result.task_plan {
                            println!("  🧠 Understanding: {}", plan.understanding);
                            println!("  🛠️  Approach: {}", plan.approach);
                            println!("  📊 Complexity: {:?}", plan.complexity);
                            println!("  🔢 Estimated Steps: {}", plan.estimated_steps.unwrap_or(0));
                            if !plan.requirements.is_empty() {
                                println!("  📋 Requirements: {}", plan.requirements.join(", "));
                            }
                        }
                        println!();
                        println!("📋 Summary:");
                        println!("  {}", task_result.summary);
                        if let Some(details) = task_result.details {
                            println!();
                            println!("🔍 Detailed Results:");
                            println!("  {}", details.replace('\n', "\n  "));
                        }
                        if task_result.execution_time.is_some() {
                            println!();
                            println!("📊 Performance Metrics:");
                            println!("  • Internal execution time: {}s", task_result.execution_time.unwrap_or(0));
                            println!("  • Total wall-clock time: {:.2}s", duration.as_secs_f32());
                        }
                    }
                    _ => {
                        println!("📋 Result:");
                        println!("{}", task_result.summary);
                        if let Some(details) = task_result.details {
                            println!();
                            println!("🔍 Details:");
                            println!("{}", details);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Task Status: FAILED");
                println!("🚨 Error Details:");
                println!("  {}", e);
            }
        }

        println!("====================================");
        println!("🏁 Task execution finished at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        println!();

        Ok(())
    }

    async fn handle_interactive(config_path: String) -> anyhow::Result<()> {
        let config = AgentConfig::load_with_fallback(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
        let mut agent = create_agent(&config).await?;

        println!("AI-Native Code Agent - Interactive Mode");
        println!("Type 'exit' or 'quit' to exit");
        println!("Type 'help' for available commands");
        println!();

        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            match input {
                "exit" | "quit" => {
                    println!("Goodbye!");
                    break;
                }
                "help" => {
                    Self::print_help();
                }
                "tools" => {
                    Self::print_available_tools(&agent).await;
                }
                _ => {
                    let result = agent.process_task(input).await;
                    match result {
                        Ok(task_result) => {
                            println!("✅ {}", task_result.summary);
                            if let Some(details) = task_result.details {
                                println!("\nDetails:\n{}", details);
                            }
                        }
                        Err(e) => {
                            println!("❌ Error: {}", e);
                        }
                    }
                    println!();
                }
            }
        }

        Ok(())
    }

    async fn handle_tools(config_path: String) -> anyhow::Result<()> {
        let config = AgentConfig::load_with_fallback(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
        let agent = create_agent(&config).await?;

        Self::print_available_tools(&agent).await;
        Ok(())
    }

    async fn handle_config(config_path: String) -> anyhow::Result<()> {
        let config = AgentConfig::load_with_fallback(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
        println!("Configuration:");
        println!("{:#?}", config);
        Ok(())
    }

    fn print_help() {
        println!("Available commands:");
        println!("  exit, quit  - Exit the program");
        println!("  help        - Show this help message");
        println!("  tools       - List available tools");
        println!("  config      - Show current configuration");
        println!("  Any other text will be processed as a task");
        println!();
    }

    async fn print_available_tools(agent: &crate::agent::TaskAgent) {
        let tools = agent.get_tools();
        let tool_names = tools.get_tool_names().await;

        println!("Available tools:");
        for tool_name in tool_names {
            println!("  • {}", tool_name);
        }
        println!();
    }
}

/// Create an agent with the given configuration
async fn create_agent(config: &AgentConfig) -> anyhow::Result<crate::agent::TaskAgent> {
    // Create model based on provider
    let model: Box<dyn LanguageModel> = match &config.model.provider {
        crate::config::ModelProvider::OpenAI => {
            let api_key = config.model.api_key.clone()
                .ok_or_else(|| anyhow::anyhow!("OpenAI API key not found"))?;
            Box::new(crate::models::OpenAIModel::new(api_key, config.model.model_name.clone()))
        }
        crate::config::ModelProvider::Anthropic => {
            let api_key = config.model.api_key.clone()
                .ok_or_else(|| anyhow::anyhow!("Anthropic API key not found"))?;
            Box::new(crate::models::AnthropicModel::new(api_key, config.model.model_name.clone()))
        }
        crate::config::ModelProvider::Zhipu => {
            let api_key = config.model.api_key.clone()
                .ok_or_else(|| anyhow::anyhow!("Zhipu API key not found"))?;
            Box::new(crate::models::ZhipuModel::new(api_key, config.model.model_name.clone(), config.model.endpoint.clone()))
        }
        crate::config::ModelProvider::Local(ref endpoint) => {
            Box::new(crate::models::LocalModel::new(endpoint.clone(), config.model.model_name.clone()))
        }
    };

    let agent = crate::agent::TaskAgent::new(model, config.clone());

    // Register basic tools
    agent.register_tool(crate::tools::ReadFileTool).await;
    agent.register_tool(crate::tools::WriteFileTool).await;
    agent.register_tool(crate::tools::RunCommandTool).await;
    agent.register_tool(crate::tools::ListFilesTool).await;

    Ok(agent)
}