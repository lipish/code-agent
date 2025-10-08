#!/bin/bash

# Simple test script to verify agent works

set -e

echo "🧪 Simple Agent Test"
echo "===================="
echo ""

# Export API key
export DEEPSEEK_API_KEY="sk-78f437f4e0174650ae18734e6ec5bd03"
echo "✅ API Key set: ${DEEPSEEK_API_KEY:0:10}...${DEEPSEEK_API_KEY: -10}"
echo ""

# Test 1: Simple question
echo "📋 Test 1: What is 2+2?"
echo ""
cargo run --quiet --bin task-runner -- task "What is 2+2? Just answer with the number."
echo ""
echo "===================="
echo "✅ Test complete!"

