//! 简单的 LLM 连接测试
//! 测试智谱 API 是否可用

use agent_runner::config::{ModelConfig, ModelProvider};
use agent_runner::models::{LanguageModel, LlmModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 测试智谱 GLM-4 API 连接");
    println!("═══════════════════════════════════════════════════════════════════");
    
    // 配置模型
    let model_config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4".to_string(),
        api_key: Some("d2a0da2b02954b1f91a0a4ec16d4521b.GA2Tz9sF9kt4zVd3".to_string()),
        endpoint: None,
        max_tokens: 500,  // 降低 token 限制以加快测试
        temperature: 0.7,
    };
    
    println!("配置信息:");
    println!("  提供商: {:?}", model_config.provider);
    println!("  模型: {}", model_config.model_name);
    println!("  Max Tokens: {}", model_config.max_tokens);
    println!();
    
    // 创建模型
    println!("创建 LLM 模型实例...");
    let model = LlmModel::from_config(model_config)?;
    
    // 简单测试
    println!("发送测试请求...");
    println!();
    
    let prompt = "请用一句话介绍什么是 License 管理系统。";
    
    println!("📤 Prompt:");
    println!("─────────────────────────────────────────");
    println!("{}", prompt);
    println!();
    
    println!("⏳ 等待 LLM 响应...");
    
    match model.complete(prompt).await {
        Ok(response) => {
            println!();
            println!("📥 Response:");
            println!("─────────────────────────────────────────");
            println!("{}", response.content);
            println!();
            println!("✅ API 连接成功！");
        }
        Err(e) => {
            println!();
            println!("❌ API 调用失败:");
            println!("{:?}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}
