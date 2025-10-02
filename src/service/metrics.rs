//! Service metrics collection

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use metrics::{counter, gauge, histogram};
use crate::service_types::{SystemMetrics, NetworkMetrics, ServiceHealth};

/// Metrics collector for the AI Agent service
#[derive(Debug)]
pub struct MetricsCollector {
    /// Start time of the service
    start_time: Instant,
    /// Current metrics
    metrics: Arc<RwLock<ServiceMetricsData>>,
}

/// Internal metrics data structure
#[derive(Debug, Default)]
struct ServiceMetricsData {
    /// Total number of tasks processed
    total_tasks: u64,
    /// Number of completed tasks
    completed_tasks: u64,
    /// Number of failed tasks
    failed_tasks: u64,
    /// Number of active tasks
    active_tasks: u32,
    /// Task execution times (in seconds)
    task_execution_times: Vec<f64>,
    /// Tool usage counts
    tool_usage: HashMap<String, u64>,
    /// Error counts by type
    error_counts: HashMap<String, u64>,
    /// Current system metrics
    system_metrics: Option<SystemMetrics>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            metrics: Arc::new(RwLock::new(ServiceMetricsData::default())),
        }
    }

    /// Get service uptime in seconds
    pub fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Record a task start
    pub async fn record_task_start(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.total_tasks += 1;
        metrics.active_tasks += 1;
        // TODO: Fix metrics macros
        // counter!("ai_agent_tasks_total").increment(1);
        // gauge!("ai_agent_active_tasks").set(metrics.active_tasks as f64);
    }

    /// Record a task completion
    pub async fn record_task_completion(&self, execution_time_seconds: f64, success: bool) {
        let mut metrics = self.metrics.write().await;

        if metrics.active_tasks > 0 {
            metrics.active_tasks -= 1;
        }

        if success {
            metrics.completed_tasks += 1;
            counter!("ai_agent_tasks_completed_total").increment(1);
        } else {
            metrics.failed_tasks += 1;
            counter!("ai_agent_tasks_failed_total").increment(1);
        }

        metrics.task_execution_times.push(execution_time_seconds);

        // Keep only last 1000 execution times
        if metrics.task_execution_times.len() > 1000 {
            metrics.task_execution_times.remove(0);
        }

        histogram!("ai_agent_task_duration_seconds").record(execution_time_seconds);
        gauge!("ai_agent_active_tasks").set(metrics.active_tasks as f64);
    }

    /// Record tool usage
    pub async fn record_tool_usage(&self, tool_name: &str) {
        let mut metrics = self.metrics.write().await;
        *metrics.tool_usage.entry(tool_name.to_string()).or_insert(0) += 1;
        counter!("ai_agent_tool_usage_total", "tool" => tool_name).increment(1);
    }

    /// Record an error
    pub async fn record_error(&self, error_type: &str) {
        let mut metrics = self.metrics.write().await;
        *metrics.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
        counter!("ai_agent_errors_total", "error_type" => error_type).increment(1);
    }

    /// Update system metrics
    pub async fn update_system_metrics(&self, system_metrics: SystemMetrics) {
        let mut metrics = self.metrics.write().await;
        metrics.system_metrics = Some(system_metrics.clone());

        gauge!("ai_agent_cpu_usage_percent").set(system_metrics.cpu_usage_percent);
        gauge!("ai_agent_memory_usage_mb").set(system_metrics.memory_usage_mb);
        gauge!("ai_agent_disk_usage_mb").set(system_metrics.disk_usage_mb);
        gauge!("ai_agent_network_bytes_received").set(system_metrics.network_io.bytes_received as f64);
        gauge!("ai_agent_network_bytes_sent").set(system_metrics.network_io.bytes_sent as f64);
        gauge!("ai_agent_network_active_connections").set(system_metrics.network_io.active_connections as f64);
    }

    /// Get current metrics snapshot
    pub async fn get_metrics_snapshot(&self) -> MetricsSnapshot {
        let metrics = self.metrics.read().await;
        let avg_execution_time = if metrics.task_execution_times.is_empty() {
            0.0
        } else {
            metrics.task_execution_times.iter().sum::<f64>() / metrics.task_execution_times.len() as f64
        };

        MetricsSnapshot {
            uptime_seconds: self.uptime_seconds(),
            total_tasks: metrics.total_tasks,
            completed_tasks: metrics.completed_tasks,
            failed_tasks: metrics.failed_tasks,
            active_tasks: metrics.active_tasks,
            average_execution_time_seconds: avg_execution_time,
            tool_usage: metrics.tool_usage.clone(),
            error_counts: metrics.error_counts.clone(),
            system_metrics: metrics.system_metrics.clone(),
        }
    }

    /// Reset metrics
    pub async fn reset(&self) {
        let mut metrics = self.metrics.write().await;
        *metrics = ServiceMetricsData::default();
    }

    /// Get health status based on metrics
    pub async fn get_health_status(&self) -> ServiceHealth {
        let metrics = self.metrics.read().await;

        // Calculate failure rate
        let failure_rate = if metrics.total_tasks > 0 {
            metrics.failed_tasks as f64 / metrics.total_tasks as f64
        } else {
            0.0
        };

        // Check system metrics if available
        if let Some(system_metrics) = &metrics.system_metrics {
            // Check if system metrics are within acceptable ranges
            if system_metrics.cpu_usage_percent > 90.0
                || system_metrics.memory_usage_mb > 8000.0 // 8GB
                || failure_rate > 0.5 {
                return ServiceHealth::Unhealthy;
            } else if system_metrics.cpu_usage_percent > 70.0
                || system_metrics.memory_usage_mb > 4000.0 // 4GB
                || failure_rate > 0.1 {
                return ServiceHealth::Degraded;
            }
        } else {
            // If no system metrics available, use task-based metrics
            if failure_rate > 0.5 {
                return ServiceHealth::Unhealthy;
            } else if failure_rate > 0.1 {
                return ServiceHealth::Degraded;
            }
        }

        ServiceHealth::Healthy
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics snapshot for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    /// Service uptime in seconds
    pub uptime_seconds: u64,
    /// Total number of tasks processed
    pub total_tasks: u64,
    /// Number of completed tasks
    pub completed_tasks: u64,
    /// Number of failed tasks
    pub failed_tasks: u64,
    /// Number of currently active tasks
    pub active_tasks: u32,
    /// Average task execution time in seconds
    pub average_execution_time_seconds: f64,
    /// Tool usage statistics
    pub tool_usage: HashMap<String, u64>,
    /// Error counts by type
    pub error_counts: HashMap<String, u64>,
    /// Current system metrics
    pub system_metrics: Option<SystemMetrics>,
}

/// System metrics collector
pub struct SystemMetricsCollector;

impl SystemMetricsCollector {
    /// Collect current system metrics
    pub async fn collect() -> Result<SystemMetrics, Box<dyn std::error::Error + Send + Sync>> {
        // This is a simplified implementation
        // In a real implementation, you would use system-specific libraries
        // like `sysinfo` or `psutil` to collect actual metrics

        let cpu_usage = Self::get_cpu_usage().await?;
        let memory_usage = Self::get_memory_usage().await?;
        let disk_usage = Self::get_disk_usage().await?;
        let network_io = Self::get_network_io().await?;

        Ok(SystemMetrics {
            cpu_usage_percent: cpu_usage,
            memory_usage_mb: memory_usage,
            disk_usage_mb: disk_usage,
            network_io,
        })
    }

    /// Get CPU usage percentage
    async fn get_cpu_usage() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        // Simplified CPU usage calculation
        // In a real implementation, you would read from /proc/stat on Linux
        // or use platform-specific APIs
        Ok(25.0) // Placeholder value
    }

    /// Get memory usage in MB
    async fn get_memory_usage() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        // Simplified memory usage calculation
        // In a real implementation, you would read from /proc/meminfo on Linux
        // or use platform-specific APIs
        Ok(512.0) // Placeholder value: 512MB
    }

    /// Get disk usage in MB
    async fn get_disk_usage() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        // Simplified disk usage calculation
        // In a real implementation, you would use system-specific APIs
        Ok(1024.0) // Placeholder value: 1GB
    }

    /// Get network I/O metrics
    async fn get_network_io() -> Result<NetworkMetrics, Box<dyn std::error::Error + Send + Sync>> {
        // Simplified network I/O calculation
        // In a real implementation, you would read from /proc/net/dev on Linux
        // or use platform-specific APIs
        Ok(NetworkMetrics {
            bytes_received: 1024 * 1024, // 1MB
            bytes_sent: 512 * 1024,       // 512KB
            active_connections: 5,
        })
    }
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub collection_interval_seconds: u64,
    /// Enable Prometheus metrics export
    pub enable_prometheus: bool,
    /// Prometheus metrics endpoint
    pub prometheus_endpoint: String,
    /// Retention period for metrics data in seconds
    pub retention_period_seconds: u64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval_seconds: 30,
            enable_prometheus: true,
            prometheus_endpoint: "/metrics".to_string(),
            retention_period_seconds: 3600, // 1 hour
        }
    }
}