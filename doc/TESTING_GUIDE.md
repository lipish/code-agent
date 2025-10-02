# AI Agent Testing Guide

## Quick Start

1. **Set Environment Variable:**
   ```bash
   export ZHIPU_API_KEY=your-api-key-here
   # Or copy .env.example to .env and configure there
   ```

2. **Test CLI Interface:**
   ```bash
   cargo run -- task "测试任务"
   ```

3. **Test HTTP Service:**
   ```bash
   # Start the service
   cargo run --bin ai-agent-server

   # In another terminal, test the API
   curl -X POST http://localhost:8080/api/v1/tasks \
     -H "Content-Type: application/json" \
     -d '{"task": "测试任务"}'
   ```

## Testing Methods

### 1. CLI Testing

```bash
# Basic conversation
cargo run -- task "Hello! Introduce yourself."

# File operations
cargo run -- task "Read the README.md file and summarize it."

# Directory operations
cargo run -- task "List all Rust source files in the project."

# Command execution
cargo run -- task "Run 'cargo check' and show me the results."

# File creation
cargo run -- task "Create a file called test.txt with the content 'Hello World'."
```

### 2. HTTP Service Testing

```bash
# Start the service
cargo run --bin ai-agent-server

# Test health endpoint
curl http://localhost:8080/health

# Test service status
curl http://localhost:8080/api/v1/status

# Test task execution
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"task": "Read README.md and summarize"}'

# Test batch tasks
curl -X POST http://localhost:8080/api/v1/tasks/batch \
  -H "Content-Type: application/json" \
  -d '{
    "tasks": [
      {"task": "List files in current directory"},
      {"task": "Show git status"}
    ],
    "mode": "parallel"
  }'

# Get metrics
curl http://localhost:8080/api/v1/metrics
```

### 3. Interactive Mode

```bash
cargo run -- interactive
```

### 4. Rust API Testing

```bash
# Run Rust client examples
cargo run --example rust_client
cargo run --example http_client
cargo run --example in_process_service
```

### 5. Docker Testing

```bash
# Build Docker image
docker build -t ai-agent-service .

# Run container
docker run -p 8080:8080 \
  -e AI_AGENT_API_KEY=your-api-key \
  ai-agent-service

# Test with Docker Compose
cd examples
docker-compose up -d
```

```

## Test Categories

### CLI Tests
- [ ] CLI startup and initialization
- [ ] Configuration loading
- [ ] Tool registration and discovery
- [ ] Basic task execution

### Service Tests
- [ ] HTTP service startup and health checks
- [ ] API endpoint functionality
- [ ] Concurrent task handling
- [ ] Metrics collection

### Tool-Specific Tests
- [ ] **read_file**: Read various file types (txt, toml, rs)
- [ ] **write_file**: Create and modify files
- [ ] **list_files**: Directory listing with different paths
- [ ] **run_command**: Execute safe shell commands

### Integration Tests
- [ ] Multi-step tasks requiring multiple tools
- [ ] Error handling and recovery
- [ ] Safety checks and blocked commands
- [ ] Large file handling
- [ ] Batch task execution

### Performance Tests
- [ ] Response time measurement
- [ ] Concurrent task handling
- [ ] Memory usage monitoring
- [ ] API rate limiting

## Sample Test Commands

### File Operations
```bash
# Test reading different file types
cargo run -- task "Read config.toml and show me the model configuration"
cargo run -- task "Read src/main.rs and summarize what it does"

# Test file writing
cargo run -- task "Create a summary.txt file with a brief summary of this project"
```

### Command Execution
```bash
# Safe commands
cargo run -- task "Run 'cargo --version' and tell me the version"
cargo run -- task "Execute 'ls -la' and show me the output"
cargo run -- task "Run 'date' and tell me the current date and time"

# Blocked commands (should fail)
cargo run -- task "Run 'rm -rf /' (this should be blocked)"
```

### Complex Tasks
```bash
# Multi-step tasks
cargo run -- task "Analyze this project: read the README, list source files, and create a summary"
cargo run -- task "Check the project structure and create a documentation file"
cargo run -- task "Validate the Rust code and report any issues found"
```

## Troubleshooting

### Common Issues

1. **API Key Not Found**
   ```bash
   export ZHIPU_API_KEY=your_api_key_here
   ```

2. **Configuration Loading Issues**
   ```bash
   # Check if config file exists and is valid
   cat config.toml
   cargo run -- config
   ```

3. **Tool Format Errors**
   - Currently experiencing tool format compatibility issues with zhipu API
   - This affects tasks that require tool usage
   - Basic conversation should work once the format issue is resolved

4. **Network/Connection Issues**
   ```bash
   # Test API connectivity
   curl -X POST "https://open.bigmodel.cn/api/paas/v4/chat/completions" \
     -H "Authorization: Bearer $ZHIPU_API_KEY" \
     -H "Content-Type: application/json" \
     -d '{"model":"GLM-4.6","messages":[{"role":"user","content":"Hello"}]}'
   ```

## Current Status

✅ **Working:**
- Configuration loading
- Environment variable setup
- Tool registration
- API authentication
- Basic connectivity to zhipu API

⚠️ **Known Issues:**
- Tool format compatibility with zhipu API
- Agent always tries to use tools even for simple tasks
- Some requests fail with "tools[0].type:不能为空" error

🔧 **In Progress:**
- Fixing tool format for zhipu API compatibility
- Implementing proper tool usage detection
- Testing with various task types

## Next Steps

1. Fix the tool format issue
2. Test basic conversation functionality
3. Verify tool operations work correctly
4. Run comprehensive integration tests
5. Performance optimization and monitoring