#!/bin/bash

# Test Script for AI Agent with Zhipu GLM-4.6
echo "🤖 AI Agent Testing Script - Enhanced"
echo "===================================="

# Function to print a section header
print_section() {
    echo ""
    echo "📍 $1"
    echo "----------------------------------------"
}

# Function to run a task with detailed output
run_task() {
    local task_description="$1"
    local expected_complexity="$2"

    echo "🎯 Task: $task_description"
    echo "📊 Expected Complexity: $expected_complexity"
    echo "⏱️  Running..."
    echo "----------------------------------------"

    # Run the task and capture the output
    cargo run -- task "$task_description"

    local exit_code=$?
    echo "----------------------------------------"
    if [ $exit_code -eq 0 ]; then
        echo "✅ Task completed successfully"
    else
        echo "❌ Task failed with exit code $exit_code"
    fi
    echo ""
}

# Set up environment
export ZHIPU_API_KEY=d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3

# Change to project root directory
cd ..

print_section "1️⃣  Configuration Test"
cargo run -- config

print_section "2️⃣  Tools Test"
cargo run -- tools

print_section "3️⃣  Simple Tasks (Non-Tool)"
run_task "Hello! Please introduce yourself and tell me what you can do." "Simple"

print_section "4️⃣  Basic File Operations"
echo "Creating test files..."
echo "This is a test file for reading operations." > test/test_file.txt
echo "print('Hello from Python!')" > test/test_script.py

run_task "Please read the content of test/test_file.txt and tell me what it contains." "Simple"

print_section "5️⃣  Directory Operations"
run_task "Please list all files in the current directory and identify which are Rust source files." "Simple"

print_section "6️⃣  Command Execution"
run_task "Please run the command 'echo Hello from AI Agent!' and show me the output." "Simple"

print_section "7️⃣  File Writing Test"
run_task "Please write a short poem about AI to a file called ai_poem.txt." "Simple"

print_section "8️⃣  Multi-Step Task Test"
run_task "Please read the README.md file, create a summary of this project, and write that summary to a file called project_summary.md." "Moderate"

print_section "9️⃣  Complex Project Analysis"
run_task "Analyze this Rust project by: 1) Read all .rs files in src/, 2) Identify the main components, 3) Create a architecture overview, 4) Write a detailed analysis to project_analysis.md including strengths and suggestions." "Complex"

print_section "🔟  Advanced Development Task"
run_task "Create a development documentation file called DEVELPMENT.md that includes: 1) How to set up the project, 2) How to run tests, 3) Available commands, 4) Architecture explanation based on the actual code structure." "Complex"

print_section "1️⃣1️⃣  Code Quality Analysis"
run_task "Analyze the code quality of this project by: 1) Check if there are any TODO comments, 2) Look for potential improvements in error handling, 3) Suggest better naming conventions, 4) Write a code_quality_report.md with findings." "Complex"

print_section "1️⃣2️⃣  Test Suite Enhancement"
run_task "Examine the existing test files and create an enhanced test suite by: 1) Reading all existing tests, 2) Identifying missing test cases, 3) Creating a comprehensive_test_plan.md with suggestions for improving test coverage." "Moderate"

print_section "1️⃣3️⃣  Interactive Mode Demo"
echo "Starting interactive mode for manual testing..."
echo "Commands to try in interactive mode:"
echo "  - 'list the files in src/'"
echo "  - 'read the Cargo.toml file'"
echo "  - 'run cargo check and show results'"
echo "  - 'help'"
echo "  - 'exit'"
echo ""
read -p "Press Enter to start interactive mode, or Ctrl+C to skip..."

cargo run -- interactive

print_section "1️⃣4️⃣  Cleanup"
echo "Cleaning up test files..."
rm -f test/test_file.txt test/test_script.py ai_poem.txt project_summary.md project_analysis.md DEVELPMENT.md code_quality_report.md comprehensive_test_plan.md

echo ""
echo "🎉 Enhanced Testing Complete!"
echo "============================="
echo "Files created during testing have been cleaned up."
echo "Test results show the agent's capabilities across various complexity levels."