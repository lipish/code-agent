#!/bin/bash

# Detailed Execution Test Script for AI Agent
echo "🔍 AI Agent Detailed Execution Test"
echo "=================================="

# Set up environment
export ZHIPU_API_KEY=d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3

# Change to project root directory
cd ..

# Function to run with detailed output
detailed_task() {
    local task="$1"
    local description="$2"

    echo ""
    echo "🚀 STARTING TASK: $description"
    echo "📝 Task Description: $task"
    echo "⏰ Started at: $(date)"
    echo "=================================="

    # Enable verbose cargo output
    echo "🔧 Running with cargo output:"
    echo ""

    # Run with detailed output
    RUST_LOG=debug cargo run -- task "$task" --output text

    local exit_code=$?
    echo ""
    echo "=================================="
    echo "🏁 Task completed at: $(date)"
    echo "📊 Exit code: $exit_code"
    if [ $exit_code -eq 0 ]; then
        echo "✅ SUCCESS"
    else
        echo "❌ FAILED"
    fi
    echo ""
    read -p "Press Enter to continue to next task..."
}

echo ""
echo "🎯 This test will show detailed execution process"
echo "📋 We'll run 3 progressively complex tasks"
echo ""
read -p "Press Enter to begin testing..."

# Task 1: Simple file reading
echo "📁 Setting up test files..."
echo "Rust project structure:" > test/project_info.txt
echo "- src/main.rs: Entry point" >> test/project_info.txt
echo "- src/agent.rs: Core agent logic" >> test/project_info.txt
echo "- src/models.rs: AI model interfaces" >> test/project_info.txt
echo "- src/tools.rs: Tool implementations" >> test/project_info.txt

detailed_task \
    "Read the test/project_info.txt file and summarize its contents" \
    "Simple File Reading Task"

# Task 2: Multi-step file operations
detailed_task \
    "List all .rs files in the src/ directory, then create a file called rust_files.md that lists each file with a brief description of what you think it does based on its name" \
    "Multi-step File Operations Task"

# Task 3: Complex analysis task
detailed_task \
    "Analyze this Rust project by: 1) Read the Cargo.toml to understand dependencies, 2) Check the main.rs to understand the application structure, 3) Look at the agent.rs to understand the core functionality, then create a project_overview.md with your findings" \
    "Complex Project Analysis Task"

echo ""
echo "🧹 Cleaning up test files..."
rm -f test/project_info.txt rust_files.md project_overview.md

echo ""
echo "🎉 All tests completed!"
echo "======================"
echo "You've seen the agent handle tasks of increasing complexity."
echo "Check the terminal output above for detailed execution process."