#!/bin/bash

# Agent Test Script
# Tests the agent with different providers and scenarios

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🧪 Agent Testing Script"
echo "======================="
echo ""

# Load API keys from keys.yaml
ZHIPU_KEY=$(grep -A 2 "zhipu:" keys.yaml | grep "api_key:" | awk '{print $2}' | tr -d '"')
DEEPSEEK_KEY=$(grep -A 2 "deepseek:" keys.yaml | grep "api_key:" | awk '{print $2}' | tr -d '"')
MOONSHOT_KEY=$(grep -A 2 "moonshot:" keys.yaml | grep "api_key:" | awk '{print $2}' | tr -d '"')

# Check which provider to use
PROVIDER=${1:-deepseek}

case $PROVIDER in
  zhipu)
    export ZHIPU_API_KEY="$ZHIPU_KEY"
    MODEL="glm-4.6"
    PROVIDER_NAME="Zhipu AI"
    # Update config.toml
    sed -i.bak 's/provider = .*/provider = "zhipu"/' config.toml
    sed -i.bak 's/model_name = .*/model_name = "glm-4.6"/' config.toml
    ;;
  deepseek)
    export DEEPSEEK_API_KEY="$DEEPSEEK_KEY"
    MODEL="deepseek-chat"
    PROVIDER_NAME="DeepSeek"
    # Update config.toml
    sed -i.bak 's/provider = .*/provider = "deepseek"/' config.toml
    sed -i.bak 's/model_name = .*/model_name = "deepseek-chat"/' config.toml
    ;;
  moonshot)
    export MOONSHOT_API_KEY="$MOONSHOT_KEY"
    MODEL="moonshot-v1-8k"
    PROVIDER_NAME="Moonshot"
    # Update config.toml
    sed -i.bak 's/provider = .*/provider = "moonshot"/' config.toml
    sed -i.bak 's/model_name = .*/model_name = "moonshot-v1-8k"/' config.toml
    ;;
  *)
    echo -e "${RED}❌ Unknown provider: $PROVIDER${NC}"
    echo "Usage: $0 [zhipu|deepseek|moonshot]"
    echo ""
    echo "Examples:"
    echo "  $0 deepseek   # Use DeepSeek (recommended)"
    echo "  $0 zhipu      # Use Zhipu AI"
    echo "  $0 moonshot   # Use Moonshot"
    exit 1
    ;;
esac

echo -e "${GREEN}📡 Provider: $PROVIDER_NAME${NC}"
echo -e "${GREEN}🤖 Model: $MODEL${NC}"
echo "======================="
echo ""

# Create test report directory
mkdir -p test_reports
REPORT_FILE="test_reports/agent_test_${PROVIDER}_$(date +%Y%m%d_%H%M%S).md"

# Start report
cat > "$REPORT_FILE" << EOF
# Agent Test Report

**Date**: $(date)
**Provider**: $PROVIDER_NAME
**Model**: $MODEL

---

## Test Cases

EOF

# Test Case 1: Simple Question
echo "📋 Test Case 1: Simple Question"
echo "Task: What is 2+2?"
echo ""

START_TIME=$(date +%s)
RESULT=$(cargo run --quiet --bin task-runner -- task "What is 2+2? Just give me the number." 2>&1 || echo "ERROR")
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

if [[ "$RESULT" != *"ERROR"* ]] && [[ "$RESULT" != *"error"* ]]; then
    echo -e "${GREEN}✅ Success! Duration: ${DURATION}s${NC}"
    echo "Response: $RESULT"
    cat >> "$REPORT_FILE" << EOF
### Test Case 1: Simple Question ✅

**Task**: What is 2+2?
**Duration**: ${DURATION}s
**Status**: Success

**Response**:
\`\`\`
$RESULT
\`\`\`

---

EOF
else
    echo -e "${RED}❌ Failed! Duration: ${DURATION}s${NC}"
    echo "Error: $RESULT"
    cat >> "$REPORT_FILE" << EOF
### Test Case 1: Simple Question ❌

**Task**: What is 2+2?
**Duration**: ${DURATION}s
**Status**: Failed

**Error**:
\`\`\`
$RESULT
\`\`\`

---

EOF
fi

echo ""

# Test Case 2: Code Generation
echo "📋 Test Case 2: Code Generation"
echo "Task: Write a hello world function in Rust"
echo ""

START_TIME=$(date +%s)
RESULT=$(cargo run --quiet --bin task-runner -- task "Write a simple hello world function in Rust. Just show the code." 2>&1 || echo "ERROR")
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

if [[ "$RESULT" != *"ERROR"* ]] && [[ "$RESULT" != *"error"* ]]; then
    echo -e "${GREEN}✅ Success! Duration: ${DURATION}s${NC}"
    echo "Response: ${RESULT:0:200}..."
    cat >> "$REPORT_FILE" << EOF
### Test Case 2: Code Generation ✅

**Task**: Write a hello world function in Rust
**Duration**: ${DURATION}s
**Status**: Success

**Response**:
\`\`\`
$RESULT
\`\`\`

---

EOF
else
    echo -e "${RED}❌ Failed! Duration: ${DURATION}s${NC}"
    echo "Error: $RESULT"
    cat >> "$REPORT_FILE" << EOF
### Test Case 2: Code Generation ❌

**Task**: Write a hello world function in Rust
**Duration**: ${DURATION}s
**Status**: Failed

**Error**:
\`\`\`
$RESULT
\`\`\`

---

EOF
fi

echo ""

# Test Case 3: Analysis
echo "📋 Test Case 3: Analysis"
echo "Task: List 3 benefits of Rust"
echo ""

START_TIME=$(date +%s)
RESULT=$(cargo run --quiet --bin task-runner -- task "List 3 main benefits of Rust programming language. Be concise." 2>&1 || echo "ERROR")
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

if [[ "$RESULT" != *"ERROR"* ]] && [[ "$RESULT" != *"error"* ]]; then
    echo -e "${GREEN}✅ Success! Duration: ${DURATION}s${NC}"
    echo "Response: ${RESULT:0:200}..."
    cat >> "$REPORT_FILE" << EOF
### Test Case 3: Analysis ✅

**Task**: List 3 benefits of Rust
**Duration**: ${DURATION}s
**Status**: Success

**Response**:
\`\`\`
$RESULT
\`\`\`

---

EOF
else
    echo -e "${RED}❌ Failed! Duration: ${DURATION}s${NC}"
    echo "Error: $RESULT"
    cat >> "$REPORT_FILE" << EOF
### Test Case 3: Analysis ❌

**Task**: List 3 benefits of Rust
**Duration**: ${DURATION}s
**Status**: Failed

**Error**:
\`\`\`
$RESULT
\`\`\`

---

EOF
fi

echo ""
echo "======================="
echo "📊 Test Summary"
echo "======================="
echo ""
echo -e "${GREEN}📄 Full report saved to: $REPORT_FILE${NC}"
echo ""

# Add summary to report
cat >> "$REPORT_FILE" << EOF

## Summary

**Provider**: $PROVIDER_NAME
**Model**: $MODEL
**Total Tests**: 3
**Report Location**: $REPORT_FILE

## Observations

1. **Response Quality**: Check if responses are accurate and helpful
2. **Performance**: Monitor response times
3. **Error Handling**: Verify error messages are clear

## Recommendations

Based on the test results:

- If all tests passed: This provider works well for your use case
- If some tests failed: Check error messages and adjust configuration
- Compare results across different providers to find the best one

### Provider Comparison

Run tests with different providers:
\`\`\`bash
./test_agent.sh deepseek   # Usually fastest and most reliable
./test_agent.sh zhipu      # Good for Chinese language tasks
./test_agent.sh moonshot   # Good balance of speed and quality
\`\`\`

---

*Generated by Agent Test Script*
EOF

# Restore config.toml backup
if [ -f config.toml.bak ]; then
    mv config.toml.bak config.toml
fi

echo "✅ Testing complete!"
echo ""
echo "💡 Tips:"
echo "  - View report: cat $REPORT_FILE"
echo "  - Test another provider: ./test_agent.sh [zhipu|deepseek|moonshot]"
echo "  - Compare results across providers"
echo ""

