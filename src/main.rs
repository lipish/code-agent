mod cli;
mod llm;
mod agent;
mod context;
mod tools;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let cli = Cli::parse();
    let working_dir = cli
        .working_dir
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("cwd"));

    let mut agent = agent::Agent::new(agent::Config {
        model: cli.model.clone(),
        yes: cli.yes,
        working_dir,
    })?;

    match cli.command {
        Commands::Run => agent.repl().await?,
        Commands::Exec { prompt } => agent.exec(&prompt).await?,
        Commands::Plan { prompt } => {
            let plan = agent.plan(&prompt).await?;
            println!("{}", serde_json::to_string_pretty(&plan)?);
        }
    }

    Ok(())
}

