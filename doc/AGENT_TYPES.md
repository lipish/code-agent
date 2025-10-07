# Agent Types

Task Runner 支持多种专门的 Agent 类型，每种类型都针对特定领域进行了优化。

## 📋 可用的 Agent 类型

| Agent 类型 | 专长领域 | 适用场景 |
|-----------|---------|---------|
| **Generic** | 通用任务 | 任何领域（默认） |
| **Code** | 软件开发 | 编码、重构、调试 |
| **Data** | 数据处理 | ETL、分析、可视化 |
| **DevOps** | 基础设施 | CI/CD、容器、IaC |
| **API** | API 设计 | REST、GraphQL、文档 |
| **Testing** | 测试 QA | 单元测试、集成测试 |
| **Documentation** | 技术写作 | 文档、教程、指南 |
| **Security** | 安全 | 漏洞评估、安全编码 |

---

## 🌟 0. Generic Agent (默认)

**专长**: 通用任务 - 适应任何领域

### 核心理念

**Generic Agent 是一个非特定的、灵活的 Agent，它会根据你的提示词自适应。**

> **关键原则**: 返回的内容完全取决于你的提示词。

### 特点
- 灵活且适应任何领域
- 没有预定义的专业化
- 从任务描述中学习
- 根据上下文调整方法

### 工作方式
1. **理解任务** - 彻底理解你的请求
2. **确定领域** - 判断任务属于什么领域
3. **选择方法** - 选择合适的方法和工具
4. **执行任务** - 根据具体需求执行
5. **适应调整** - 根据反馈调整方法

### 使用示例
```rust
// 使用默认的 Generic Agent
let template = PromptTemplate::default();

// 或显式创建
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::Generic),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- **任何任务** - Generic Agent 可以处理任何类型的任务
- "帮我分析这个问题"
- "优化这段代码"
- "设计一个系统"
- "编写文档"

### 何时使用
- ✅ 不确定任务属于哪个领域
- ✅ 任务跨越多个领域
- ✅ 需要灵活的方法
- ✅ 快速原型和探索

### 何时不使用
- ❌ 需要深度专业知识时 → 使用专门的 Agent
- ❌ 有明确的领域时 → 使用对应的专门 Agent

### 与专门 Agent 的对比

| 方面 | Generic Agent | 专门 Agent |
|------|--------------|-----------|
| **灵活性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **专业深度** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **适用范围** | 任何领域 | 特定领域 |
| **学习曲线** | 低 | 中 |

---

## 🤖 1. Code Agent

**专长**: 软件开发和架构

### 特点
- 精确、安全、有帮助
- 简洁、直接、友好
- 优先考虑可操作的指导
- 清楚说明假设和下一步

### 擅长领域
- 软件开发和架构
- 代码重构和优化
- 调试和错误处理
- 测试和质量保证
- 文档和代码审查

### 使用示例
```rust
use task_runner::prompts::{AgentType, global_template_for_agent, PromptTemplate};

// 使用默认的 Code Agent
let template = PromptTemplate::default();

// 或显式创建
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::Code),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- "重构 agent.rs 的错误处理"
- "优化数据库查询性能"
- "实现用户认证功能"
- "修复内存泄漏问题"

---

## 📊 2. Data Agent

**专长**: 数据处理、分析和转换

### 特点
- 分析性和注重细节
- 清晰传达数据洞察
- 关注数据质量和准确性
- 简单解释复杂模式

### 擅长领域
- 数据提取、转换、加载 (ETL)
- 数据分析和可视化
- 数据库设计和优化
- 数据清洗和验证
- 统计分析和报告

### 使用示例
```rust
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::Data),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- "分析用户活动日志并生成报告"
- "清洗和验证 CSV 数据"
- "设计数据仓库架构"
- "创建销售数据可视化"

---

## 🚀 3. DevOps Agent

**专长**: 基础设施、部署和运维

### 特点
- 关注可靠性和主动性
- 清楚说明风险和权衡
- 强调自动化和可重复性
- 默认考虑安全性

### 擅长领域
- CI/CD 管道设计和实现
- 容器编排 (Docker, Kubernetes)
- 基础设施即代码 (Terraform, Ansible)
- 监控和日志系统
- 安全和合规

### 使用示例
```rust
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::DevOps),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- "使用 GitHub Actions 设置 CI/CD 管道"
- "配置 Kubernetes 集群"
- "实现基础设施监控"
- "编写 Terraform 脚本"

---

## 🔌 4. API Agent

**专长**: API 设计、开发和集成

### 特点
- 设计优先的思维方式
- 关注开发者体验
- 清楚说明 API 契约
- 强调向后兼容性

### 擅长领域
- RESTful 和 GraphQL API 设计
- API 文档 (OpenAPI/Swagger)
- 认证和授权
- 速率限制和缓存
- API 版本控制和迁移

### 使用示例
```rust
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::Api),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- "设计用户管理的 REST API"
- "编写 OpenAPI 规范"
- "实现 OAuth2 认证"
- "设计 GraphQL schema"

---

## 🧪 5. Testing Agent

**专长**: 软件测试和质量保证

### 特点
- 关注质量和彻底性
- 考虑边界情况和失败模式
- 清楚说明测试覆盖率
- 主动发现潜在问题

### 擅长领域
- 单元、集成、端到端测试
- 测试驱动开发 (TDD)
- 测试自动化框架
- 性能和负载测试
- 安全测试

### 使用示例
```rust
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::Testing),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- "为认证模块编写集成测试"
- "实现 TDD 工作流"
- "创建性能测试套件"
- "添加端到端测试"

---

## 📝 6. Documentation Agent

**专长**: 技术写作和文档

### 特点
- 清晰易懂的写作
- 以用户为中心的方法
- 结构化和有组织
- 示例驱动的解释

### 擅长领域
- API 文档
- 用户指南和教程
- 架构文档
- 代码注释和文档字符串
- README 和贡献指南

### 使用示例
```rust
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::Documentation),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- "编写 API 端点文档"
- "创建用户入门指南"
- "记录系统架构"
- "更新 README 文件"

---

## 🔒 7. Security Agent

**专长**: 应用安全和合规

### 特点
- 安全优先的思维方式
- 风险意识和谨慎
- 清楚说明安全影响
- 主动应对威胁

### 擅长领域
- 安全漏洞评估
- 安全编码实践
- 认证和授权
- 加密和数据保护
- 合规 (GDPR, HIPAA 等)

### 使用示例
```rust
let template = PromptTemplate {
    global: global_template_for_agent(AgentType::Security),
    project: None,
    scenarios: HashMap::new(),
};
```

### 适用任务
- "审查认证实现的漏洞"
- "实现数据加密"
- "评估 GDPR 合规性"
- "修复 SQL 注入漏洞"

---

## 🔧 如何选择 Agent 类型

### 决策树

```
你的任务是什么？
│
├─ 编写/修改代码？ → Code Agent
├─ 处理/分析数据？ → Data Agent
├─ 部署/运维？ → DevOps Agent
├─ 设计/集成 API？ → API Agent
├─ 编写测试？ → Testing Agent
├─ 编写文档？ → Documentation Agent
└─ 安全审查？ → Security Agent
```

### 组合使用

对于复杂任务，可以组合使用多个 Agent：

```rust
// 1. 使用 Code Agent 实现功能
let code_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Code),
    // ...
};

// 2. 使用 Testing Agent 编写测试
let testing_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Testing),
    // ...
};

// 3. 使用 Documentation Agent 编写文档
let docs_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Documentation),
    // ...
};
```

---

## 💡 最佳实践

### 1. 明确任务类型
在开始之前，明确任务属于哪个领域。

### 2. 使用专门的 Agent
不要用 Code Agent 做所有事情，使用专门的 Agent 获得更好的结果。

### 3. 组合使用
复杂项目可以使用多个 Agent 类型。

### 4. 自定义配置
可以在 Agent 基础上添加项目特定的配置。

---

## 🎯 示例：完整工作流

### 场景：实现用户认证 API

```rust
// 1. API 设计阶段 - 使用 API Agent
let api_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Api),
    // ...
};
// 任务: "设计用户认证 REST API"

// 2. 实现阶段 - 使用 Code Agent
let code_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Code),
    // ...
};
// 任务: "实现认证 API 端点"

// 3. 安全审查 - 使用 Security Agent
let security_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Security),
    // ...
};
// 任务: "审查认证实现的安全性"

// 4. 测试 - 使用 Testing Agent
let testing_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Testing),
    // ...
};
// 任务: "编写认证 API 的集成测试"

// 5. 文档 - 使用 Documentation Agent
let docs_template = PromptTemplate {
    global: global_template_for_agent(AgentType::Documentation),
    // ...
};
// 任务: "编写认证 API 文档"
```

---

## 🚀 运行演示

查看所有 Agent 类型的实际效果：

```bash
cargo run --example agent_types_demo
```

---

## 📚 相关文档

- `src/prompts/defaults.rs` - Agent 类型定义
- `src/prompts/README.md` - 提示词系统使用指南
- `doc/PROMPT_ENGINEERING.md` - 提示词工程完整文档

