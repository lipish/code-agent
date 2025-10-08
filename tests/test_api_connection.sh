#!/bin/bash

# API Connection Test Script
# Tests connection to different LLM providers

echo "🔍 API Connection Test"
echo "======================"
echo ""

# Load API keys from keys.yaml
ZHIPU_KEY=$(grep -A 2 "zhipu:" keys.yaml | grep "api_key:" | awk '{print $2}' | tr -d '"')
ZHIPU_URL=$(grep -A 3 "zhipu:" keys.yaml | grep "base_url:" | awk '{print $2}' | tr -d '"')

DEEPSEEK_KEY=$(grep -A 2 "deepseek:" keys.yaml | grep "api_key:" | awk '{print $2}' | tr -d '"')
DEEPSEEK_URL=$(grep -A 3 "deepseek:" keys.yaml | grep "base_url:" | awk '{print $2}' | tr -d '"')

MOONSHOT_KEY=$(grep -A 2 "moonshot:" keys.yaml | grep "api_key:" | awk '{print $2}' | tr -d '"')
MOONSHOT_URL=$(grep -A 3 "moonshot:" keys.yaml | grep "base_url:" | awk '{print $2}' | tr -d '"')

echo "📋 Loaded API Keys:"
echo "  Zhipu: ${ZHIPU_KEY:0:10}...${ZHIPU_KEY: -10}"
echo "  DeepSeek: ${DEEPSEEK_KEY:0:10}...${DEEPSEEK_KEY: -10}"
echo "  Moonshot: ${MOONSHOT_KEY:0:10}...${MOONSHOT_KEY: -10}"
echo ""

# Test Zhipu AI
echo "🧪 Testing Zhipu AI..."
echo "  URL: $ZHIPU_URL"
echo "  Model: glm-4.6"
echo ""

RESPONSE=$(curl -s -w "\nHTTP_CODE:%{http_code}" \
  -X POST "$ZHIPU_URL/chat/completions" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ZHIPU_KEY" \
  -d '{
    "model": "glm-4.6",
    "messages": [{"role": "user", "content": "Hello"}],
    "max_tokens": 10
  }')

HTTP_CODE=$(echo "$RESPONSE" | grep "HTTP_CODE:" | cut -d: -f2)
BODY=$(echo "$RESPONSE" | sed '/HTTP_CODE:/d')

if [ "$HTTP_CODE" = "200" ]; then
    echo "  ✅ Success! HTTP $HTTP_CODE"
    echo "  Response: $(echo $BODY | jq -r '.choices[0].message.content' 2>/dev/null || echo $BODY)"
else
    echo "  ❌ Failed! HTTP $HTTP_CODE"
    echo "  Error: $(echo $BODY | jq -r '.error.message' 2>/dev/null || echo $BODY)"
fi
echo ""

# Test DeepSeek
echo "🧪 Testing DeepSeek..."
echo "  URL: $DEEPSEEK_URL"
echo "  Model: deepseek-chat"
echo ""

RESPONSE=$(curl -s -w "\nHTTP_CODE:%{http_code}" \
  -X POST "$DEEPSEEK_URL/chat/completions" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $DEEPSEEK_KEY" \
  -d '{
    "model": "deepseek-chat",
    "messages": [{"role": "user", "content": "Hello"}],
    "max_tokens": 10
  }')

HTTP_CODE=$(echo "$RESPONSE" | grep "HTTP_CODE:" | cut -d: -f2)
BODY=$(echo "$RESPONSE" | sed '/HTTP_CODE:/d')

if [ "$HTTP_CODE" = "200" ]; then
    echo "  ✅ Success! HTTP $HTTP_CODE"
    echo "  Response: $(echo $BODY | jq -r '.choices[0].message.content' 2>/dev/null || echo $BODY)"
else
    echo "  ❌ Failed! HTTP $HTTP_CODE"
    echo "  Error: $(echo $BODY | jq -r '.error.message' 2>/dev/null || echo $BODY)"
fi
echo ""

# Test Moonshot
echo "🧪 Testing Moonshot..."
echo "  URL: $MOONSHOT_URL"
echo "  Model: moonshot-v1-8k"
echo ""

RESPONSE=$(curl -s -w "\nHTTP_CODE:%{http_code}" \
  -X POST "$MOONSHOT_URL/chat/completions" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $MOONSHOT_KEY" \
  -d '{
    "model": "moonshot-v1-8k",
    "messages": [{"role": "user", "content": "Hello"}],
    "max_tokens": 10
  }')

HTTP_CODE=$(echo "$RESPONSE" | grep "HTTP_CODE:" | cut -d: -f2)
BODY=$(echo "$RESPONSE" | sed '/HTTP_CODE:/d')

if [ "$HTTP_CODE" = "200" ]; then
    echo "  ✅ Success! HTTP $HTTP_CODE"
    echo "  Response: $(echo $BODY | jq -r '.choices[0].message.content' 2>/dev/null || echo $BODY)"
else
    echo "  ❌ Failed! HTTP $HTTP_CODE"
    echo "  Error: $(echo $BODY | jq -r '.error.message' 2>/dev/null || echo $BODY)"
fi
echo ""

echo "======================"
echo "📊 Summary"
echo "======================"
echo ""
echo "💡 Recommendations:"
echo ""
echo "1. If Zhipu failed with authentication error:"
echo "   - Check if API key is correct"
echo "   - Verify API key format (should be: xxxxx.xxxxxx)"
echo "   - Check if API key has expired"
echo "   - Visit: https://open.bigmodel.cn/"
echo ""
echo "2. If DeepSeek or Moonshot work:"
echo "   - Use them as alternative providers"
echo "   - Update config.toml to use working provider"
echo ""
echo "3. To update keys.yaml:"
echo "   - Get new API key from provider console"
echo "   - Update the api_key field"
echo "   - Run this test again"
echo ""

