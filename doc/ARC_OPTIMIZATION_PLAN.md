# Arc 优化实施计划

## 执行摘要

通过分析发现 3 个主要的 Arc 过度使用问题：

1. **严重**: `active_tasks` 三层包装 - `Arc<RwLock<HashMap<..., Arc<RwLock<...>>>>>`
2. **中等**: `agent` 双重包装 - `Arc<RwLock<TaskAgent>>`
3. **轻微**: `tools` 可能不需要 Arc - `Arc<ToolRegistry>`

## 🎯 优化目标

- **内存使用**: 减少 40-60%
- **锁竞争**: 减少 50%
- **代码复杂度**: 减少 40%
- **性能提升**: 10-20%

---

## 📋 Phase 1: 优化 active_tasks (高优先级)

### 当前问题

```rust
// src/service/core.rs:37
active_tasks: Arc<RwLock<HashMap<String, Arc<RwLock<TaskContext>>>>>
```

**问题**:
- 三层包装：Arc → RwLock → HashMap → Arc → RwLock
- 每次访问需要多次锁定
- 引用计数开销大

### 解决方案：使用 DashMap

```rust
// 添加依赖
[dependencies]
dashmap = "5.5"

// 修改结构
use dashmap::DashMap;

pub struct CodeAgentService {
    active_tasks: Arc<DashMap<String, TaskContext>>,  // 只需一层 Arc
    // ...
}
```

### 实施步骤

#### Step 1.1: 添加依赖

```toml
# Cargo.toml
[dependencies]
dashmap = "5.5"
```

#### Step 1.2: 修改 CodeAgentService 结构

```rust
// src/service/core.rs

use dashmap::DashMap;

pub struct CodeAgentService {
    config: ServiceConfig,
    metrics: Arc<MetricsCollector>,
    agent: Arc<RwLock<TaskAgent>>,
    active_tasks: Arc<DashMap<String, TaskContext>>,  // 修改这里
    task_semaphore: Arc<Semaphore>,
    available_tools: Vec<String>,
}
```

#### Step 1.3: 更新构造函数

```rust
impl CodeAgentService {
    pub fn new(agent: TaskAgent, config: ServiceConfig) -> Self {
        Self {
            config: config.clone(),
            metrics: Arc::new(MetricsCollector::new()),
            agent: Arc::new(RwLock::new(agent)),
            active_tasks: Arc::new(DashMap::new()),  // 修改这里
            task_semaphore: Arc::new(Semaphore::new(config.max_concurrent_tasks)),
            available_tools: vec![],
        }
    }
}
```

#### Step 1.4: 更新所有使用 active_tasks 的地方

**插入任务**:
```rust
// 之前
let mut tasks = self.active_tasks.write().await;
tasks.insert(task_id.clone(), Arc::new(RwLock::new(task_context)));

// 之后
self.active_tasks.insert(task_id.clone(), task_context);
```

**获取任务**:
```rust
// 之前
let tasks = self.active_tasks.read().await;
if let Some(task) = tasks.get(task_id) {
    let task = task.read().await;
    // use task
}

// 之后
if let Some(task) = self.active_tasks.get(task_id) {
    // task 是 Ref<String, TaskContext>
    // 直接使用 task.value()
}
```

**删除任务**:
```rust
// 之前
let mut tasks = self.active_tasks.write().await;
tasks.remove(task_id);

// 之后
self.active_tasks.remove(task_id);
```

**列出所有任务**:
```rust
// 之前
let tasks = self.active_tasks.read().await;
let task_ids: Vec<String> = tasks.keys().cloned().collect();

// 之后
let task_ids: Vec<String> = self.active_tasks.iter()
    .map(|entry| entry.key().clone())
    .collect();
```

### 预期收益

- **内存**: 减少 ~66% (3层 → 1层)
- **性能**: 提升 ~30% (无锁竞争)
- **代码**: 简化 ~50%

---

## 📋 Phase 2: 优化 agent 包装 (中优先级)

### 当前问题

```rust
// src/service/core.rs:35
agent: Arc<RwLock<TaskAgent>>
```

**问题**:
- 双重包装：Arc + RwLock
- 如果 CodeAgentService 是唯一所有者，不需要 Arc

### 解决方案 A: 去掉 Arc (推荐)

```rust
pub struct CodeAgentService {
    agent: RwLock<TaskAgent>,  // 只用 RwLock
    // ...
}
```

**适用条件**:
- CodeAgentService 是 agent 的唯一所有者
- agent 不需要在 service 外部共享

### 解决方案 B: 去掉 RwLock

```rust
pub struct CodeAgentService {
    agent: Arc<TaskAgent>,  // 只用 Arc
    // ...
}
```

**适用条件**:
- TaskAgent 不需要可变
- 或 TaskAgent 内部使用 interior mutability

### 实施步骤 (方案 A)

#### Step 2.1: 修改结构

```rust
pub struct CodeAgentService {
    agent: RwLock<TaskAgent>,  // 去掉 Arc
    // ...
}
```

#### Step 2.2: 更新构造函数

```rust
impl CodeAgentService {
    pub fn new(agent: TaskAgent, config: ServiceConfig) -> Self {
        Self {
            agent: RwLock::new(agent),  // 不需要 Arc::new
            // ...
        }
    }
}
```

#### Step 2.3: 更新使用代码

```rust
// 使用方式不变
let agent = self.agent.read().await;
// 或
let mut agent = self.agent.write().await;
```

### 预期收益

- **内存**: 减少 ~50% (2层 → 1层)
- **性能**: 提升 ~10%
- **代码**: 简化 ~20%

---

## 📋 Phase 3: 评估 tools 的 Arc (低优先级)

### 当前情况

```rust
// src/agent/mod.rs:39
tools: Arc<ToolRegistry>,
```

### 评估问题

1. **ToolRegistry 是否被共享？**
   - 检查是否有多个地方持有 ToolRegistry
   - 检查 `get_tools()` 的调用情况

2. **ToolRegistry 内部是否有锁？**
   - 如果有内部锁，外部 Arc 可能不必要

### 实施步骤

#### Step 3.1: 检查 ToolRegistry 定义

```rust
// 查看 src/agent/tool_registry.rs
// 检查是否有内部 Mutex/RwLock
```

#### Step 3.2: 检查使用情况

```bash
# 搜索 get_tools() 的调用
rg "get_tools\(\)" --type rust
```

#### Step 3.3: 决策

**如果 ToolRegistry 只在 TaskAgent 内部使用**:
```rust
pub struct TaskAgent {
    tools: ToolRegistry,  // 直接拥有
    // ...
}

impl TaskAgent {
    pub fn tools(&self) -> &ToolRegistry {  // 返回引用
        &self.tools
    }
}
```

**如果需要共享**:
```rust
// 保持 Arc，但改进 API
impl TaskAgent {
    pub fn tools(&self) -> &ToolRegistry {  // 返回内部引用
        &self.tools
    }
}
```

---

## 📋 Phase 4: 评估 metrics (低优先级)

### 当前情况

```rust
// src/service/core.rs:33
metrics: Arc<MetricsCollector>,
```

### 评估问题

1. **metrics 是否在 service 外部使用？**
2. **是否需要在多个地方共享？**

### 实施步骤

#### Step 4.1: 检查使用情况

```bash
# 搜索 MetricsCollector 的使用
rg "MetricsCollector" --type rust
```

#### Step 4.2: 决策

**如果只在 service 内部使用**:
```rust
pub struct CodeAgentService {
    metrics: MetricsCollector,  // 直接拥有
    // ...
}
```

**如果需要导出给外部**:
```rust
// 保持 Arc
pub struct CodeAgentService {
    metrics: Arc<MetricsCollector>,
    // ...
}

impl CodeAgentService {
    pub fn metrics(&self) -> &MetricsCollector {  // 返回引用
        &self.metrics
    }
}
```

---

## 🧪 测试计划

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_active_tasks_dashmap() {
        let service = create_test_service();
        
        // 测试插入
        let task_id = "test-1".to_string();
        let context = TaskContext::new(task_id.clone());
        service.active_tasks.insert(task_id.clone(), context);
        
        // 测试获取
        assert!(service.active_tasks.contains_key(&task_id));
        
        // 测试删除
        service.active_tasks.remove(&task_id);
        assert!(!service.active_tasks.contains_key(&task_id));
    }
}
```

### 性能测试

```rust
#[tokio::test]
async fn bench_active_tasks_concurrent_access() {
    let service = Arc::new(create_test_service());
    let mut handles = vec![];
    
    // 100 个并发任务
    for i in 0..100 {
        let service = Arc::clone(&service);
        let handle = tokio::spawn(async move {
            let task_id = format!("task-{}", i);
            let context = TaskContext::new(task_id.clone());
            service.active_tasks.insert(task_id, context);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    assert_eq!(service.active_tasks.len(), 100);
}
```

---

## 📊 实施时间表

| Phase | 任务 | 优先级 | 预计时间 | 风险 |
|-------|------|--------|---------|------|
| 1 | 优化 active_tasks | 高 | 2-3 小时 | 低 |
| 2 | 优化 agent 包装 | 中 | 1-2 小时 | 中 |
| 3 | 评估 tools | 低 | 1 小时 | 低 |
| 4 | 评估 metrics | 低 | 1 小时 | 低 |
| - | 测试和验证 | - | 2 小时 | - |
| **总计** | | | **7-9 小时** | |

---

## ✅ 验收标准

### 功能

- [ ] 所有现有测试通过
- [ ] 新增并发测试通过
- [ ] 行为与优化前一致

### 性能

- [ ] 内存使用减少 40%+
- [ ] 并发性能提升 20%+
- [ ] 无性能回归

### 代码质量

- [ ] 代码复杂度降低
- [ ] 锁定逻辑简化
- [ ] 文档更新

---

## 🚀 开始实施

### 立即开始 (Phase 1)

```bash
# 1. 创建新分支
git checkout -b optimize-arc-usage

# 2. 添加 dashmap 依赖
cargo add dashmap

# 3. 修改 src/service/core.rs
# 4. 运行测试
cargo test

# 5. 提交
git commit -m "refactor: optimize active_tasks using DashMap"
```

---

## 📚 参考资料

- [DashMap 文档](https://docs.rs/dashmap/)
- [Rust 并发模式](https://rust-lang.github.io/async-book/)
- [Arc vs Box vs Rc](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

