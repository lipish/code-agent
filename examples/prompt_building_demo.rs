//! Prompt Building Demo
//!
//! This example demonstrates how PromptBuilder constructs the full prompt
//! that is sent to the LLM, showing how the LLM knows about the output format.

use task_runner::prompts::{PromptBuilder, PromptTemplate};

fn main() {
    println!("=== Prompt Building Demo ===\n");
    println!("This shows how the LLM learns about the output format.\n");

    // Create a default template
    let template = PromptTemplate::default();

    // Build a prompt for a simple task
    let builder = PromptBuilder::new(template);
    let prompt = builder.build("重构 agent.rs 的错误处理");

    println!("📝 Full Prompt Sent to LLM:");
    println!("{}", "=".repeat(80));
    println!("{}", prompt);
    println!("{}", "=".repeat(80));

    println!("\n\n🔍 Key Sections Breakdown:\n");

    // Extract and highlight key sections
    highlight_section(&prompt, "# System Role", "This tells the LLM its personality and behavior");
    highlight_section(&prompt, "# Output Format", "This tells the LLM what fields to include");
    highlight_section(&prompt, "**Required Fields**:", "This defines each field and its purpose");
    highlight_section(&prompt, "# Constraints", "This guides the LLM's behavior");

    println!("\n\n💡 How LLM Understands:\n");
    println!("1. LLM reads the '# Output Format' section");
    println!("2. LLM sees the required fields: UNDERSTANDING, APPROACH, PLAN, EXECUTION");
    println!("3. LLM reads the description of each field");
    println!("4. LLM generates output matching this structure");

    println!("\n\n🎯 Example LLM Output:\n");
    println!("{}", "-".repeat(80));
    print_example_output();
    println!("{}", "-".repeat(80));

    println!("\n\n✅ The LLM knows the format because:");
    println!("   - It's explicitly told in the prompt");
    println!("   - Each field has a clear description");
    println!("   - The format is part of the system instructions");
}

fn highlight_section(prompt: &str, marker: &str, explanation: &str) {
    if let Some(pos) = prompt.find(marker) {
        let section_start = pos;
        let section_end = prompt[section_start..]
            .find("\n\n")
            .map(|p| section_start + p)
            .unwrap_or(prompt.len());
        
        let section = &prompt[section_start..section_end.min(section_start + 200)];
        
        println!("📌 {}", marker);
        println!("   {}", explanation);
        println!("   Preview: {}...\n", section.lines().next().unwrap_or(""));
    }
}

fn print_example_output() {
    println!(r#"
UNDERSTANDING:
需要改进 agent.rs 中的错误处理机制，使用 Result 类型替代 unwrap()，
并添加更详细的错误信息。

APPROACH:
1. 识别所有使用 unwrap() 和 expect() 的地方
2. 创建自定义错误类型 AgentError
3. 使用 ? 操作符传播错误，提供清晰的错误上下文

PLAN:
Phase 1: 准备工作
- 创建 src/errors.rs 定义 AgentError
- 添加 thiserror 依赖到 Cargo.toml

Phase 2: 重构核心模块
- 修改 agent.rs 中的函数签名返回 Result
- 替换所有 unwrap() 为 ? 操作符

Phase 3: 验证
- 运行 cargo build 确保编译通过
- 运行 cargo test 确保测试通过

EXECUTION:
1. 创建错误类型定义
   文件: src/errors.rs
   代码:
   ```rust
   use thiserror::Error;
   
   #[derive(Error, Debug)]
   pub enum AgentError {{
       #[error("Task execution failed: {{0}}")]
       ExecutionError(String),
   }}
   ```

2. 添加依赖
   命令: cargo add thiserror

3. 验证
   命令: cargo build && cargo test
"#);
}

