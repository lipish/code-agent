//! Service configuration and status types
//!
//! This module contains types for service configuration, health, and monitoring.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Service status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Service name
    pub name: String,
    /// Service version
    pub version: String,
    /// Service health status
    pub health: ServiceHealth,
    /// Service uptime in seconds
    pub uptime_seconds: u64,
    /// Number of active tasks
    pub active_tasks: usize,
    /// Total tasks processed
    pub total_tasks_processed: u64,
    /// System metrics
    pub system_metrics: SystemMetrics,
    /// Network metrics
    pub network_metrics: NetworkMetrics,
    /// Last update timestamp
    pub timestamp: DateTime<Utc>,

    // Legacy fields for backward compatibility
    #[serde(alias = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_tasks: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_tasks: Option<u64>,
    #[serde(default)]
    pub available_tools: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// System resource metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Disk usage in bytes
    pub disk_usage: u64,
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMetrics {
    /// Bytes received
    pub bytes_received: u64,
    /// Bytes sent
    pub bytes_sent: u64,
    /// Number of requests
    pub requests: u64,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: u32,
    /// Request timeout in seconds
    pub request_timeout_seconds: u64,
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS configuration
    pub cors: Option<CorsConfig>,
    /// Rate limiting configuration
    pub rate_limit: Option<RateLimitConfig>,

    // Legacy fields for backward compatibility
    #[serde(alias = "rate_limiting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limiting: Option<RateLimitConfig>,
    #[serde(alias = "default_task_timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_timeout: Option<u64>,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: u32,
    /// Time window in seconds
    pub window_seconds: u64,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        let rate_limit_config = Some(RateLimitConfig {
            max_requests: 100,
            window_seconds: 60,
        });

        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_concurrent_tasks: 10,
            request_timeout_seconds: 300,
            enable_cors: true,
            cors: Some(CorsConfig {
                allowed_origins: vec!["*".to_string()],
                allowed_methods: vec!["GET".to_string(), "POST".to_string()],
                allowed_headers: vec!["Content-Type".to_string()],
            }),
            rate_limit: rate_limit_config.clone(),
            // Legacy fields
            rate_limiting: rate_limit_config,
            default_task_timeout: Some(300),
        }
    }
}

