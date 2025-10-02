#!/bin/bash

# Main test runner for AI Agent
echo "🧪 AI Agent Test Suite"
echo "====================="
echo "Running all tests from test/ directory"
echo ""

# Get the project root directory
PROJECT_ROOT=$(cd .. && pwd)
export ZHIPU_API_KEY=d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3

echo "📁 Project root: $PROJECT_ROOT"
echo "📁 Test directory: $(pwd)"
echo ""

# Function to run a test script
run_test() {
    local script_name="$1"
    local description="$2"

    echo "🚀 Running: $description"
    echo "📜 Script: $script_name"
    echo "----------------------------------------"

    if [ -f "./$script_name" ]; then
        chmod +x "./$script_name"
        ./"$script_name"
        local exit_code=$?
        echo "----------------------------------------"
        if [ $exit_code -eq 0 ]; then
            echo "✅ $script_name completed successfully"
        else
            echo "❌ $script_name failed with exit code $exit_code"
        fi
    else
        echo "❌ Test script $script_name not found"
    fi
    echo ""
}

# Run all tests
echo "🎯 Available Tests:"
echo ""

run_test "demo.sh" "Quick Demo with Verbose Output"
run_test "test_detailed.sh" "Detailed Execution Demo"
run_test "test_agent.sh" "Comprehensive Test Suite"

echo ""
echo "🔧 Additional Tests:"
echo "----------------------------------------"

# Test individual file operations
echo "📄 Testing file reading..."
cd "$PROJECT_ROOT"
cargo run -- task "Read the test/test_file and summarize its contents"

echo ""
echo "🔧 Testing script execution..."
cargo run -- task "Run the test/test_script and show the output"

echo ""
echo "📁 Testing directory listing..."
cargo run -- task "List all files in the test/ directory"

echo ""
echo "✅ All tests completed!"
echo "====================="