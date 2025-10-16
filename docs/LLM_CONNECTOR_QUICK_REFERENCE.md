# llm-connector Quick Reference

## 版本信息

**当前版本**: 0.3.8  
**GitHub**: https://github.com/lipish/llm-connector  
**特性**: `streaming`

## 支持的协议

| 协议 | 构造函数 | 需要 API Key | 模型发现 | 流式支持 |
|------|----------|--------------|----------|----------|
| OpenAI | `openai(key, endpoint)` | ✅ | ✅ | ✅ |
| Anthropic | `anthropic(key)` | ✅ | 有限 | ✅ |
| Zhipu | `zhipu(key)` | ✅ | ✅ | ✅ |
| Aliyun | `aliyun(key)` | ✅ | ❌ | ✅ |
| Ollama | `ollama(endpoint)` | ❌ | ✅ | ✅ |

## 快速开始

### 基本用法

```rust
use llm_connector::{LlmClient, ChatRequest, Message};

// 1. 创建客户端
let client = LlmClient::openai("sk-...", None);

// 2. 构建请求
let request = ChatRequest {
    model: "gpt-4".to_string(),
    messages: vec![Message::user("Hello!")],
    ..Default::default()
};

// 3. 发送请求
let response = client.chat(&request).await?;
println!("{}", response.choices[0].message.content);
```

## 各协议示例

### OpenAI

```rust
// 默认端点
let client = LlmClient::openai("sk-...", None);

// 自定义端点
let client = LlmClient::openai("sk-...", Some("https://api.example.com/v1"));

let request = ChatRequest {
    model: "gpt-4".to_string(),
    messages: vec![
        Message::system("You are helpful."),
        Message::user("Hello!"),
    ],
    temperature: Some(0.7),
    max_tokens: Some(100),
    ..Default::default()
};
```

### Anthropic (Claude)

```rust
let client = LlmClient::anthropic("sk-ant-...");

let request = ChatRequest {
    model: "claude-3-5-sonnet-20241022".to_string(),
    messages: vec![Message::user("Hello!")],
    max_tokens: Some(200), // ⚠️ Anthropic 要求 max_tokens
    ..Default::default()
};
```

### Zhipu (智谱AI)

```rust
// 使用专用构造函数（0.3.8+）
let client = LlmClient::zhipu("sk-...");

let request = ChatRequest {
    model: "glm-4".to_string(),
    messages: vec![Message::user("你好！")],
    ..Default::default()
};
```

**可用模型**: glm-4, glm-4-flash, glm-4-air, glm-4-plus, glm-4x

### Aliyun (阿里云 DashScope)

```rust
// 使用专用构造函数（0.3.8+）
let client = LlmClient::aliyun("sk-...");

let request = ChatRequest {
    model: "qwen-max".to_string(),
    messages: vec![Message::user("你好！")],
    ..Default::default()
};
```

**可用模型**: qwen-turbo, qwen-plus, qwen-max

### Ollama (本地)

```rust
// 默认 localhost:11434
let client = LlmClient::ollama(None);

// 自定义地址
let client = LlmClient::ollama(Some("http://192.168.1.100:11434"));

let request = ChatRequest {
    model: "llama3.2".to_string(),
    messages: vec![Message::user("Hello!")],
    ..Default::default()
};
```

## OpenAI-Compatible 提供商

所有以下提供商使用 `openai()` 构造函数：

### DeepSeek

```rust
let client = LlmClient::openai("sk-...", Some("https://api.deepseek.com/v1"));

let request = ChatRequest {
    model: "deepseek-chat".to_string(),
    messages: vec![Message::user("Hello!")],
    ..Default::default()
};
```

### Moonshot (Kimi)

```rust
let client = LlmClient::openai("sk-...", Some("https://api.moonshot.cn/v1"));

let request = ChatRequest {
    model: "moonshot-v1-32k".to_string(),
    messages: vec![Message::user("你好！")],
    ..Default::default()
};
```

### LongCat

```rust
let client = LlmClient::openai("ak-...", Some("https://api.longcat.chat/openai"));

let request = ChatRequest {
    model: "LongCat-Flash-Chat".to_string(),
    messages: vec![Message::user("Hello!")],
    ..Default::default()
};
```

### VolcEngine (火山引擎)

```rust
let client = LlmClient::openai("sk-...", Some("https://ark.cn-beijing.volces.com/api/v3"));

let request = ChatRequest {
    model: "your-endpoint-id".to_string(),
    messages: vec![Message::user("你好！")],
    ..Default::default()
};
```

## 高级功能

### 模型发现

```rust
// 获取可用模型列表
let models = client.fetch_models().await?;
println!("Available models: {:?}", models);
```

**支持情况**:
- ✅ OpenAI, DeepSeek, Moonshot, LongCat
- ✅ Anthropic (有限)
- ✅ Zhipu
- ✅ Ollama
- ❌ Aliyun

### 流式响应

启用 `streaming` 特性：

```toml
llm-connector = { version = "0.3.8", features = ["streaming"] }
```

```rust
use futures_util::StreamExt;

let mut stream = client.chat_stream(&request).await?;

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    if let Some(content) = chunk.get_content() {
        print!("{}", content);
    }
}
```

### Ollama 模型管理

```rust
use llm_connector::ollama::OllamaModelOps;

// 列出模型
let models = client.list_models().await?;

// 拉取模型
client.pull_model("llama3.2").await?;

// 查看详情
let details = client.show_model("llama3.2").await?;

// 删除模型
client.delete_model("llama3.2").await?;
```

### 协议信息

```rust
let protocol = client.protocol_name();
println!("Using protocol: {}", protocol);
```

## agent-runner 集成

### 配置模型

```rust
use agent_runner::config::{ModelConfig, ModelProvider};

let config = ModelConfig {
    provider: ModelProvider::OpenAI,
    model_name: "gpt-4".to_string(),
    api_key: Some("sk-...".to_string()),
    endpoint: None,
    max_tokens: 4096,
    temperature: 0.7,
};
```

### 创建模型实例

```rust
use agent_runner::models::LlmModel;

let model = LlmModel::from_config(config)?;
```

### 使用模型

```rust
use agent_runner::models::LanguageModel;

// 基本完成
let response = model.complete("Hello!").await?;
println!("{}", response.content);

// 获取可用模型
let models = model.fetch_available_models().await?;

// 获取协议名称
let protocol = model.protocol_name();
```

## 环境变量

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# Anthropic
export ANTHROPIC_API_KEY="sk-ant-..."

# Zhipu
export ZHIPU_API_KEY="sk-..."

# Aliyun
export ALIYUN_API_KEY="sk-..."

# DeepSeek
export DEEPSEEK_API_KEY="sk-..."

# Moonshot
export MOONSHOT_API_KEY="sk-..."

# LongCat
export LONGCAT_API_KEY="ak-..."
```

## 常见问题

### Q: Anthropic 请求失败

A: Anthropic 要求 `max_tokens` 参数：

```rust
let request = ChatRequest {
    model: "claude-3-5-sonnet-20241022".to_string(),
    messages: vec![Message::user("Hello!")],
    max_tokens: Some(200), // ✅ 必须设置
    ..Default::default()
};
```

### Q: Ollama 连接失败

A: 确保 Ollama 服务正在运行：

```bash
ollama serve
```

### Q: 如何知道使用哪个协议？

A: 使用 `protocol_name()`:

```rust
println!("Protocol: {}", client.protocol_name());
```

### Q: Zhipu/Aliyun 应该用哪个构造函数？

A: 0.3.8+ 使用专用构造函数：

```rust
// ✅ 推荐（0.3.8+）
let client = LlmClient::zhipu(api_key);
let client = LlmClient::aliyun(api_key);

// ❌ 旧方式（仍然可用但不推荐）
let client = LlmClient::openai(api_key, Some(zhipu_endpoint));
```

## 测试示例

### 运行功能演示

```bash
cargo run --example llm_connector_features
```

### 测试 API Key

```bash
# 设置 API key
export OPENAI_API_KEY="sk-..."

# 运行测试
cargo test --lib models
```

## 版本升级

### 从 0.3.1 升级到 0.3.8

```toml
# 之前
llm-connector = "0.3.1"

# 现在
llm-connector = { version = "0.3.8", features = ["streaming"] }
```

**变更**:
- ✅ Zhipu: 新增 `zhipu()` 专用构造函数
- ✅ Aliyun: 新增 `aliyun()` 专用构造函数
- ✅ 新增: `fetch_models()` 模型发现
- ✅ 新增: Ollama 模型管理功能
- ✅ 改进: Anthropic 流式支持

**兼容性**: 向后兼容，现有代码无需修改

## 最佳实践

### 1. 缓存模型列表

```rust
// ✅ 缓存结果
let models = client.fetch_models().await?;
// 使用缓存的模型列表

// ❌ 避免重复调用
for _ in 0..10 {
    client.fetch_models().await?; // 浪费 API 调用
}
```

### 2. 错误处理

```rust
use llm_connector::error::LlmConnectorError;

match client.chat(&request).await {
    Ok(response) => {
        println!("{}", response.choices[0].message.content);
    }
    Err(e) => match e {
        LlmConnectorError::AuthenticationError(msg) => {
            eprintln!("Auth error: {}", msg);
        }
        LlmConnectorError::RateLimitError(msg) => {
            eprintln!("Rate limit: {}", msg);
        }
        _ => eprintln!("Error: {}", e),
    }
}
```

### 3. 流式响应

```rust
use futures_util::StreamExt;

let mut stream = client.chat_stream(&request).await?;
let mut full_response = String::new();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    if let Some(content) = chunk.get_content() {
        print!("{}", content);
        full_response.push_str(&content);
    }
}

println!("\n\nFull response: {}", full_response);
```

## 参考资源

- **GitHub**: https://github.com/lipish/llm-connector
- **升级文档**: [LLM_CONNECTOR_UPGRADE.md](LLM_CONNECTOR_UPGRADE.md)
- **示例代码**: `examples/llm_connector_features.rs`
