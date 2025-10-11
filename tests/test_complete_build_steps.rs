//! 完整的Rust项目构建步骤演示
//! 包含从创建到构建验证的完整流程

use tokio;
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};

#[tokio::test]
async fn test_complete_rust_project_build_steps() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 完整的Rust项目构建步骤演示");
    println!("===============================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 2000,
        temperature: 0.4,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let task = "创建一个Rust项目，名字叫hello-world，并且完成构建";
    
    // 明确要求包含构建步骤的提示词
    let detailed_build_prompt = format!(
        "请为以下任务提供完整的执行步骤，特别要包含构建过程：

任务：{}

请严格按照以下格式返回，必须包含6个步骤：

**步骤1：** 环境准备
**步骤2：** 创建项目
**步骤3：** 进入项目目录
**步骤4：** 查看项目结构
**步骤5：** 构建项目（使用cargo build）
**步骤6：** 运行项目验证

每个步骤要求：
- 说明具体操作
- 提供具体命令
- 说明预期结果
- 特别是步骤5要详细说明如何使用cargo build构建项目
- 步骤6要说明如何验证构建成功并运行程序

用中文回答，格式要清晰。", task);
    
    println!("🎯 任务: {}", task);
    println!();
    println!("📤 请求包含构建步骤的详细流程...");
    
    match model.complete(&detailed_build_prompt).await {
        Ok(response) => {
            println!("✅ 获取成功！");
            println!();
            println!("📝 完整的构建流程:");
            println!("{}", "═".repeat(60));
            println!("{}", response.content);
            println!("{}", "═".repeat(60));
            
            // 检查是否包含关键的构建步骤
            analyze_build_steps(&response.content);
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 获取失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test] 
async fn test_build_verification_steps() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 构建验证步骤详解");
    println!("====================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1800,
        temperature: 0.3,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    // 专门询问构建相关的步骤
    let build_focused_prompt = "请详细说明在Rust项目中如何进行构建和验证：

1. 如何使用cargo build构建项目？
2. 如何使用cargo run编译并运行项目？
3. 如何检查构建是否成功？
4. 构建过程中可能出现什么问题？
5. 如何验证hello-world程序正确输出？

请用中文回答，提供具体的命令和预期输出。";
    
    println!("📤 询问构建验证的详细步骤...");
    
    match model.complete(build_focused_prompt).await {
        Ok(response) => {
            println!("✅ 获取成功！");
            println!();
            println!("🔧 构建验证详解:");
            println!("{}", "─".repeat(50));
            println!("{}", response.content);
            println!("{}", "─".repeat(50));
            
            // 提取构建命令
            extract_build_commands(&response.content);
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 获取失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

/// 分析构建步骤的完整性
fn analyze_build_steps(content: &str) {
    println!();
    println!("🔍 构建步骤完整性分析:");
    println!("{}", "─".repeat(30));
    
    let has_cargo_new = content.contains("cargo new") || content.contains("创建项目");
    let has_cd_command = content.contains("cd ") || content.contains("进入");
    let has_cargo_build = content.contains("cargo build") || content.contains("构建");
    let has_cargo_run = content.contains("cargo run") || content.contains("运行");
    let has_verification = content.contains("验证") || content.contains("检查") || content.contains("Hello, world!");
    
    println!("📌 项目创建 (cargo new): {}", if has_cargo_new { "✅ 包含" } else { "❌ 缺失" });
    println!("📌 目录切换 (cd): {}", if has_cd_command { "✅ 包含" } else { "❌ 缺失" });
    println!("📌 项目构建 (cargo build): {}", if has_cargo_build { "✅ 包含" } else { "❌ 缺失" });
    println!("📌 项目运行 (cargo run): {}", if has_cargo_run { "✅ 包含" } else { "❌ 缺失" });
    println!("📌 结果验证: {}", if has_verification { "✅ 包含" } else { "❌ 缺失" });
    
    let completeness_score = [has_cargo_new, has_cd_command, has_cargo_build, has_cargo_run, has_verification]
        .iter().filter(|&&x| x).count();
    
    println!();
    println!("📊 完整性评分: {}/5", completeness_score);
    
    if completeness_score == 5 {
        println!("🎉 步骤非常完整！");
    } else if completeness_score >= 3 {
        println!("⚠️ 步骤基本完整，但还可以改进");
    } else {
        println!("❌ 步骤不够完整，缺少关键环节");
    }
}

/// 提取构建相关的命令
fn extract_build_commands(content: &str) {
    println!();
    println!("🛠️ 提取的构建命令:");
    println!("{}", "─".repeat(25));
    
    let lines: Vec<&str> = content.lines().collect();
    let mut found_commands = Vec::new();
    
    for line in &lines {
        let line = line.trim();
        
        if line.starts_with("cargo ") || line.contains("`cargo ") {
            found_commands.push(line);
            println!("💻 {}", line.replace("`", ""));
        } else if line.contains("cd ") && !line.contains("说明") {
            found_commands.push(line);
            println!("📁 {}", line);
        }
    }
    
    if found_commands.is_empty() {
        println!("⚠️ 未找到明确的命令格式");
    } else {
        println!();
        println!("✅ 总共提取到 {} 个命令", found_commands.len());
    }
    
    // 检查是否包含关键构建命令
    let content_lower = content.to_lowercase();
    println!();
    println!("🔧 关键构建命令检查:");
    println!("  cargo build: {}", if content_lower.contains("cargo build") { "✅" } else { "❌" });
    println!("  cargo run: {}", if content_lower.contains("cargo run") { "✅" } else { "❌" });
    println!("  cargo check: {}", if content_lower.contains("cargo check") { "✅" } else { "⚪" });
}