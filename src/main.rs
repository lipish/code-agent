//! Main entry point for the AI-Native Code Agent

use code_agent::cli::Cli;
use clap::Parser;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    info!("Starting AI-Native Code Agent");

    match cli.run().await {
        Ok(_) => {
            info!("Task completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Task failed: {}", e);
            Err(e)
        }
    }
}