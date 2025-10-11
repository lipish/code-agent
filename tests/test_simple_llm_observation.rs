//! 简化的LLM观察测试，直接显示输入输出过程

use tokio;
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};

#[tokio::test]
async fn test_simple_llm_input_output() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 简化LLM输入输出观察");
    println!("====================");
    
    // 创建模型配置
    let config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 800,
        temperature: 0.5,
    };
    
    println!("🔧 模型配置完成: {} ({:?})", config.model_name, config.provider);
    
    // 创建LLM实例
    let llm = LlmModel::from_config(config)?;
    println!("✅ LLM实例创建成功");
    
    // 输入内容
    let input = "任务：创建一个Rust工程，名字叫hello-world\n\n请用中文回答：\n1. 这个任务需要几个步骤？\n2. 第一步应该做什么？\n3. 最重要的命令是什么？";
    
    println!("\n📤 发送给LLM的输入:");
    println!("{}", "─".repeat(50));
    println!("{}", input);
    println!("{}", "─".repeat(50));
    
    println!("\n⏱️ 正在等待LLM响应...");
    
    let start = std::time::Instant::now();
    match llm.complete(input).await {
        Ok(response) => {
            let duration = start.elapsed();
            
            println!("✅ 收到响应！耗时: {:?}", duration);
            println!("\n📥 LLM完整输出:");
            println!("{}", "─".repeat(50));
            println!("{}", response.content);
            println!("{}", "─".repeat(50));
            
            // 简单统计
            println!("\n📊 输出统计:");
            println!("  - 字符数: {}", response.content.len());
            println!("  - 行数: {}", response.content.lines().count());
            println!("  - 包含'cargo': {}", response.content.contains("cargo"));
            println!("  - 包含'rust': {}", response.content.to_lowercase().contains("rust"));
            
            if let Some(usage) = response.usage {
                println!("  - 输入token: {}", usage.prompt_tokens);
                println!("  - 输出token: {}", usage.completion_tokens);
                println!("  - 总token: {}", usage.total_tokens);
            }
            
            Ok(())
        }
        Err(e) => {
            println!("❌ LLM调用失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test]
async fn test_minimal_connectivity() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔗 最小连接测试");
    println!("===============");
    
    let config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 100,
        temperature: 0.1,
    };
    
    let llm = LlmModel::from_config(config)?;
    
    let simple_input = "用中文回答：创建Rust项目的第一步是什么？";
    println!("📤 输入: {}", simple_input);
    
    match llm.complete(simple_input).await {
        Ok(response) => {
            println!("📥 输出: {}", response.content.trim());
            println!("✅ 连接测试成功");
            Ok(())
        }
        Err(e) => {
            println!("❌ 连接测试失败: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}