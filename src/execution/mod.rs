//! Task Execution Module
//!
//! This module provides specialized execution capabilities for different types of operations.

pub mod file_ops;
pub mod command_ops;

// Re-export commonly used items
pub use file_ops::{read_file, write_file, list_files};
pub use command_ops::run_command;

