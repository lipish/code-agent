# AI Agent Test Suite

This directory contains all test files and scripts for the AI-Native Code Agent project.

## 📁 Test Files

### Test Scripts
- **`run_tests.sh`** - Main test runner that executes all tests
- **`test_agent.sh`** - Comprehensive test suite with 14 different tests
- **`test_detailed.sh`** - Detailed execution demo with 3 progressively complex tasks
- **`demo.sh`** - Quick demonstration of enhanced verbose output

### Test Resources
- **`test_file`** - Sample text file for file reading tests
- **`test_script`** - Executable script for command execution tests

## 🚀 Usage

### Run All Tests
```bash
cd test
./run_tests.sh
```

### Run Individual Tests
```bash
cd test

# Comprehensive test suite
./test_agent.sh

# Detailed execution demo
./test_detailed.sh

# Quick demo with verbose output
./demo.sh
```

### Test Categories

1. **Simple Tests**: Basic functionality, file operations, command execution
2. **Moderate Tests**: Multi-step tasks, project analysis
3. **Complex Tests**: Code quality analysis, documentation generation

## 📋 Test Coverage

- ✅ Configuration loading
- ✅ Tool registration and execution
- ✅ File operations (read, write, list)
- ✅ Command execution
- ✅ Interactive mode
- ✅ Project analysis
- ✅ Documentation generation
- ✅ Error handling

## 🔧 Environment Setup

The test scripts automatically set the required environment variable:
```bash
export ZHIPU_API_KEY=d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3
```

## 📊 Output Formats

Tests demonstrate different output formats:
- `text` (default) - Standard output
- `json` - Structured JSON output
- `verbose` - Enhanced detailed output with performance metrics

## 🧹 Cleanup

All test scripts automatically clean up created files after execution.