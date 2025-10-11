//! 简单的步骤展示测试

use tokio;
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};

#[tokio::test]
async fn test_get_detailed_steps() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 获取任务的详细步骤");
    println!("====================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1500,
        temperature: 0.5,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let task = "创建一个Rust项目，名字叫hello-world";
    
    // 专门请求详细步骤的提示词
    let steps_prompt = format!(
        "请为以下任务提供详细的执行步骤：

任务：{}

请严格按照以下格式返回：

**步骤1：** [第一个具体步骤]
**步骤2：** [第二个具体步骤]  
**步骤3：** [第三个具体步骤]
**步骤4：** [第四个具体步骤]
**步骤5：** [第五个具体步骤]
**步骤6：** [第六个具体步骤]

要求：
- 每个步骤都要具体可执行
- 包含具体的命令（如果需要）
- 说明每步的预期结果
- 步骤5必须说明如何构建项目（使用cargo build）
- 步骤6说明如何验证构建结果
- 用中文回答", task);
    
    println!("🎯 任务: {}", task);
    println!();
    println!("📤 请求详细步骤...");
    
    match model.complete(&steps_prompt).await {
        Ok(response) => {
            println!("✅ 获取成功！");
            println!();
            println!("📝 LLM返回的详细步骤:");
            println!("{}", "═".repeat(50));
            println!("{}", response.content);
            println!("{}", "═".repeat(50));
            
            // 解析和格式化步骤
            println!();
            println!("🔍 解析后的步骤列表:");
            println!("{}", "─".repeat(30));
            
            parse_steps(&response.content);
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 获取失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

fn parse_steps(content: &str) {
    for line in content.lines() {
        let line = line.trim();
        
        // 查找步骤行
        if line.contains("步骤") && (line.contains("：") || line.contains(":")) {
            println!("📌 {}", line);
        } else if line.starts_with(char::is_numeric) && line.contains(".") {
            println!("📌 {}", line);
        } else if line.starts_with("**步骤") {
            println!("📌 {}", line.replace("**", ""));
        } else if !line.is_empty() && !line.contains("要求") && !line.contains("任务") && line.len() > 5 {
            // 可能是步骤描述
            if line.contains("cargo") || line.contains("cd") || line.contains("运行") || 
               line.contains("创建") || line.contains("检查") || line.contains("确认") {
                println!("  💡 {}", line);
            }
        }
    }
}