# AI Agent Service API Documentation

## Overview

The AI Agent Service provides both Rust API and HTTP REST API interfaces for integrating AI-powered code assistance into your applications.

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Rust Client   │    │  HTTP Client    │    │  Other Clients  │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────┴─────────────┐
                    │    AI Agent Service     │
                    │  (Core Business Logic)  │
                    └─────────────┬─────────────┘
                                 │
          ┌──────────────────────┼──────────────────────┐
          │                      │                      │
    ┌─────┴─────┐        ┌──────┴───────┐        ┌──────┴─────┐
    │  Models   │        │   Tools     │        │  Metrics   │
    │ (Zhipu,   │        │ (File Ops,  │        │ (Prometheus│
    │ OpenAI,   │        │ Commands,  │        │  Export)   │
    │ etc.)     │        │ etc.)       │        │            │
    └───────────┘        └─────────────┘        └────────────┘
```

## 🚀 Quick Start

### Rust API Usage

```rust
use ai_agent::{
    service::{AiAgentService, ServiceConfig, AiAgentClient, ApiClientBuilder},
    config::AgentConfig
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service
    let service = Arc::new(AiAgentService::new(
        ServiceConfig::default(),
        AgentConfig::load_with_fallback("config.toml")?
    ).await?);

    // Create in-process client
    let client = AiAgentClient::new(ApiClientBuilder::in_process(service));

    // Execute task
    let response = client.execute_simple_task("Hello, world!").await?;
    println!("Result: {}", response.result.unwrap().summary);

    Ok(())
}
```

### HTTP API Usage

```bash
# Start the service
cargo run --bin ai-agent-server

# Execute a task via HTTP
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "task": "Read the README.md file and summarize it"
  }'

# Get service status
curl http://localhost:8080/api/v1/status
```

## 📋 API Reference

### Core Types

#### TaskRequest
```json
{
  "task": "string - Task description",
  "task_id": "string - Optional custom ID",
  "context": {
    "working_directory": "string - Working directory",
    "environment": {"key": "value"},
    "tools": ["tool1", "tool2"],
    "constraints": {
      "max_execution_time": 300,
      "max_steps": 10,
      "allowed_paths": ["/safe/path"],
      "forbidden_operations": ["rm -rf /"]
    }
  },
  "priority": "low|normal|high|critical",
  "metadata": {"custom": "data"}
}
```

#### TaskResponse
```json
{
  "task_id": "string",
  "status": "queued|running|completed|failed|cancelled|timeout",
  "result": {
    "success": true,
    "summary": "Task summary",
    "details": "Detailed results",
    "artifacts": [{
      "artifact_type": "file|text|json|image|log|report|other",
      "name": "artifact name",
      "content": "artifact content (small artifacts)",
      "size": 1024,
      "metadata": {}
    }],
    "execution_time": 30
  },
  "plan": {
    "understanding": "AI understanding of task",
    "approach": "AI approach to solve",
    "complexity": "simple|moderate|complex",
    "estimated_steps": 3,
    "requirements": ["tool1", "tool2"],
    "created_at": "2024-01-01T00:00:00Z"
  },
  "steps": [{
    "step_number": 1,
    "step_type": "analysis|planning|tool_use|execution|verification|completion",
    "description": "Step description",
    "input": {},
    "output": {},
    "status": "pending|running|completed|failed|skipped",
    "error": "Error message if failed",
    "execution_time_ms": 1500,
    "timestamp": "2024-01-01T00:00:00Z"
  }],
  "metrics": {
    "total_execution_time": 30,
    "planning_time_ms": 1500,
    "execution_time_ms": 28500,
    "steps_executed": 3,
    "tools_used": 2,
    "memory_usage_mb": 256.5,
    "cpu_usage_percent": 15.2,
    "custom_metrics": {}
  },
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error",
    "details": "Additional error details",
    "stack_trace": "Debug stack trace",
    "timestamp": "2024-01-01T00:00:00Z"
  },
  "created_at": "2024-01-01T00:00:00Z",
  "started_at": "2024-01-01T00:00:01Z",
  "completed_at": "2024-01-01T00:00:31Z"
}
```

### REST API Endpoints

#### Health Check
```http
GET /health
GET /healthz
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "service": "ai-agent-service",
  "version": "0.2.0"
}
```

#### Service Status
```http
GET /api/v1/status
```

**Response:**
```json
{
  "name": "AI Agent Service",
  "version": "0.2.0",
  "status": "healthy",
  "uptime_seconds": 3600,
  "active_tasks": 2,
  "completed_tasks": 150,
  "failed_tasks": 3,
  "available_tools": ["read_file", "write_file", "run_command", "list_files"],
  "system_metrics": {
    "cpu_usage_percent": 25.5,
    "memory_usage_mb": 512.0,
    "disk_usage_mb": 1024.0,
    "network_io": {
      "bytes_received": 1048576,
      "bytes_sent": 524288,
      "active_connections": 5
    }
  },
  "last_updated": "2024-01-01T00:00:00Z"
}
```

#### Execute Task
```http
POST /api/v1/tasks
Content-Type: application/json

{
  "task": "Read the Cargo.toml file and list dependencies",
  "priority": "normal"
}
```

**Response:** `TaskResponse` (see above)

#### Execute Batch Tasks
```http
POST /api/v1/tasks/batch
Content-Type: application/json

{
  "tasks": [
    {"task": "Task 1"},
    {"task": "Task 2"}
  ],
  "mode": "parallel",
  "continue_on_error": true
}
```

**Response:**
```json
{
  "batch_id": "uuid",
  "responses": [TaskResponse, TaskResponse],
  "statistics": {
    "total_tasks": 2,
    "completed_tasks": 2,
    "failed_tasks": 0,
    "total_execution_time": 15,
    "average_execution_time": 7.5
  }
}
```

#### Get Task Status
```http
GET /api/v1/tasks/{task_id}
```

**Response:** `TaskResponse` (see above)

#### Cancel Task
```http
DELETE /api/v1/tasks/{task_id}
```

**Response:** `204 No Content` or error

#### Get Metrics
```http
GET /api/v1/metrics
```

**Response:**
```json
{
  "uptime_seconds": 3600,
  "total_tasks": 155,
  "completed_tasks": 150,
  "failed_tasks": 3,
  "active_tasks": 2,
  "average_execution_time_seconds": 12.5,
  "tool_usage": {
    "read_file": 45,
    "write_file": 23,
    "run_command": 67,
    "list_files": 20
  },
  "error_counts": {
    "TIMEOUT": 1,
    "TOOL_ERROR": 2
  },
  "system_metrics": {...}
}
```

#### List Available Tools
```http
GET /api/v1/tools
```

**Response:**
```json
{
  "tools": [
    {
      "name": "read_file",
      "description": "Read the contents of a file",
      "parameters": ["path"]
    },
    {
      "name": "write_file",
      "description": "Write content to a file",
      "parameters": ["path", "content"]
    },
    {
      "name": "run_command",
      "description": "Execute a shell command",
      "parameters": ["command", "working_dir"]
    },
    {
      "name": "list_files",
      "description": "List files and directories",
      "parameters": ["path"]
    }
  ]
}
```

## 🔧 Rust API

### Core Traits

#### AiAgentApi
```rust
#[async_trait]
pub trait AiAgentApi: Send + Sync {
    async fn execute_task(&self, request: TaskRequest) -> ServiceResult<TaskResponse>;
    async fn execute_batch(&self, request: BatchTaskRequest) -> ServiceResult<BatchTaskResponse>;
    async fn get_task_status(&self, task_id: &str) -> ServiceResult<TaskResponse>;
    async fn cancel_task(&self, task_id: &str) -> ServiceResult<()>;
    async fn get_service_status(&self) -> ServiceResult<ServiceStatus>;
    async fn get_metrics(&self) -> ServiceResult<MetricsSnapshot>;
    async fn subscribe_to_task_updates(&self, task_id: &str) -> ServiceResult<Box<dyn Stream<Item = WebSocketMessage> + Send>>;
}
```

### Client Builders

#### In-Process Client
```rust
let service = Arc::new(AiAgentService::new(service_config, agent_config).await?);
let client = AiAgentClient::new(ApiClientBuilder::in_process(service));
```

#### HTTP Client
```rust
let client = AiAgentClient::new(
    ApiClientBuilder::http_with_auth("http://localhost:8080", "api-key")
);
```

#### Environment-Based Client
```rust
let client = AiAgentClient::new(ApiClientBuilder::from_env()?);
```

### Convenience Methods

```rust
// Simple task execution
let response = client.execute_simple_task("Hello, world!").await?;

// Task with context
let mut env = HashMap::new();
env.insert("PATH".to_string(), "/usr/bin".to_string());
let response = client.execute_task_with_context("List files", Some("/tmp"), Some(env)).await?;
```

## 🔄 WebSocket API (Future)

The service supports real-time task updates via WebSocket:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws/tasks/{task_id}');

ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    switch (message.type) {
        case 'TaskUpdate':
            console.log('Task status:', message.data.status);
            break;
        case 'TaskProgress':
            console.log('Progress:', message.data.progress);
            break;
        case 'TaskCompleted':
            console.log('Result:', message.data.result);
            break;
        case 'TaskFailed':
            console.error('Error:', message.data.error);
            break;
    }
};
```

## 📊 Metrics and Monitoring

### Prometheus Metrics

The service exports Prometheus metrics at `/metrics`:

- `ai_agent_requests_total` - Total number of API requests
- `ai_agent_request_duration_seconds` - Request duration histogram
- `ai_agent_tasks_total` - Total tasks processed
- `ai_agent_tasks_completed_total` - Completed tasks
- `ai_agent_tasks_failed_total` - Failed tasks
- `ai_agent_active_tasks` - Currently active tasks
- `ai_agent_cpu_usage_percent` - CPU usage
- `ai_agent_memory_usage_mb` - Memory usage

### Custom Metrics

Tasks can include custom metrics in their responses:

```rust
TaskMetrics {
    custom_metrics: {
        "files_processed".to_string() => 15.0,
        "lines_of_code".to_string() => 342.0,
    }
}
```

## 🔒 Authentication

### API Key Authentication

Set the API key via environment variable:

```bash
export AI_AGENT_API_KEY=your-secure-api-key
```

Include it in requests:

```bash
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:8080/api/v1/status
```

### Rate Limiting

Configure rate limiting in service configuration:

```toml
[service.rate_limiting]
requests_per_minute = 60
burst_size = 10
cleanup_interval_seconds = 300
```

## 🚨 Error Handling

### HTTP Status Codes

- `200 OK` - Successful request
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Missing or invalid API key
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Task or resource not found
- `408 Request Timeout` - Task execution timeout
- `429 Too Many Requests` - Rate limit exceeded
- `500 Internal Server Error` - Server error
- `502 Bad Gateway` - Model provider error
- `503 Service Unavailable` - Service at capacity

### Error Response Format

```json
{
  "code": "TASK_TIMEOUT",
  "message": "Task execution timeout",
  "details": "Task exceeded maximum execution time of 300 seconds",
  "stack_trace": "Debug stack trace (development only)",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## 🧪 Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
# Start service
cargo run --bin ai-agent-server &

# Run tests
cargo test --test integration

# Stop service
kill %1
```

### Load Testing
```bash
# Install hey
go install github.com/rakyll/hey@latest

# Load test
hey -n 1000 -c 50 \
  -H "Content-Type: application/json" \
  -d '{"task": "Simple test task"}' \
  http://localhost:8080/api/v1/tasks
```

## 📈 Performance

### Benchmarks

Typical performance characteristics:

- **Cold Start**: 2-5 seconds
- **Task Planning**: 1-3 seconds
- **Simple Task**: 1-10 seconds
- **Complex Task**: 10-60 seconds
- **Concurrent Tasks**: Up to configured limit
- **Memory Usage**: 100-500MB per service instance
- **CPU Usage**: 5-30% during task execution

### Optimization Tips

1. **Batch similar tasks** to reduce planning overhead
2. **Use task constraints** to limit execution time
3. **Set appropriate timeouts** for your use case
4. **Monitor metrics** to identify bottlenecks
5. **Scale horizontally** for high load scenarios

## 🔧 Configuration

### Service Configuration

```toml
[service]
max_concurrent_tasks = 10
default_task_timeout = 300
max_task_timeout = 3600
enable_metrics = true
log_level = "info"

[service.cors]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "DELETE"]
allowed_headers = ["*"]
allow_credentials = false

[service.rate_limiting]
requests_per_minute = 60
burst_size = 10
cleanup_interval_seconds = 300
```

### Environment Variables

```bash
# Service
AI_AGENT_MAX_CONCURRENT_TASKS=10
AI_AGENT_DEFAULT_TASK_TIMEOUT=300
AI_AGENT_ENABLE_METRICS=true
AI_AGENT_LOG_LEVEL=info

# Server
BIND_ADDRESS=0.0.0.0:8080

# Model
AI_AGENT_MODEL_PROVIDER=zhipu
AI_AGENT_MODEL_NAME=glm-4
AI_AGENT_API_KEY=your-api-key

# CORS
AI_AGENT_CORS_ALLOWED_ORIGINS=*
```

## 📚 Examples

See the `/examples` directory for complete examples:

- `rust_client.rs` - Comprehensive Rust client usage
- `http_client.rs` - HTTP client examples
- `in_process_service.rs` - In-process service usage
- `docker-compose.yml` - Complete Docker setup

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/lipish/code-agent/issues)
- **Documentation**: [Main README](../README.md)
- **Examples**: [Examples Directory](./README.md)