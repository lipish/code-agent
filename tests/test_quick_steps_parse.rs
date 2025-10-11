//! 快速展示步骤解析结果

use tokio;
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};

#[tokio::test]
async fn test_quick_steps_parse() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 快速步骤解析演示");
    println!("===================");
    
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 800,
        temperature: 0.3,
    };
    
    let model = LlmModel::from_config(model_config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let simple_prompt = "请列出创建Rust项目hello-world的5个具体步骤，每个步骤用一行，格式为：步骤X：具体操作";
    
    println!("📤 请求: {}", simple_prompt);
    println!();
    
    match model.complete(simple_prompt).await {
        Ok(response) => {
            println!("✅ LLM原始回复:");
            println!("{}", "═".repeat(50));
            println!("{}", response.content);
            println!("{}", "═".repeat(50));
            
            println!();
            println!("🔍 解析后的步骤:");
            println!("{}", "─".repeat(30));
            
            // 直接按行解析
            let mut step_count = 0;
            for (i, line) in response.content.lines().enumerate() {
                let line = line.trim();
                if !line.is_empty() {
                    if line.contains("步骤") || line.starts_with(char::is_numeric) {
                        step_count += 1;
                        println!("📌 [步骤{}] {}", step_count, line);
                    } else if line.len() > 10 && (line.contains("cargo") || line.contains("cd") || line.contains("创建")) {
                        println!("   💡 {}", line);
                    }
                }
            }
            
            println!();
            println!("📊 解析统计:");
            println!("   总共识别出 {} 个步骤", step_count);
            
            Ok(())
        }
        Err(e) => {
            println!("❌ 调用失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}