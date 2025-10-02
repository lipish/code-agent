//! AI-Native Code Agent
//!
//! A minimal, AI-native code assistant that maximizes AI autonomy
//! while providing reliable execution capabilities.

pub mod agent;
pub mod config;
pub mod models;
pub mod tools;
pub mod cli;
pub mod types;
pub mod errors;
pub mod understanding;
pub mod execution;

pub use agent::CodeAgent;
pub use config::AgentConfig;
pub use models::LanguageModel;
pub use tools::Tool;
pub use types::*;
pub use errors::AgentError;