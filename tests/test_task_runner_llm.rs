use task_runner::models::{LlmModel, LanguageModel};
use task_runner::config::{ModelConfig, ModelProvider};
use serde_yaml::Value;
use std::fs;
use std::collections::HashMap;

#[tokio::test]
async fn test_task_runner_llm_integration() {
    println!("🧪 Task-Runner LLM Integration Test - Multiple Providers");
    println!("======================================================");
    println!();

    // More interesting test prompt
    let test_prompt = "Write a simple Rust function that calculates the factorial of a number. Only provide the code without explanation.";
    println!("📝 Test prompt: '{}'", test_prompt);
    println!();

    // Test DeepSeek
    test_provider_integration(
        "DeepSeek",
        ModelProvider::DeepSeek,
        "deepseek-chat",
        "sk-78f437f4e0174650ae18734e6ec5bd03",
        "https://api.deepseek.com/v1",
        test_prompt
    ).await;

    // Test Zhipu
    test_provider_integration(
        "Zhipu",
        ModelProvider::Zhipu,
        "glm-4",
        "your-api-key-here",
        "https://open.bigmodel.cn/api/paas/v4",
        test_prompt
    ).await;

    // Test Moonshot
    test_provider_integration(
        "Moonshot",
        ModelProvider::Moonshot,
        "moonshot-v1-8k",
        "sk-5ipahcLR7y73YfOE5Tlkq39cpcIIcbLcOKlI7G69x7DtVw4b",
        "https://api.moonshot.cn/v1",
        test_prompt
    ).await;

    // Test LongCat
    test_provider_integration(
        "LongCat",
        ModelProvider::LongCat,
        "LongCat-Flash-Chat",
        "ak_11o3bI6O03mx2yS8jb2hD61q7DJ4d",
        "https://api.longcat.chat/openai",  // Fixed: use /openai not /v1
        test_prompt
    ).await;

    // Load enhanced configuration for VolcEngine
    let volcengine_config = load_provider_config("volcengine");
    let volcengine_endpoint = volcengine_config.as_ref()
        .and_then(|config| config.get("endpoint"))
        .and_then(|endpoint| endpoint.as_str())
        .map(|endpoint_id| format!("https://ark.cn-beijing.volces.com/api/v3/{}", endpoint_id))
        .unwrap_or_else(|| "https://ark.cn-beijing.volces.com/api/v3".to_string());
    
    let volcengine_model = volcengine_config.as_ref()
        .and_then(|config| config.get("endpoint"))
        .and_then(|endpoint| endpoint.as_str())
        .unwrap_or("ep-20251006132256-vrq2pyw");

    // Test VolcEngine with endpoint from keys.yaml
    test_provider_integration(
        "VolcEngine",
        ModelProvider::VolcEngine,
        volcengine_model,  // Use endpoint ID as model name for VolcEngine
        "26f962bd-450e-4876-bc32-a732e6da9cd2",
        &volcengine_endpoint,  // Use constructed endpoint URL
        test_prompt
    ).await;

    // Test Aliyun
    test_provider_integration(
        "Aliyun",
        ModelProvider::Aliyun,
        "qwen3-max",
        "sk-17cb8a1feec2440bad2c5a73d7d08af2",
        "https://dashscope.aliyuncs.com/compatible-mode/v1",  // Fixed: use compatible-mode endpoint
        test_prompt
    ).await;

    println!();
    println!("✅ Task-runner LLM integration test completed for all providers!");
}

async fn test_provider_integration(
    provider_name: &str,
    provider: ModelProvider,
    model_name: &str,
    api_key: &str,
    endpoint: &str,
    prompt: &str
) {
    println!("🔧 Testing {} via task-runner LlmModel:", provider_name);
    println!("  Provider: {:?}", provider);
    println!("  Model: {}", model_name);
    println!("  Endpoint: {}", endpoint);
    println!("  API Key: {}...{}", 
             &api_key[..std::cmp::min(8, api_key.len())],
             &api_key[std::cmp::max(0, api_key.len().saturating_sub(4))..]);

    let config = ModelConfig {
        provider,
        model_name: model_name.to_string(),
        api_key: Some(api_key.to_string()),
        endpoint: Some(endpoint.to_string()),
        max_tokens: 4000,
        temperature: 0.7,
    };

    match LlmModel::from_config(config) {
        Ok(model) => {
            println!("  ✅ LlmModel created successfully");
            println!("  🔍 Model name: {}", model.model_name());
            println!("  🔧 Supports tools: {}", model.supports_tools());
            
            println!("  🚀 Testing completion...");
            println!("  📤 Sending prompt to LLM service:");
            println!("      > '{}'", prompt);
            println!("  📡 Making API call to {}...", endpoint);
            
            match model.complete(prompt).await {
                Ok(response) => {
                    println!("  ✅ SUCCESS! {} integration works!", provider_name);
                    println!("  📥 Full LLM Response:");
                    let content = response.content.trim();
                    println!("      < '{}'", content);
                    if let Some(usage) = response.usage {
                        println!("  📊 Token usage details:");
                        println!("      Prompt tokens: {}", usage.prompt_tokens);
                        println!("      Completion tokens: {}", usage.completion_tokens);
                        println!("      Total tokens: {}", usage.total_tokens);
                    }
                    println!("  🎉 {} LLM connector integration VERIFIED!", provider_name);
                }
                Err(e) => {
                    println!("  ❌ {} completion failed: {}", provider_name, e);
                    println!("  🔍 Error analysis:");
                    let error_str = e.to_string();
                    if error_str.contains("authentication") || error_str.contains("401") {
                        println!("    - Authentication error: API key was rejected");
                        println!("    - This means task-runner successfully sent the request");
                        println!("    - But the API key might be invalid or expired");
                    } else if error_str.contains("API key") {
                        println!("    - API key configuration error in task-runner");
                    } else {
                        println!("    - Other error: {}", error_str);
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Failed to create {} LlmModel: {}", provider_name, e);
            println!("  🔍 This indicates a configuration or setup issue");
        }
    }
    
    println!();
}

fn load_provider_config(provider_name: &str) -> Option<HashMap<String, Value>> {
    let content = fs::read_to_string("./keys.yaml").ok()?;
    let yaml: Value = serde_yaml::from_str(&content).ok()?;
    
    yaml.get("providers")
        .and_then(|providers| providers.as_mapping())
        .and_then(|providers| providers.get(&Value::String(provider_name.to_string())))
        .and_then(|provider| provider.as_mapping())
        .map(|mapping| {
            mapping.iter().map(|(k, v)| {
                (k.as_str().unwrap_or("").to_string(), v.clone())
            }).collect()
        })
}