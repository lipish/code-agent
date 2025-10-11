use tokio;
use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};

#[tokio::test]
async fn test_raw_llm_interaction_observation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 直接观察LLM模型交互过程");
    println!("=====================================");
    
    // 创建模型配置
    let config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some(std::env::var("ZHIPU_API_KEY").unwrap_or_else(|_| "your-api-key-here".to_string())),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 2000,
        temperature: 0.7,
    };
    
    println!("📋 模型配置:");
    println!("  - 提供商: {:?}", config.provider);
    println!("  - 模型: {}", config.model_name);
    println!("  - 最大令牌: {}", config.max_tokens);
    println!("  - 温度: {}", config.temperature);
    
    // 创建LLM实例
    let llm = match LlmModel::from_config(config) {
        Ok(model) => model,
        Err(e) => {
            println!("❌ LLM实例创建失败: {}", e);
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };
    println!("✅ LLM实例创建成功");
    
    // 构造输入提示词
    let input_prompt = r#"你是一个任务规划专家。请将以下任务分解为详细的步骤计划：

任务：创建一个Rust工程，名字叫hello-world

请按照以下格式返回：
1. 任务理解：[描述你对任务的理解]
2. 准备工作：[列出需要的准备工作]
3. 详细步骤：[列出具体的执行步骤]
4. 验证方法：[如何验证任务完成]

请用中文回答，并提供详细的技术说明。"#;
    
    println!("\n📝 发送给模型的提示词:");
    println!("=====================================");
    println!("{}", input_prompt);
    
    println!("\n🚀 正在调用LLM模型...");
    println!("=====================================");
    
    let start_time = std::time::Instant::now();
    
    // 直接调用LLM
    match llm.complete(&input_prompt).await {
        Ok(response) => {
            let duration = start_time.elapsed();
            
            println!("✅ LLM调用成功！耗时: {:?}", duration);
            println!("\n🤖 模型原始返回内容:");
            println!("=====================================");
            println!("{}", response.content);
            println!("=====================================");
            
            // 分析返回内容
            println!("\n📊 返回内容分析:");
            println!("  - 响应长度: {} 字符", response.content.len());
            println!("  - 包含行数: {} 行", response.content.lines().count());
            
            // 检查是否包含预期的结构
            let has_task_understanding = response.content.contains("任务理解") || response.content.contains("理解");
            let has_preparation = response.content.contains("准备工作") || response.content.contains("准备");
            let has_steps = response.content.contains("详细步骤") || response.content.contains("步骤");
            let has_verification = response.content.contains("验证") || response.content.contains("检查");
            
            println!("  - 包含任务理解: {}", if has_task_understanding { "✅" } else { "❌" });
            println!("  - 包含准备工作: {}", if has_preparation { "✅" } else { "❌" });
            println!("  - 包含详细步骤: {}", if has_steps { "✅" } else { "❌" });
            println!("  - 包含验证方法: {}", if has_verification { "✅" } else { "❌" });
            
            // 统计数字列表项
            let numbered_items = response.content.lines()
                .filter(|line| line.trim().chars().next().map_or(false, |c| c.is_ascii_digit()))
                .count();
            println!("  - 编号项目数: {}", numbered_items);
            
            // 检查Rust相关内容
            let rust_mentions = response.content.matches("Rust").count() + 
                              response.content.matches("rust").count() + 
                              response.content.matches("cargo").count() +
                              response.content.matches("Cargo").count();
            println!("  - Rust相关提及: {} 次", rust_mentions);
            
            // 显示token使用情况
            if let Some(usage) = &response.usage {
                println!("  - Token使用: {} (输入: {}, 输出: {})", 
                        usage.total_tokens, usage.prompt_tokens, usage.completion_tokens);
            }
            
            Ok(())
        },
        Err(e) => {
            let duration = start_time.elapsed();
            println!("❌ LLM调用失败！耗时: {:?}", duration);
            println!("错误信息: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}

#[tokio::test]
async fn test_step_by_step_observation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 分步骤观察LLM交互");
    println!("========================");
    
    let config = ModelConfig {
        provider: ModelProvider::Zhipu,
        model_name: "glm-4-flash".to_string(),
        api_key: Some(std::env::var("ZHIPU_API_KEY").unwrap_or_else(|_| "your-api-key-here".to_string())),
        endpoint: Some("https://open.bigmodel.cn/api/paas/v4".to_string()),
        max_tokens: 1500,
        temperature: 0.3,
    };
    
    let llm = match LlmModel::from_config(config) {
        Ok(model) => model,
        Err(e) => {
            println!("❌ LLM实例创建失败: {}", e);
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };
    
    // 第一步：理解任务
    let step1_prompt = "请分析这个任务：创建一个Rust工程，名字叫hello-world。你认为这个任务的核心要求是什么？";
    
    println!("\n📝 步骤1 - 任务理解");
    println!("提示词: {}", step1_prompt);
    
    if let Ok(response1) = llm.complete(&step1_prompt).await {
        println!("🤖 模型回复:");
        println!("{}", response1.content);
        
        // 第二步：制定计划
        let step2_prompt = format!(
            "基于你对任务的理解：{}，现在请制定详细的执行计划，包括具体的命令和步骤。", 
            response1.content.chars().take(200).collect::<String>()
        );
        
        println!("\n📝 步骤2 - 制定计划");
        println!("提示词: {}", step2_prompt);
        
        if let Ok(response2) = llm.complete(&step2_prompt).await {
            println!("🤖 模型回复:");
            println!("{}", response2.content);
            
            println!("\n✅ 两步骤交互完成");
        }
    }
    
    Ok(())
}