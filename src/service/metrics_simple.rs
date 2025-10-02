//! Service metrics collection - Simplified version

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use crate::service_types::{SystemMetrics, NetworkMetrics, ServiceHealth};

/// Metrics collector for the AI Agent service
#[derive(Debug)]
pub struct MetricsCollector {
    start_time: Instant,
    metrics: Arc<RwLock<ServiceMetrics>>,
}

/// Internal service metrics
#[derive(Debug, Clone, Default)]
pub struct ServiceMetrics {
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub active_tasks: u64,
    pub total_execution_time: f64,
    pub task_execution_times: Vec<f64>,
    pub tool_usage: HashMap<String, u64>,
    pub error_counts: HashMap<String, u64>,
    pub system_metrics: SystemMetrics,
    pub custom_metrics: HashMap<String, f64>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            metrics: Arc::new(RwLock::new(ServiceMetrics::default())),
        }
    }

    /// Get current metrics snapshot
    pub async fn get_metrics_snapshot(&self) -> MetricsSnapshot {
        let metrics = self.metrics.read().await;
        MetricsSnapshot {
            uptime_seconds: self.start_time.elapsed().as_secs(),
            total_tasks: metrics.total_tasks,
            completed_tasks: metrics.completed_tasks,
            failed_tasks: metrics.failed_tasks,
            active_tasks: metrics.active_tasks,
            average_execution_time_seconds: if metrics.completed_tasks > 0 {
                metrics.total_execution_time / metrics.completed_tasks as f64
            } else {
                0.0
            },
            tool_usage: metrics.tool_usage.clone(),
            error_counts: metrics.error_counts.clone(),
            system_metrics: metrics.system_metrics.clone(),
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
    }

    /// Record a task completion
    pub async fn record_task_completion(&self, execution_time_seconds: f64, success: bool) {
        let mut metrics = self.metrics.write().await;

        if metrics.active_tasks > 0 {
            metrics.active_tasks -= 1;
        }

        if success {
            metrics.completed_tasks += 1;
        } else {
            metrics.failed_tasks += 1;
        }

        metrics.task_execution_times.push(execution_time_seconds);

        // Keep only last 1000 execution times
        if metrics.task_execution_times.len() > 1000 {
            metrics.task_execution_times.remove(0);
        }
    }

    /// Record tool usage
    pub async fn record_tool_usage(&self, tool_name: &str) {
        let mut metrics = self.metrics.write().await;
        *metrics.tool_usage.entry(tool_name.to_string()).or_insert(0) += 1;
    }

    /// Record an error
    pub async fn record_error(&self, error_type: &str) {
        let mut metrics = self.metrics.write().await;
        *metrics.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
    }

    /// Update system metrics
    pub async fn update_system_metrics(&self, system_metrics: SystemMetrics) {
        let mut metrics = self.metrics.write().await;
        metrics.system_metrics = system_metrics;
    }

    /// Set a custom metric
    pub async fn set_custom_metric(&self, name: &str, value: f64) {
        let mut metrics = self.metrics.write().await;
        metrics.custom_metrics.insert(name.to_string(), value);
    }

    /// Get a custom metric
    pub async fn get_custom_metric(&self, name: &str) -> Option<f64> {
        let metrics = self.metrics.read().await;
        metrics.custom_metrics.get(name).copied()
    }

    /// Reset all metrics
    pub async fn reset(&self) {
        let mut metrics = self.metrics.write().await;
        *metrics = ServiceMetrics::default();
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of current metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub uptime_seconds: u64,
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub active_tasks: u64,
    pub average_execution_time_seconds: f64,
    pub tool_usage: HashMap<String, u64>,
    pub error_counts: HashMap<String, u64>,
    pub system_metrics: SystemMetrics,
}