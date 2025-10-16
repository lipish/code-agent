//! Demonstration of llm-connector 0.3.8 Features
//!
//! This example showcases the capabilities of llm-connector library:
//! - Multiple protocol support (OpenAI, Anthropic, Zhipu, Aliyun, Ollama)
//! - Model discovery (fetch available models from API)
//! - Unified interface across different providers
//! - Protocol information retrieval

use agent_runner::config::{ModelConfig, ModelProvider};
use agent_runner::models::{LlmModel, LanguageModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 llm-connector 0.3.8 功能演示");
    println!("================================================================================\n");

    // Demo 1: OpenAI
    demo_openai().await?;
    
    // Demo 2: Zhipu (使用专用构造函数)
    demo_zhipu().await?;
    
    // Demo 3: Aliyun (使用专用构造函数)
    demo_aliyun().await?;
    
    // Demo 4: Ollama (本地模型)
    demo_ollama().await?;
    
    // Demo 5: OpenAI-compatible providers
    demo_openai_compatible().await?;

    Ok(())
}

async fn demo_openai() -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Demo 1: OpenAI Protocol");
    println!("────────────────────────────────────────");
    
    // 检查环境变量
    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        let config = ModelConfig {
            provider: ModelProvider::OpenAI,
            model_name: "gpt-4".to_string(),
            api_key: Some(api_key),
            endpoint: None, // 使用默认端点
            max_tokens: 100,
            temperature: 0.7,
        };
        
        let model = LlmModel::from_config(config)?;
        
        println!("✓ 协议: {}", model.protocol_name());
        println!("✓ 模型名称: {}", model.model_name());
        println!("✓ 支持工具: {}", if model.supports_tools() { "是" } else { "否" });
        
        // 尝试获取可用模型列表
        match model.fetch_available_models().await {
            Ok(models) => {
                println!("✓ 可用模型: {:?}", &models[..models.len().min(5)]);
                if models.len() > 5 {
                    println!("  ... 还有 {} 个模型", models.len() - 5);
                }
            }
            Err(e) => println!("⚠️  获取模型列表失败: {}", e),
        }
    } else {
        println!("⚠️  未设置 OPENAI_API_KEY 环境变量，跳过此演示");
    }
    
    println!();
    Ok(())
}

async fn demo_zhipu() -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Demo 2: Zhipu Protocol (智谱AI)");
    println!("────────────────────────────────────────");
    
    if let Ok(api_key) = std::env::var("ZHIPU_API_KEY") {
        let config = ModelConfig {
            provider: ModelProvider::Zhipu,
            model_name: "glm-4".to_string(),
            api_key: Some(api_key),
            endpoint: None, // 使用 llm-connector 内置的 Zhipu 端点
            max_tokens: 100,
            temperature: 0.7,
        };
        
        let model = LlmModel::from_config(config)?;
        
        println!("✓ 协议: {}", model.protocol_name());
        println!("✓ 模型名称: {}", model.model_name());
        println!("✓ 使用专用构造函数: LlmClient::zhipu()");
        
        // Zhipu 支持模型发现
        match model.fetch_available_models().await {
            Ok(models) => {
                println!("✓ 可用模型: {:?}", models);
            }
            Err(e) => println!("⚠️  获取模型列表失败: {}", e),
        }
    } else {
        println!("⚠️  未设置 ZHIPU_API_KEY 环境变量，跳过此演示");
        println!("   提示: Zhipu AI 现在有专用的构造函数 LlmClient::zhipu()");
    }
    
    println!();
    Ok(())
}

async fn demo_aliyun() -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Demo 3: Aliyun Protocol (阿里云 DashScope)");
    println!("────────────────────────────────────────");
    
    if let Ok(api_key) = std::env::var("ALIYUN_API_KEY") {
        let config = ModelConfig {
            provider: ModelProvider::Aliyun,
            model_name: "qwen-max".to_string(),
            api_key: Some(api_key),
            endpoint: None, // 使用 llm-connector 内置的 Aliyun 端点
            max_tokens: 100,
            temperature: 0.7,
        };
        
        let model = LlmModel::from_config(config)?;
        
        println!("✓ 协议: {}", model.protocol_name());
        println!("✓ 模型名称: {}", model.model_name());
        println!("✓ 使用专用构造函数: LlmClient::aliyun()");
        
        // Aliyun 不支持模型发现
        println!("⚠️  Aliyun 协议不支持自动模型发现");
        println!("   可用模型: qwen-turbo, qwen-plus, qwen-max");
    } else {
        println!("⚠️  未设置 ALIYUN_API_KEY 环境变量，跳过此演示");
        println!("   提示: Aliyun DashScope 现在有专用的构造函数 LlmClient::aliyun()");
    }
    
    println!();
    Ok(())
}

async fn demo_ollama() -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Demo 4: Ollama Protocol (本地模型)");
    println!("────────────────────────────────────────");
    
    let config = ModelConfig {
        provider: ModelProvider::Ollama,
        model_name: "llama3.2".to_string(),
        api_key: None, // Ollama 不需要 API key
        endpoint: None, // 默认 http://localhost:11434
        max_tokens: 100,
        temperature: 0.7,
    };
    
    let model = LlmModel::from_config(config)?;
    
    println!("✓ 协议: {}", model.protocol_name());
    println!("✓ 模型名称: {}", model.model_name());
    println!("✓ 无需 API key");
    
    // Ollama 支持完整的模型发现
    match model.fetch_available_models().await {
        Ok(models) => {
            println!("✓ 本地已安装的模型:");
            for (i, model_name) in models.iter().enumerate() {
                println!("  {}. {}", i + 1, model_name);
            }
            
            if models.is_empty() {
                println!("  (未找到已安装的模型)");
                println!("  提示: 使用 'ollama pull llama3.2' 下载模型");
            }
        }
        Err(e) => {
            println!("⚠️  无法连接到 Ollama: {}", e);
            println!("  提示: 确保 Ollama 服务正在运行 (http://localhost:11434)");
        }
    }
    
    println!();
    Ok(())
}

async fn demo_openai_compatible() -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Demo 5: OpenAI-Compatible Providers");
    println!("────────────────────────────────────────");
    
    // DeepSeek
    if let Ok(api_key) = std::env::var("DEEPSEEK_API_KEY") {
        println!("\n🔹 DeepSeek:");
        let config = ModelConfig {
            provider: ModelProvider::DeepSeek,
            model_name: "deepseek-chat".to_string(),
            api_key: Some(api_key),
            endpoint: None,
            max_tokens: 100,
            temperature: 0.7,
        };
        
        let model = LlmModel::from_config(config)?;
        println!("  协议: {}", model.protocol_name());
        
        match model.fetch_available_models().await {
            Ok(models) => println!("  可用模型: {:?}", models),
            Err(e) => println!("  获取失败: {}", e),
        }
    }
    
    // Moonshot
    if let Ok(api_key) = std::env::var("MOONSHOT_API_KEY") {
        println!("\n🔹 Moonshot (Kimi):");
        let config = ModelConfig {
            provider: ModelProvider::Moonshot,
            model_name: "moonshot-v1-32k".to_string(),
            api_key: Some(api_key),
            endpoint: None,
            max_tokens: 100,
            temperature: 0.7,
        };
        
        let model = LlmModel::from_config(config)?;
        println!("  协议: {}", model.protocol_name());
        
        match model.fetch_available_models().await {
            Ok(models) => println!("  可用模型: {:?}", models),
            Err(e) => println!("  获取失败: {}", e),
        }
    }
    
    // LongCat
    if let Ok(api_key) = std::env::var("LONGCAT_API_KEY") {
        println!("\n🔹 LongCat:");
        let config = ModelConfig {
            provider: ModelProvider::LongCat,
            model_name: "LongCat-Flash-Chat".to_string(),
            api_key: Some(api_key),
            endpoint: None,
            max_tokens: 100,
            temperature: 0.7,
        };
        
        let model = LlmModel::from_config(config)?;
        println!("  协议: {}", model.protocol_name());
        
        match model.fetch_available_models().await {
            Ok(models) => println!("  可用模型: {:?}", models),
            Err(e) => println!("  获取失败: {}", e),
        }
    }
    
    if std::env::var("DEEPSEEK_API_KEY").is_err() 
        && std::env::var("MOONSHOT_API_KEY").is_err()
        && std::env::var("LONGCAT_API_KEY").is_err() {
        println!("\n⚠️  未设置任何 OpenAI-compatible 提供商的 API key");
        println!("   支持的提供商: DeepSeek, Moonshot, LongCat, VolcEngine 等");
    }
    
    println!();
    Ok(())
}
