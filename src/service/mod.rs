//! AI Agent Service Implementation
//!
//! This module provides a complete service layer for the AI Agent,
//! including API types, core service logic, error handling, and metrics.
//!
//! # Module Organization
//!
//! - `types` - Well-organized API types (task, batch, service, websocket)
//! - `api` - HTTP API endpoints
//! - `core` - Core service implementation
//! - `error` - Error types and handling
//! - `metrics` - Metrics collection and reporting

pub mod types;
pub mod api;
pub mod core;
pub mod error;
pub mod metrics_simple;
pub use metrics_simple as metrics;

pub use api::*;
pub use core::*;
pub use error::*;
pub use metrics::*;