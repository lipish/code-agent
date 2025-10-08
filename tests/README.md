# Tests Directory

This directory contains all test scripts for the task-runner project.

---

## 📋 Test Scripts

### 1. test_api_connection.sh

**Purpose**: Test API connectivity to different LLM providers

**What it does**:
- Tests connection to Zhipu AI
- Tests connection to DeepSeek
- Tests connection to Moonshot
- Verifies API keys are valid
- Shows HTTP response codes

**Usage**:
```bash
cd tests
./test_api_connection.sh
```

**Expected Output**:
```
🧪 Testing Zhipu AI...
  ✅ Success! HTTP 200
  Response: Hello! How can I help you?

🧪 Testing DeepSeek...
  ✅ Success! HTTP 200
  Response: Hello! How can I assist you today?

🧪 Testing Moonshot...
  ✅ Success! HTTP 200
  Response: Hi! What can I do for you?
```

**When to use**:
- Before running agent tests
- To verify API keys are correct
- To check which providers are working
- To diagnose connection issues

---

### 2. test_simple.sh

**Purpose**: Quick smoke test of the agent

**What it does**:
- Sets up environment variables
- Runs a simple task ("What is 2+2?")
- Shows basic agent functionality

**Usage**:
```bash
cd tests
./test_simple.sh
```

**Expected Output**:
```
🧪 Simple Agent Test
====================

✅ API Key set: sk-78f437f...4e6ec5bd03

📋 Test 1: What is 2+2?

🚀 Starting AI Agent Task Execution
====================================
✅ Task Status: SUCCESS
📝 Result: 4
====================================
```

**When to use**:
- Quick verification that agent works
- After code changes
- Before running full test suite

---

### 3. test_agent.sh

**Purpose**: Comprehensive agent testing with multiple scenarios

**What it does**:
- Tests 3 different scenarios:
  1. Simple question
  2. Code generation
  3. Analysis task
- Generates detailed markdown report
- Measures response times
- Records success/failure status

**Usage**:
```bash
cd tests

# Test with DeepSeek (recommended)
./test_agent.sh deepseek

# Test with Zhipu AI
./test_agent.sh zhipu

# Test with Moonshot
./test_agent.sh moonshot
```

**Test Cases**:

1. **Simple Question**: "What is 2+2?"
   - Tests basic reasoning
   - Fast response expected

2. **Code Generation**: "Write a hello world function in Rust"
   - Tests code generation capability
   - Checks code quality

3. **Analysis**: "List 3 benefits of Rust"
   - Tests analytical thinking
   - Checks response structure

**Output**:
- Console output with colored status
- Markdown report in `../test_reports/`
- Timing information
- Success/failure status

**Report Location**:
```
test_reports/agent_test_deepseek_20251008_091234.md
```

**When to use**:
- Full agent evaluation
- Comparing different providers
- Before production deployment
- Performance benchmarking

---

## 🚀 Quick Start

### Prerequisites

1. **API Keys**: Ensure `keys.yaml` is configured
   ```yaml
   providers:
     zhipu:
       api_key: "your-key-here"
     deepseek:
       api_key: "your-key-here"
     moonshot:
       api_key: "your-key-here"
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

### Running Tests

**Step 1**: Test API connectivity
```bash
cd tests
./test_api_connection.sh
```

**Step 2**: Quick smoke test
```bash
./test_simple.sh
```

**Step 3**: Full agent test
```bash
./test_agent.sh deepseek
```

---

## 📊 Test Reports

Test reports are saved in `test_reports/` directory:

```
test_reports/
├── agent_test_deepseek_20251008_091234.md
├── agent_test_zhipu_20251008_092345.md
└── agent_test_moonshot_20251008_093456.md
```

**Report Format**:
```markdown
# Agent Test Report

**Date**: 2025-10-08
**Provider**: DeepSeek
**Model**: deepseek-chat

## Test Cases

### Test Case 1: Simple Question ✅
**Task**: What is 2+2?
**Duration**: 2s
**Status**: Success
**Response**: 4

### Test Case 2: Code Generation ✅
**Task**: Write a hello world function in Rust
**Duration**: 5s
**Status**: Success
**Response**: [code here]

## Summary
**Total Tests**: 3
**Passed**: 3
**Failed**: 0
**Success Rate**: 100%
```

---

## 🔧 Troubleshooting

### Issue: API Authentication Failed

**Symptoms**:
```
❌ Failed! HTTP 401
Error: Authentication failed: Incorrect API key
```

**Solutions**:
1. Check `keys.yaml` has correct API keys
2. Verify API key format (no extra spaces)
3. Check if API key has expired
4. Run `./test_api_connection.sh` to diagnose

---

### Issue: Connection Timeout

**Symptoms**:
```
❌ Failed! HTTP 000
Error: Connection timeout
```

**Solutions**:
1. Check internet connection
2. Verify base_url in `keys.yaml`
3. Check if provider service is down
4. Try different provider

---

### Issue: Model Not Found

**Symptoms**:
```
❌ Failed! HTTP 404
Error: Model not found
```

**Solutions**:
1. Check model name in `config.toml`
2. Verify model is available for your API key
3. Check provider documentation for model names

---

## 📚 Related Documentation

- `../doc/AGENT_TESTING_GUIDE.md` - Comprehensive testing guide
- `../doc/AGENT_TYPES.md` - Agent types documentation
- `../keys.yaml` - API configuration
- `../config.toml` - Agent configuration

---

## 🎯 Best Practices

### Before Committing Code

1. Run API connection test
2. Run simple smoke test
3. Run full agent test with at least one provider
4. Check test reports for any failures

### Before Production Deployment

1. Test all providers
2. Compare performance across providers
3. Review test reports
4. Verify success rate > 90%

### Regular Testing

- Run tests weekly to catch API changes
- Monitor response times
- Track success rates over time
- Update test cases as needed

---

## 🔄 CI/CD Integration

To integrate these tests into CI/CD:

```yaml
# .github/workflows/test.yml
name: Agent Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
      - name: Build
        run: cargo build --release
      - name: Test API Connection
        run: cd tests && ./test_api_connection.sh
        env:
          DEEPSEEK_API_KEY: ${{ secrets.DEEPSEEK_API_KEY }}
      - name: Run Agent Tests
        run: cd tests && ./test_agent.sh deepseek
        env:
          DEEPSEEK_API_KEY: ${{ secrets.DEEPSEEK_API_KEY }}
```

---

*Last updated: 2025-10-08*

