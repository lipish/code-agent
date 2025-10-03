# AI Agent Service Examples

This directory contains examples of how to use the AI Agent Service both as a Rust library and as an HTTP service.

## 📁 Examples Overview

### 1. **Rust Client Examples**

#### `rust_client.rs`
Demonstrates how to use the AI Agent Service from Rust code:
- Basic task execution
- Tasks with custom context
- Priority tasks
- Service health checking
- Metrics retrieval
- Batch processing

**Run with:**
```bash
cd examples
cargo run --example rust_client --features service --features service
```

#### `in_process_service.rs`
Shows how to run the AI Agent Service in-process (without HTTP):
- Creating a service instance
- In-process API usage
- Concurrent task execution
- Service monitoring

**Run with:**
```bash
cd examples
cargo run --example in_process_service --features service
```

#### `http_client.rs`
Demonstrates HTTP client usage:
- Connecting to remote service
- File operations via HTTP
- Code generation tasks
- Batch processing over HTTP
- Task monitoring

**Run with:**
```bash
cd examples
# Start the service first
cargo run --bin code-agent-server --features service

# In another terminal, run the client
cargo run --example http_client --features service
```

### 2. **Docker Deployment**

#### `docker-compose.yml`
Complete Docker setup with:
- AI Agent Service
- Prometheus monitoring
- Grafana dashboard
- Health checks
- Volume mounts

**Deploy with:**
```bash
cd examples
docker-compose up -d
```

## 🚀 Quick Start

### Option 1: In-Process Usage
```rust
use ai_agent::{service::{AiAgentService, ServiceConfig, AiAgentClient, ApiClientBuilder}, config::AgentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service
    let service = Arc::new(AiAgentService::new(
        ServiceConfig::default(),
        AgentConfig::load_with_fallback("config.toml")?
    ).await?);

    // Create client
    let client = AiAgentClient::new(ApiClientBuilder::in_process(service));

    // Execute a task
    let response = client.execute_simple_task("Hello, world!").await?;
    println!("Result: {}", response.result.unwrap().summary);

    Ok(())
}
```

### Option 2: HTTP Client Usage
```rust
use ai_agent::service::{AiAgentClient, ApiClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client
    let client = AiAgentClient::new(
        ApiClientBuilder::http_with_auth("http://localhost:8080", "your-api-key")
    );

    // Execute a task
    let response = client.execute_simple_task("List files in current directory").await?;
    println!("Result: {}", response.result.unwrap().summary);

    Ok(())
}
```

### Option 3: Environment Configuration
```bash
export AI_AGENT_API_URL=http://localhost:8080
export AI_AGENT_API_KEY=your-api-key

# The client will automatically use these
cargo run --example rust_client --features service
```

## 🔧 Configuration Examples

### Service Configuration
```toml
[service]
max_concurrent_tasks = 10
default_task_timeout = 300
enable_metrics = true
log_level = "info"

[service.cors]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "DELETE"]
allowed_headers = ["*"]
allow_credentials = false
```

### Environment Variables
```bash
# Service configuration
AI_AGENT_MAX_CONCURRENT_TASKS=10
AI_AGENT_DEFAULT_TASK_TIMEOUT=300
AI_AGENT_ENABLE_METRICS=true
AI_AGENT_LOG_LEVEL=info

# Model configuration
AI_AGENT_MODEL_PROVIDER=zhipu
AI_AGENT_MODEL_NAME=glm-4
AI_AGENT_API_KEY=your-api-key

# Server configuration
BIND_ADDRESS=0.0.0.0:8080
```

## 📊 API Endpoints

### REST API
- `GET /health` - Health check
- `GET /api/v1/status` - Service status
- `GET /api/v1/metrics` - Service metrics
- `GET /api/v1/tools` - Available tools
- `POST /api/v1/tasks` - Execute task
- `POST /api/v1/tasks/batch` - Execute batch
- `GET /api/v1/tasks/{id}` - Get task status
- `DELETE /api/v1/tasks/{id}` - Cancel task

### Task Request Format
```json
{
  "task": "Read the README.md file",
  "task_id": "optional-custom-id",
  "context": {
    "working_directory": "/path/to/dir",
    "environment": {"VAR": "value"},
    "tools": ["read_file", "write_file"],
    "constraints": {
      "max_execution_time": 300,
      "max_steps": 10,
      "allowed_paths": ["/safe/path"]
    }
  },
  "priority": "normal",
  "metadata": {"key": "value"}
}
```

### Task Response Format
```json
{
  "task_id": "uuid",
  "status": "completed",
  "result": {
    "success": true,
    "summary": "Task completed successfully",
    "details": "File content here...",
    "artifacts": [],
    "execution_time": 5
  },
  "plan": {
    "understanding": "Understanding of the task",
    "approach": "Approach to solve it",
    "complexity": "Simple",
    "estimated_steps": 1,
    "requirements": ["read_file tool"],
    "created_at": "2024-01-01T00:00:00Z"
  },
  "steps": [...],
  "metrics": {...}
}
```

## 🐳 Docker Usage

### Build and Run
```bash
# Build the image
docker build -t ai-agent-service .

# Run the service
docker run -p 8080:8080 \
  -e AI_AGENT_API_KEY=your-api-key \
  ai-agent-service
```

### Docker Compose
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f ai-agent-service

# Stop services
docker-compose down
```

## 📈 Monitoring

### Prometheus Metrics
Available at `http://localhost:9090` when using Docker Compose:
- `ai_agent_requests_total` - Total requests
- `ai_agent_request_duration_seconds` - Request duration histogram
- `ai_agent_active_tasks` - Current active tasks
- `ai_agent_completed_tasks_total` - Completed tasks
- `ai_agent_failed_tasks_total` - Failed tasks

### Grafana Dashboard
Available at `http://localhost:3000` (admin/admin) when using Docker Compose:
- Service health overview
- Task execution metrics
- Performance trends
- Error rates

## 🧪 Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
# Start service
cargo run --bin ai-agent-server &

# Run integration tests
cargo test --test integration

# Stop service
kill %1
```

### Load Testing
```bash
# Install hey
go install github.com/rakyll/hey@latest

# Load test the service
hey -n 100 -c 10 -m POST \
  -H "Content-Type: application/json" \
  -d '{"task": "Simple test task"}' \
  http://localhost:8080/api/v1/tasks
```

## 🔒 Security

### API Keys
Set API keys via environment variables:
```bash
export AI_AGENT_API_KEY=your-secure-api-key
```

### HTTPS
Configure HTTPS by setting up a reverse proxy:
```nginx
server {
    listen 443 ssl;
    server_name your-domain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 🛠️ Troubleshooting

### Common Issues

1. **Service won't start**
   - Check AI_AGENT_API_KEY is set
   - Verify model configuration
   - Check log output

2. **Tasks timeout**
   - Increase AI_AGENT_DEFAULT_TASK_TIMEOUT
   - Check system resources
   - Monitor task complexity

3. **Connection refused**
   - Ensure service is running
   - Check BIND_ADDRESS configuration
   - Verify firewall settings

4. **High memory usage**
   - Reduce AI_AGENT_MAX_CONCURRENT_TASKS
   - Monitor task execution
   - Set appropriate task limits

### Debug Mode
```bash
RUST_LOG=debug cargo run --bin ai-agent-server
```

## 📚 Additional Resources

- [Main README](../README.md)
- [API Documentation](../doc/API.md)
- [Configuration Guide](../doc/CONFIGURATION.md)
- [Deployment Guide](../doc/DEPLOYMENT.md)