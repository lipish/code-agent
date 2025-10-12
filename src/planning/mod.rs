//! Task Planning Module
//!
//! This module provides AI-powered task analysis and execution planning capabilities.
//!
//! The planning engine analyzes task requirements and creates detailed execution plans,
//! including step-by-step approaches, complexity estimation, and required tools.
//!
//! # Examples
//!
//! ```no_run
//! use task_runner::planning::{PlanningEngine, PlanningConfig};
//! use task_runner::models::MockModel;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let model = Arc::new(MockModel::new("test".to_string()));
//! let config = PlanningConfig {
//!     verbose: true,
//!     max_retries: 3,
//!     auto_infer_type: true,
//! };
//! let engine = PlanningEngine::with_config(model, config);
//! let plan = engine.analyze_task("Create a configuration loader").await?;
//! # Ok(())
//! # }
//! ```

mod engine;
mod approach_parser;

pub use engine::{PlanningEngine, PlanningConfig};
pub use approach_parser::ApproachParser;

// Backward compatibility aliases (deprecated)
#[deprecated(since = "0.2.3", note = "Use `PlanningEngine` instead")]
pub use engine::PlanningEngine as UnderstandingEngine;

#[deprecated(since = "0.2.3", note = "Use `PlanningConfig` instead")]
pub use engine::PlanningConfig as UnderstandingConfig;

