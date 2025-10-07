# 重构指南

## 已完成的重构

### 1. 创建 Understanding 模块

**文件**: `src/understanding.rs`

**目的**: 将任务理解逻辑从 `agent.rs` 中分离出来，实现单一职责原则。

**核心结构**:
```rust
pub struct PlanningEngine {
    model: Arc<dyn LanguageModel>,
}

impl PlanningEngine {
    // 创建新的理解引擎
    pub fn new(model: Arc<dyn LanguageModel>) -> Self
    
    // 分析任务并创建执行计划
    pub async fn understand_task(&self, request: &str) -> Result<TaskPlan, AgentError>
    
    // 构建理解提示词（私有）
    fn build_understanding_prompt(&self, request: &str) -> String
    
    // 解析 AI 响应为结构化计划（私有）
    fn parse_task_plan(&self, response: &str) -> Result<TaskPlan, AgentError>
}
```

**使用示例**:
```rust
use task_runner::planning::PlanningEngine;
use std::sync::Arc;

let model = Arc::new(create_your_model());
let engine = PlanningEngine::new(model);
let plan = engine.understand_task("读取 README.md 文件").await?;

println!("理解: {}", plan.understanding);
println!("方法: {}", plan.approach);
println!("复杂度: {:?}", plan.complexity);
```

### 2. 重构 CodeAgent

**变更**: 从 `Box<dyn LanguageModel>` 改为 `Arc<dyn LanguageModel>`

**原因**:
- 支持多个组件共享同一模型实例
- `PlanningEngine` 和 `CodeAgent` 可以共享模型
- 更好的内存效率

**迁移指南**:

**之前**:
```rust
let model = Box::new(ZhipuModel::new(config)?);
let agent = CodeAgent::new(model, config);
```

**之后**:
```rust
// 仍然使用 Box 创建，内部会自动转换为 Arc
let model = Box::new(ZhipuModel::new(config)?);
let agent = CodeAgent::new(model, config);

// 或者直接使用 Arc
let model = Arc::new(ZhipuModel::new(config)?);
let agent = CodeAgent::new(model.into(), config);
```

**API 变更**:
```rust
// 之前
pub fn get_model(&self) -> &Box<dyn LanguageModel>

// 之后
pub fn get_model(&self) -> &Arc<dyn LanguageModel>
```

## 架构改进

### 模块职责划分

```
┌─────────────────────────────────────────────────────────┐
│                    task_runner                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────────┐      ┌──────────────────────┐       │
│  │   agent.rs   │─────▶│ understanding.rs     │       │
│  │              │      │                      │       │
│  │ - 任务管理    │      │ - 提示词构建          │       │
│  │ - 执行协调    │      │ - AI 模型调用        │       │
│  │ - 工具调用    │      │ - 响应解析           │       │
│  └──────────────┘      └──────────────────────┘       │
│         │                                              │
│         │                                              │
│         ▼                                              │
│  ┌──────────────┐      ┌──────────────────────┐       │
│  │ execution.rs │      │     tools.rs         │       │
│  │              │      │                      │       │
│  │ - 执行引擎    │      │ - 工具注册           │       │
│  │ - 步骤管理    │      │ - 工具执行           │       │
│  └──────────────┘      └──────────────────────┘       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 数据流

```
用户请求
   │
   ▼
CodeAgent::process_task()
   │
   ├─▶ PlanningEngine::understand_task()
   │      │
   │      ├─▶ build_understanding_prompt()
   │      ├─▶ model.complete()
   │      └─▶ parse_task_plan()
   │      
   │   返回 TaskPlan
   │
   ├─▶ execute_task_real()
   │      │
   │      └─▶ 使用工具执行任务
   │
   └─▶ 返回 TaskResult
```

## 代码质量改进

### 1. 使用 matches! 宏

**之前**:
```rust
fn should_retry(&self, error: &AgentError) -> bool {
    match error {
        AgentError::NetworkError(_) => true,
        AgentError::TimeoutError => true,
        AgentError::ModelError(ModelError::RateLimited) => true,
        _ => false,
    }
}
```

**之后**:
```rust
fn should_retry(&self, error: &AgentError) -> bool {
    matches!(
        error,
        AgentError::NetworkError(_) 
        | AgentError::TimeoutError 
        | AgentError::ModelError(ModelError::RateLimited)
    )
}
```

### 2. 简化错误转换

**之前**:
```rust
.map_err(|e| AgentError::ModelError(e))?
```

**之后**:
```rust
.map_err(AgentError::ModelError)?
```

### 3. 安全的字符串解析

**之前**:
```rust
// 可能越界
understanding = line[13..].trim().to_string();
```

**之后**:
```rust
// 安全的前缀剥离
understanding = line
    .strip_prefix("UNDERSTANDING:")
    .or_else(|| line.strip_prefix("understanding:"))
    .unwrap_or("")
    .trim()
    .to_string();
```

### 4. 实现 Default trait

**之前**:
```rust
impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
}
```

**之后**:
```rust
impl Default for ToolRegistry {
    fn default() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}
```

## 测试改进

### 新增测试

**understanding.rs 测试**:
```rust
#[tokio::test]
async fn test_understanding_engine_creation() {
    let model = Arc::new(MockModel::new("test".to_string()));
    let _engine = PlanningEngine::new(model);
}

#[tokio::test]
async fn test_parse_task_plan() {
    let model = Arc::new(MockModel::new("test".to_string()));
    let engine = PlanningEngine::new(model);
    
    let response = "UNDERSTANDING: Read a file\n...";
    let plan = engine.parse_task_plan(response).unwrap();
    
    assert_eq!(plan.understanding, "Read a file");
}

#[tokio::test]
async fn test_parse_task_plan_with_requirements() {
    // 测试带依赖项的解析
}
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test planning::tests

# 运行特定测试
cargo test test_parse_task_plan

# 显示测试输出
cargo test -- --nocapture
```

## 验证清单

在提交代码前，请确保：

- [ ] `cargo build` 成功
- [ ] `cargo test` 所有测试通过
- [ ] `cargo clippy` 无严重警告
- [ ] `cargo fmt` 代码格式化
- [ ] 更新相关文档
- [ ] 添加必要的测试用例

## 常见问题

### Q: 为什么使用 Arc 而不是 Box？

**A**: `Arc` 允许多个所有者共享同一数据，这对于以下场景很重要：
- `CodeAgent` 和 `PlanningEngine` 共享模型
- 未来可能的并发执行场景
- 更灵活的架构设计

### Q: 如何添加新的理解策略？

**A**: 在 `PlanningEngine` 中添加新方法：
```rust
impl PlanningEngine {
    pub async fn understand_with_context(
        &self, 
        request: &str, 
        context: &TaskContext
    ) -> Result<TaskPlan, AgentError> {
        // 实现带上下文的理解
    }
}
```

### Q: 如何扩展任务计划解析？

**A**: 修改 `parse_task_plan` 方法，添加新的字段解析逻辑：
```rust
fn parse_task_plan(&self, response: &str) -> Result<TaskPlan, AgentError> {
    // 现有解析逻辑
    
    // 添加新字段
    if line.to_uppercase().starts_with("PRIORITY:") {
        // 解析优先级
    }
    
    // ...
}
```

## 下一步重构建议

### 1. 重构 ExecutionEngine

**目标**: 实现真正的 AI 驱动执行循环

**步骤**:
1. 创建 `ExecutionContext` 结构
2. 实现决策循环
3. 添加工具调用管理
4. 实现步骤记录

### 2. 增强工具系统

**目标**: 添加更多实用工具

**建议工具**:
- Git 操作工具
- 代码搜索工具
- 测试运行工具
- 文档生成工具

### 3. 改进错误处理

**目标**: 实现智能重试和降级

**步骤**:
1. 使用 `ErrorHandler`（目前未使用）
2. 实现错误分类
3. 添加重试策略
4. 实现降级机制

## 参考资源

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Tokio Best Practices](https://tokio.rs/tokio/tutorial)
- [Error Handling in Rust](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

