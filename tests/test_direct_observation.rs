//! 直接观察agent处理过程的测试
//! 用于观察"创建Rust工程"任务的输入输出

use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};

#[tokio::test]
async fn test_direct_llm_call() {
    println!("🎯 直接观察LLM调用过程");
    println!("======================");
    
    // 创建模型
    let config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some("your-api-key-here".to_string()),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 800,
        temperature: 0.7,
    };
    
    let model = LlmModel::from_config(config).expect("Failed to create model");
    
    // 测试提示
    let prompt = r#"
任务：创建一个Rust工程

请详细分析这个任务，并制定具体的执行步骤计划。要求：
1. 分析任务的复杂度
2. 列出需要的步骤（用数字编号）
3. 说明每个步骤的具体操作
4. 估计整个过程需要多长时间

请用中文回答，格式要清晰。
"#;
    
    println!("📤 输入提示内容：");
    println!("================");
    println!("{}", prompt.trim());
    println!();
    
    println!("🔄 正在调用智谱AI GLM-4...");
    println!();
    
    match model.complete(prompt).await {
        Ok(response) => {
            println!("✅ 调用成功！");
            println!();
            
            println!("📥 完整模型输出：");
            println!("================");
            println!("{}", response.content);
            println!();
            
            if let Some(usage) = response.usage {
                println!("📊 使用统计：");
                println!("============");
                println!("输入tokens: {}", usage.prompt_tokens);
                println!("输出tokens: {}", usage.completion_tokens);
                println!("总计tokens: {}", usage.total_tokens);
                println!();
            }
            
            // 分析输出内容
            println!("🔍 内容分析：");
            println!("============");
            
            let content = &response.content;
            
            // 检查是否有步骤编号
            let has_numbered_steps = content.contains("1.") && content.contains("2.");
            println!("包含编号步骤: {}", if has_numbered_steps { "✅ 是" } else { "❌ 否" });
            
            // 检查Rust相关内容
            let has_rust_content = content.contains("cargo") || content.contains("Rust") || content.contains("Cargo.toml");
            println!("包含Rust知识: {}", if has_rust_content { "✅ 是" } else { "❌ 否" });
            
            // 检查复杂度分析
            let has_complexity = content.contains("复杂") || content.contains("简单") || content.contains("难度");
            println!("包含复杂度分析: {}", if has_complexity { "✅ 是" } else { "❌ 否" });
            
            // 检查时间估计
            let has_time_estimate = content.contains("分钟") || content.contains("小时") || content.contains("时间");
            println!("包含时间估计: {}", if has_time_estimate { "✅ 是" } else { "❌ 否" });
            
            println!("响应长度: {} 字符", content.len());
            
        }
        Err(e) => {
            println!("❌ 调用失败: {}", e);
        }
    }
}