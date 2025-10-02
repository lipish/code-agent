#!/bin/bash

# Quick Demo of Enhanced AI Agent Execution
echo "🎯 AI Agent Enhanced Execution Demo"
echo "================================="

export ZHIPU_API_KEY=d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3

# Change to project root directory
cd ..

echo ""
echo "📝 This demo shows the enhanced execution process with detailed output"
echo ""

# Create a simple test file
echo "Project: AI-Native Code Agent" > test/demo_info.txt
echo "Language: Rust" >> test/demo_info.txt
echo "Purpose: AI-powered coding assistant" >> test/demo_info.txt

echo "🚀 Running a simple task with verbose output..."
echo ""

# Run with verbose output to show the enhanced execution process
cargo run -- task "Read the test/demo_info.txt file and tell me what this project is about" --output verbose

echo ""
echo "🧹 Cleaning up..."
rm -f test/demo_info.txt

echo ""
echo "✅ Demo completed!"
echo "You can now use --output verbose for detailed execution information."