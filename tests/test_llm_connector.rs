use llm_connector::{LlmClient, ChatRequest, Message};
use std::collections::HashMap;
use serde_yaml::Value;
use std::fs;

#[tokio::test]
async fn test_llm_connector_direct() {
    println!("🧪 Direct LLM Connector Test");
    println!("============================");
    println!();

    // Load and validate keys.yaml
    match load_keys_yaml() {
        Ok(keys) => {
            // Test main providers first (most likely to work)
            test_provider_direct("zhipu", &keys).await;
            test_provider_direct("deepseek", &keys).await;
            test_provider_direct("moonshot", &keys).await;
            
            // Test other providers (may have configuration issues)
            println!("🔍 Testing additional providers (may need configuration fixes):");
            test_provider_direct("aliyun", &keys).await;
            test_provider_direct("longcat", &keys).await;
            test_provider_direct("volcengine", &keys).await;
            
            println!("✅ Direct connector test completed!");
        }
        Err(e) => {
            println!("❌ Failed to load configuration: {}", e);
            panic!("Configuration loading failed");
        }
    }
}

#[tokio::test]
async fn test_working_providers_only() {
    println!("🎆 Testing Known Working Providers Only");
    println!("=====================================");
    println!();

    match load_keys_yaml() {
        Ok(keys) => {
            // Only test providers that should definitely work
            let working_providers = ["deepseek", "moonshot", "zhipu"];
            
            for provider in working_providers {
                if keys.contains_key(provider) {
                    test_provider_direct(provider, &keys).await;
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load configuration: {}", e);
        }
    }
}

fn load_keys_yaml() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    println!("📁 Loading keys.yaml...");
    
    // In tests/ directory, keys.yaml is in parent directory
    let keys_path = "./keys.yaml";
    if !std::path::Path::new(keys_path).exists() {
        println!("❌ keys.yaml not found at {}", keys_path);
        println!("💡 Please copy keys.yaml.example to keys.yaml and add your API keys");
        return Err("keys.yaml not found".into());
    }

    let content = fs::read_to_string(keys_path)?;
    let yaml: Value = serde_yaml::from_str(&content)?;
    
    let mut keys = HashMap::new();
    
    if let Some(providers) = yaml.get("providers").and_then(|v| v.as_mapping()) {
        for (provider_name, provider_config) in providers {
            if let (Some(name), Some(config)) = (provider_name.as_str(), provider_config.as_mapping()) {
                if let Some(api_key) = config.get(&Value::String("api_key".to_string())).and_then(|v| v.as_str()) {
                    println!("    🔑 Found api_key for {}: {} chars", name, api_key.len());
                    
                    if !api_key.is_empty() && api_key != "your-api-key-here" && !api_key.starts_with("your-") {
                        keys.insert(name.to_string(), api_key.to_string());
                        println!("    ✅ Loaded {} API key ({}...{})", 
                                name, 
                                &api_key[..std::cmp::min(6, api_key.len())],
                                &api_key[std::cmp::max(0, api_key.len().saturating_sub(4))..]);
                        println!("    🔍 Key starts with: '{}'", &api_key[..std::cmp::min(10, api_key.len())]);
                    } else {
                        println!("    ⚠️  {} API key is empty or placeholder ('{}')", name, api_key);
                    }
                } else {
                    println!("❌ {} missing api_key field", name);
                }
            }
        }
    }
    
    println!("📊 Total valid API keys loaded: {}", keys.len());
    println!();
    
    Ok(keys)
}

async fn test_provider_direct(provider_name: &str, keys: &HashMap<String, String>) {
    println!("🔍 Testing {} provider (direct llm-connector):", provider_name);
    
    // Check if API key exists
    let api_key = match keys.get(provider_name) {
        Some(key) => {
            println!("  ✅ API key found ({}...{})", 
                    &key[..std::cmp::min(6, key.len())],
                    &key[std::cmp::max(0, key.len().saturating_sub(4))..]);
            println!("  🔍 Full API key length: {} characters", key.len());
            key
        }
        None => {
            println!("  ❌ No API key found - skipping test");
            println!();
            return;
        }
    };

    // Create direct llm-connector client based on provider
    let (client, model_name) = create_direct_client(provider_name, api_key);
    
    match client {
        Ok(llm_client) => {
            println!("  ✅ LLM client created successfully");
            println!("  🔍 Model: {}", model_name);
            
            // Create a simple chat request
            let prompt = "What is 1+1? Answer with just the number.";
            let request = ChatRequest {
                model: model_name.clone(),
                messages: vec![Message::user(prompt)],
                ..Default::default()
            };
            
            println!("  🚀 Testing chat request with model: {}", model_name);
            println!("  📤 Sending prompt to LLM service:");
            println!("      > '{}'", prompt);
            println!("  🔄 Request details:");
            println!("      Model: {}", request.model);
            println!("      Messages: {} message(s)", request.messages.len());
            println!("  📡 Making API call...");
            
            // Make the API call
            match llm_client.chat(&request).await {
                Ok(response) => {
                    println!("  ✅ Response received successfully!");
                    println!("  📥 LLM Response:");
                    
                    if let Some(choice) = response.choices.first() {
                        println!("      < '{}'", choice.message.content.trim());
                        println!("      Role: {:?}", choice.message.role);
                        if let Some(finish_reason) = &choice.finish_reason {
                            println!("      Finish reason: {}", finish_reason);
                        }
                    } else {
                        println!("      < No response content found");
                    }
                    
                    if let Some(usage) = response.usage {
                        println!("  📊 Token usage:");
                        println!("      Prompt tokens: {}", usage.prompt_tokens);
                        println!("      Completion tokens: {}", usage.completion_tokens);
                        println!("      Total tokens: {}", usage.total_tokens);
                    }
                    
                    println!("  ✅ 🎉 SUCCESS: Prompt sent and response received!");
                }
                Err(e) => {
                    println!("  ❌ API call failed: {}", e);
                    println!("  🔍 Error analysis:");
                    let error_str = e.to_string();
                    if error_str.contains("authentication") || error_str.contains("Authentication") || error_str.contains("401") {
                        println!("    - ⚠️  AUTHENTICATION FAILED");
                        println!("    - The API key was sent to the provider but was rejected");
                        println!("    - This confirms the llm-connector is working correctly");
                        println!("    - Issue: API key is invalid, expired, or lacks permissions");
                    } else if error_str.contains("API key") || error_str.contains("api_key") {
                        println!("    - 🔑 API KEY CONFIGURATION ERROR");
                        println!("    - The API key might not be properly set in the request");
                    } else if error_str.contains("network") || error_str.contains("timeout") || error_str.contains("connection") {
                        println!("    - 🌐 NETWORK ERROR");
                        println!("    - This might be a connectivity issue");
                    } else {
                        println!("    - 🔍 OTHER ERROR: {}", error_str);
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ LLM client creation failed: {}", e);
            println!("  🔍 This indicates an issue with llm-connector setup");
        }
    }
    
    println!();
}

fn create_direct_client(provider_name: &str, api_key: &str) -> (Result<LlmClient, Box<dyn std::error::Error>>, String) {
    let (endpoint, model_name) = match provider_name {
        "zhipu" => (
            "https://open.bigmodel.cn/api/paas/v4",
            "glm-4"
        ),
        "deepseek" => (
            "https://api.deepseek.com/v1",
            "deepseek-chat"
        ),
        "moonshot" => (
            "https://api.moonshot.cn/v1",
            "moonshot-v1-8k"
        ),
        "aliyun" => (
            "https://dashscope.aliyuncs.com/api/v1",
            "qwen3-max"  // Updated model name
        ),
        "longcat" => (
            "https://api.longcat.chat/v1",  // Fixed endpoint
            "LongCat-Flash-Chat"  // Updated model name
        ),
        "volcengine" => (
            "https://ark.cn-beijing.volces.com/api/v3",
            "ep-20241008150227-6k2gt"  // Use actual endpoint ID format
        ),
        _ => (
            "https://api.openai.com/v1",  // Fallback
            "gpt-3.5-turbo"
        )
    };

    println!("  🔧 Creating client:");
    println!("    Endpoint: {}", endpoint);
    println!("    Model: {}", model_name);
    println!("    API Key: {}...{}", 
            &api_key[..std::cmp::min(8, api_key.len())],
            &api_key[std::cmp::max(0, api_key.len().saturating_sub(4))..]);

    // Create the LLM client using llm-connector 0.3.1 API
    let client_result = match provider_name {
        "aliyun" => {
            // Aliyun uses different authentication format
            std::env::set_var("DASHSCOPE_API_KEY", api_key);
            Ok(LlmClient::openai(api_key, Some(endpoint)))
        },
        "volcengine" => {
            // VolcEngine - OpenAI-compatible
            Ok(LlmClient::openai(api_key, Some(endpoint)))
        },
        _ => {
            // For most providers, use OpenAI-compatible client
            Ok(LlmClient::openai(api_key, Some(endpoint)))
        }
    };

    (client_result, model_name.to_string())
}