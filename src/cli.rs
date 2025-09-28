use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser, Clone)]
#[command(
    name = "agent-terminal",
    version,
    about = "A Rust code agent terminal (Roo-Code inspired)"
)]
pub struct Cli {
    /// LLM model name (e.g. gpt-4o-mini, glm-4.5). If unset, runs in manual mode.
    #[arg(long)]
    pub model: Option<String>,

    /// Approve write/command actions without interactive confirmation
    #[arg(long, short = 'y', global = true)]
    pub yes: bool,

    /// Working directory sandbox (defaults to current directory)
    #[arg(long)]
    pub working_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Interactive REPL loop
    Run,
    /// Execute a one-shot task via the agent
    Exec { prompt: String },
    /// Ask the agent to propose a plan (prints the tool call it would do)
    Plan { prompt: String },
}
