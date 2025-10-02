# AI Agent Testing Guide

## Quick Start

1. **Set Environment Variable:**
   ```bash
   export ZHIPU_API_KEY=d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3
   ```

2. **Check Configuration:**
   ```bash
   cargo run -- config
   ```

3. **Verify Tools:**
   ```bash
   cargo run -- tools
   ```

## Testing Methods

### 1. Single Task Testing

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

### 2. Interactive Mode

```bash
cargo run -- interactive
```

In interactive mode, you can use these commands:
- `help` - Show available commands
- `tools` - List available tools
- `exit` or `quit` - Exit the program
- Any other text will be processed as a task

### 3. Configuration Testing

```bash
# Check current configuration
cargo run -- config

# Test with different config files
cargo run -- config --config my_config.toml
```

### 4. Automated Testing

```bash
# Run the comprehensive test suite
./test_agent.sh

# Test individual scenarios
cargo run -- task "What files are in the src directory?"
cargo run -- task "Read Cargo.toml and show me the dependencies"
```

## Test Categories

### Basic Functionality Tests
- [ ] Agent startup and initialization
- [ ] Configuration loading
- [ ] Tool registration and discovery
- [ ] Basic conversation without tools

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
cargo run -- task "List all files in the src/models directory"

# Test file writing
cargo run -- task "Create a summary.txt file with a brief summary of this project"
cargo run -- task "Write a Rust function to calculate fibonacci to a file called fib.rs"
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