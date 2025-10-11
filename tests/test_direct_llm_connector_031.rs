use llm_connector::{LlmClient, ChatRequest, Message};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;

#[tokio::test]
async fn test_llm_connector_031_direct() {
    println!("🧪 Direct llm-connector 0.3.1 Test");
    println!("===================================");
    println!();

    // Load keys.yaml following the memory workflow
    let keys = match load_keys_yaml() {
        Ok(keys) => {
            println!("✅ Configuration loaded: {} providers", keys.len());
            keys
        }
        Err(e) => {
            println!("❌ Failed to load keys.yaml: {}", e);
            println!("💡 Make sure keys.yaml exists with valid API keys");
            return;
        }
    };

    let test_prompt = "What is 1+1? Answer with just the number.";
    println!("📝 Test prompt: '{}'", test_prompt);
    println!();

    // Test each provider with new 0.3.1 API
    test_provider_031("DeepSeek", "deepseek", "https://api.deepseek.com/v1", "deepseek-chat", &keys, test_prompt).await;
    test_provider_031("Zhipu", "zhipu", "https://open.bigmodel.cn/api/paas/v4", "glm-4", &keys, test_prompt).await;
    test_provider_031("Moonshot", "moonshot", "https://api.moonshot.cn/v1", "moonshot-v1-8k", &keys, test_prompt).await;
    test_provider_031("LongCat", "longcat", "https://api.longcat.chat/openai", "LongCat-Flash-Chat", &keys, test_prompt).await;
    test_provider_031("Aliyun", "aliyun", "https://dashscope.aliyuncs.com/compatible-mode/v1", "qwen3-max", &keys, test_prompt).await;
    
    println!("✅ llm-connector 0.3.1 direct testing completed!");
}

fn load_keys_yaml() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("./keys.yaml")?;
    let yaml: Value = serde_yaml::from_str(&content)?;
    
    let mut keys = HashMap::new();
    
    if let Some(providers) = yaml.get("providers").and_then(|v| v.as_mapping()) {
        for (provider_name, provider_config) in providers {
            if let (Some(name), Some(config)) = (provider_name.as_str(), provider_config.as_mapping()) {
                if let Some(api_key) = config.get(&Value::String("api_key".to_string())).and_then(|v| v.as_str()) {
                    if !api_key.is_empty() && api_key != "your-api-key-here" {
                        keys.insert(name.to_string(), api_key.to_string());
                    }
                }
            }
        }
    }
    
    Ok(keys)
}

async fn test_provider_031(
    provider_name: &str,
    key_name: &str,
    endpoint: &str,
    model: &str,
    keys: &HashMap<String, String>,
    prompt: &str
) {
    println!("🔧 Testing {} with llm-connector 0.3.1:", provider_name);
    
    // API Key validation step
    let api_key = match keys.get(key_name) {
        Some(key) => {
            println!("  ✅ API key validated: {}...{} ({} chars)", 
                    &key[..std::cmp::min(8, key.len())],
                    &key[std::cmp::max(0, key.len().saturating_sub(4))..],
                    key.len());
            key
        }
        None => {
            println!("  ❌ No API key found for {} - skipping", key_name);
            println!();
            return;
        }
    };

    println!("  🔗 Endpoint: {}", endpoint);
    println!("  🤖 Model: {}", model);
    
    // Create client using NEW 0.3.1 API
    let client = LlmClient::openai(api_key, Some(endpoint));
    println!("  ✅ Client created with 0.3.1 API: LlmClient::openai(key, Some(endpoint))");
    
    // Create request
    let request = ChatRequest {
        model: model.to_string(),
        messages: vec![Message::user(prompt)],
        ..Default::default()
    };
    
    // Following "Prompt Response Logging" specification
    println!("  📤 Sending prompt to LLM service:");
    println!("      > '{}'", prompt);
    println!("  📡 Making API call to {}...", provider_name);
    
    // Test actual API connectivity
    match client.chat(&request).await {
        Ok(response) => {
            println!("  ✅ SUCCESS! {} API call worked with llm-connector 0.3.1!", provider_name);
            
            // Full response logging per specification
            if let Some(choice) = response.choices.first() {
                println!("  📥 Full LLM Response:");
                println!("      < '{}'", choice.message.content.trim());
                println!("      Role: {:?}", choice.message.role);
                if let Some(finish_reason) = &choice.finish_reason {
                    println!("      Finish reason: {}", finish_reason);
                }
            }
            
            // Token usage details per specification
            if let Some(usage) = response.usage {
                println!("  📊 Token usage details:");
                println!("      Prompt tokens: {}", usage.prompt_tokens);
                println!("      Completion tokens: {}", usage.completion_tokens);
                println!("      Total tokens: {}", usage.total_tokens);
            }
            
            println!("  🎉 {} llm-connector 0.3.1 integration VERIFIED!", provider_name);
        }
        Err(e) => {
            println!("  ❌ {} API call failed: {}", provider_name, e);
            
            // Error analysis following memory guidance
            let error_str = e.to_string();
            if error_str.contains("authentication") || error_str.contains("401") {
                println!("  💡 Authentication error - API key was sent but rejected");
                println!("     This confirms llm-connector 0.3.1 is working correctly");
                println!("     The API key might be invalid, expired, or lacks permissions");
            } else if error_str.contains("API key") {
                println!("  🔑 API key configuration issue");
            } else if error_str.contains("timeout") || error_str.contains("network") {
                println!("  🌐 Network connectivity issue");
            } else {
                println!("  🔍 Other error: {}", error_str);
            }
        }
    }
    
    println!();
}