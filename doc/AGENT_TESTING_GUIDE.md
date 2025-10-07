# Agent 测试指南

## 概述

本文档说明如何测试 Agent 的性能和质量，使用智谱 AI (Zhipu AI) 作为 LLM 提供商。

---

## 🧪 测试工具

### 1. 简单测试脚本 (推荐)

**文件**: `test_agent_simple.sh`

**特点**:
- ✅ 即用即测
- ✅ 使用 CLI 接口
- ✅ 生成 Markdown 报告
- ✅ 自动记录时间和结果

**使用方法**:
```bash
# 运行测试
./test_agent_simple.sh

# 查看报告
cat test_reports/agent_test_*.md
```

---

### 2. Rust 测试程序 (开发中)

**文件**: `examples/agent_test_zhipu.rs`

**特点**:
- 🚧 更详细的测试
- 🚧 JSON 格式报告
- 🚧 性能分析
- 🚧 需要 API 修复

**状态**: WIP - 需要修复 API 调用

---

## 📋 测试用例

### Test Case 1: 简单代码任务

**任务**: "Write a Rust function to calculate fibonacci numbers"

**测试目标**:
- 代码生成能力
- 语法正确性
- 代码质量

**预期结果**:
- 提供可工作的 Rust 函数
- 包含注释和说明
- 代码符合 Rust 惯例

---

### Test Case 2: 简单问题

**任务**: "What is Rust programming language?"

**测试目标**:
- 通用知识
- 解释能力
- 回答清晰度

**预期结果**:
- 准确的定义
- 清晰的解释
- 相关的特性说明

---

### Test Case 3: 数据分析

**任务**: "Analyze the performance metrics: CPU 80%, Memory 60%, Disk 40%"

**测试目标**:
- 分析能力
- 推理能力
- 建议质量

**预期结果**:
- 识别问题
- 提供分析
- 给出建议

---

## 📊 测试报告格式

### Markdown 报告

```markdown
# Agent Test Report

**Date**: 2025-10-07
**Provider**: Zhipu AI
**Model**: glm-4.6

---

## Test Cases

### Test Case 1: Simple Code Task ✅

**Task**: Write a Rust function to calculate fibonacci numbers
**Duration**: 5s
**Status**: Success

**Result**:
\`\`\`
[Agent response here]
\`\`\`

---

## Summary

**Total Tests**: 3
**Passed**: 2
**Failed**: 1
**Success Rate**: 66.7%

## Analysis

### Observations
1. Response quality is good
2. Performance is acceptable
3. Error handling needs improvement

### Optimization Suggestions
1. Improve prompt engineering
2. Adjust model parameters
3. Add retry logic
```

---

## 🔍 评估标准

### 1. 响应质量

**评分标准**:
- ⭐⭐⭐⭐⭐ 优秀: 准确、详细、有帮助
- ⭐⭐⭐⭐ 良好: 准确、基本完整
- ⭐⭐⭐ 一般: 基本准确，缺少细节
- ⭐⭐ 较差: 部分错误或不完整
- ⭐ 很差: 错误或无用

**检查点**:
- [ ] 回答准确性
- [ ] 内容完整性
- [ ] 解释清晰度
- [ ] 实用性

---

### 2. 性能指标

**关键指标**:
- **响应时间**: < 10s 优秀, < 30s 良好, > 30s 需优化
- **Token 使用**: 监控成本
- **成功率**: > 90% 优秀, > 70% 良好, < 70% 需改进

**监控**:
- [ ] 平均响应时间
- [ ] 最大响应时间
- [ ] Token 消耗
- [ ] 错误率

---

### 3. 错误处理

**评估点**:
- [ ] 错误消息清晰
- [ ] 提供恢复建议
- [ ] 日志记录完整
- [ ] 用户友好

---

## 💡 优化建议

### 1. 提示词工程

**当前问题**:
- 提示词可能过于简单
- 缺少上下文信息
- 没有明确的输出格式要求

**优化方案**:
```rust
// 之前
"Write a Rust function to calculate fibonacci numbers"

// 之后
"Write a Rust function to calculate fibonacci numbers. 
Requirements:
- Use iterative approach for efficiency
- Include error handling
- Add documentation comments
- Provide usage example"
```

---

### 2. 模型参数调整

**Temperature**:
- 当前: 0.7
- 代码生成: 0.3-0.5 (更确定性)
- 创意任务: 0.7-0.9 (更多样性)

**Max Tokens**:
- 当前: 2000
- 简单任务: 500-1000
- 复杂任务: 2000-4000

**建议**:
```rust
// 根据任务类型调整
let temperature = match task_type {
    TaskType::CodeGeneration => 0.3,
    TaskType::Analysis => 0.5,
    TaskType::Creative => 0.8,
};
```

---

### 3. 错误处理改进

**当前问题**:
- 错误消息不够详细
- 缺少重试机制
- 没有降级策略

**优化方案**:
```rust
// 添加重试逻辑
let mut retries = 3;
while retries > 0 {
    match agent.process_task(task).await {
        Ok(result) => return Ok(result),
        Err(e) if e.is_retryable() => {
            retries -= 1;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        Err(e) => return Err(e),
    }
}
```

---

### 4. 缓存策略

**建议**:
- 缓存常见问题的回答
- 使用语义相似度匹配
- 设置合理的过期时间

**实现**:
```rust
// 伪代码
if let Some(cached) = cache.get_similar(task, threshold=0.9) {
    return Ok(cached);
}

let result = agent.process_task(task).await?;
cache.set(task, result.clone());
Ok(result)
```

---

### 5. 监控和日志

**需要记录**:
- 每次请求的详细信息
- 响应时间和 Token 使用
- 错误和异常
- 用户反馈

**实现**:
```rust
// 结构化日志
tracing::info!(
    task = %task,
    duration_ms = duration.as_millis(),
    tokens = response.tokens_used,
    success = response.success,
    "Task completed"
);
```

---

## 🚀 下一步行动

### 立即执行

1. **运行测试**
   ```bash
   ./test_agent_simple.sh
   ```

2. **查看报告**
   ```bash
   cat test_reports/agent_test_*.md
   ```

3. **分析结果**
   - 检查成功率
   - 评估响应质量
   - 识别问题

---

### 短期改进

1. **修复 agent_test_zhipu.rs**
   - 修复 API 调用
   - 添加更多测试用例
   - 生成详细报告

2. **优化提示词**
   - 根据测试结果调整
   - 添加更多上下文
   - 明确输出格式

3. **调整参数**
   - 根据任务类型调整 temperature
   - 优化 max_tokens
   - 测试不同配置

---

### 长期规划

1. **自动化测试**
   - CI/CD 集成
   - 定期回归测试
   - 性能基准测试

2. **A/B 测试**
   - 测试不同提示词
   - 比较不同模型
   - 优化参数组合

3. **用户反馈**
   - 收集真实使用数据
   - 分析失败案例
   - 持续改进

---

## 📚 参考资料

- `keys.yaml` - API 配置
- `config.toml` - Agent 配置
- `doc/PROMPT_ENGINEERING.md` - 提示词工程
- `doc/AGENT_TYPES.md` - Agent 类型说明

---

## 🎯 成功标准

### 最低标准
- ✅ 成功率 > 70%
- ✅ 平均响应时间 < 30s
- ✅ 错误消息清晰

### 优秀标准
- ⭐ 成功率 > 90%
- ⭐ 平均响应时间 < 10s
- ⭐ 响应质量高
- ⭐ 用户满意度高

---

*最后更新: 2025-10-07*

