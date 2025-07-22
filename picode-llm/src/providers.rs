use crate::client::{LlmClient, LlmResponse, RequestConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LLM provider trait
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &'static str;
    
    /// Check if provider is configured correctly
    async fn health_check(&self) -> Result<bool>;
    
    /// Generate text completion
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    
    /// Generate chat completion
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    
    /// Get model information
    async fn get_models(&self) -> Result<Vec<ModelInfo>>;
}

/// Text completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    /// Input prompt
    pub prompt: String,
    /// Model name
    pub model: String,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Sampling temperature (0.0 to 2.0)
    pub temperature: Option<f32>,
    /// Top-p nucleus sampling
    pub top_p: Option<f32>,
    /// Number of completions to generate
    pub n: Option<u32>,
    /// Stop sequences
    pub stop: Option<Vec<String>>,
}

/// Text completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// Generated completions
    pub choices: Vec<CompletionChoice>,
    /// Token usage information
    pub usage: TokenUsage,
    /// Response metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Single completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    /// Generated text
    pub text: String,
    /// Finish reason
    pub finish_reason: String,
    /// Log probability information
    pub logprobs: Option<serde_json::Value>,
}

/// Chat completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Chat messages
    pub messages: Vec<ChatMessage>,
    /// Model name
    pub model: String,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Sampling temperature
    pub temperature: Option<f32>,
    /// Top-p nucleus sampling
    pub top_p: Option<f32>,
    /// Stop sequences
    pub stop: Option<Vec<String>>,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role (system, user, assistant)
    pub role: String,
    /// Message content
    pub content: String,
}

/// Chat completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Response choices
    pub choices: Vec<ChatChoice>,
    /// Token usage
    pub usage: TokenUsage,
    /// Response metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Chat completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    /// Response message
    pub message: ChatMessage,
    /// Finish reason
    pub finish_reason: String,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Prompt tokens
    pub prompt_tokens: u32,
    /// Completion tokens
    pub completion_tokens: u32,
    /// Total tokens
    pub total_tokens: u32,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,
    /// Model name
    pub name: String,
    /// Model description
    pub description: Option<String>,
    /// Context window size
    pub context_window: Option<u32>,
    /// Maximum output tokens
    pub max_output_tokens: Option<u32>,
    /// Supported capabilities
    pub capabilities: Vec<String>,
}

/// Generic OpenAPI-compatible provider
#[derive(Debug)]
pub struct GenericProvider {
    client: LlmClient,
    base_url: String,
    api_key: String,
    name: String,
}

impl GenericProvider {
    /// Create a new generic provider
    pub fn new(name: String, base_url: String, api_key: String) -> Self {
        let client = LlmClient::new()
            .expect("Failed to create HTTP client")
            .with_header("Authorization", format!("Bearer {}", api_key))
            .with_header("Content-Type", "application/json");

        Self {
            client,
            base_url,
            api_key,
            name,
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for GenericProvider {
    fn name(&self) -> &'static str {
        // Note: This is not ideal as we need to return a static str
        // In a real implementation, you might use a different approach
        "generic_provider"
    }

    async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);
        match self.client.get(&url).await {
            Ok(response) => Ok(response.status == 200),
            Err(_) => {
                // Try models endpoint as fallback
                let models_url = format!("{}/v1/models", self.base_url);
                match self.client.get(&models_url).await {
                    Ok(response) => Ok(response.status == 200),
                    Err(_) => Ok(false),
                }
            }
        }
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!("{}/v1/completions", self.base_url);
        
        let response = self.client.post_json(&url, serde_json::to_value(&request)?).await?;
        
        if response.status != 200 {
            anyhow::bail!("API request failed with status {}: {}", response.status, response.body);
        }

        let completion_response: CompletionResponse = serde_json::from_value(response.body)?;
        Ok(completion_response)
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        
        let response = self.client.post_json(&url, serde_json::to_value(&request)?).await?;
        
        if response.status != 200 {
            anyhow::bail!("API request failed with status {}: {}", response.status, response.body);
        }

        let chat_response: ChatResponse = serde_json::from_value(response.body)?;
        Ok(chat_response)
    }

    async fn get_models(&self) -> Result<Vec<ModelInfo>> {
        let url = format!("{}/v1/models", self.base_url);
        
        let response = self.client.get(&url).await?;
        
        if response.status != 200 {
            anyhow::bail!("API request failed with status {}: {}", response.status, response.body);
        }

        // Parse OpenAI-compatible models response
        let models_response: serde_json::Value = response.body;
        let models_array = models_response["data"].as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid models response format"))?;

        let mut models = Vec::new();
        for model in models_array {
            let id = model["id"].as_str().unwrap_or("unknown").to_string();
            let name = id.clone(); // Use ID as name for generic provider
            
            models.push(ModelInfo {
                id,
                name,
                description: None,
                context_window: None,
                max_output_tokens: None,
                capabilities: vec!["text-completion".to_string(), "chat".to_string()],
            });
        }

        Ok(models)
    }
}

/// Create a provider from configuration
pub fn create_provider(config: ProviderConfig) -> Result<Box<dyn LlmProvider>> {
    match config.provider_type.as_str() {
        "openai" => {
            let provider = GenericProvider::new(
                "OpenAI".to_string(),
                config.base_url.unwrap_or_else(|| "https://api.openai.com".to_string()),
                config.api_key,
            );
            Ok(Box::new(provider))
        }
        "anthropic" => {
            let provider = GenericProvider::new(
                "Anthropic".to_string(),
                config.base_url.unwrap_or_else(|| "https://api.anthropic.com".to_string()),
                config.api_key,
            );
            Ok(Box::new(provider))
        }
        "generic" | _ => {
            let provider = GenericProvider::new(
                config.name.unwrap_or_else(|| "Generic Provider".to_string()),
                config.base_url.ok_or_else(|| anyhow::anyhow!("base_url required for generic provider"))?,
                config.api_key,
            );
            Ok(Box::new(provider))
        }
    }
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type (openai, anthropic, generic)
    pub provider_type: String,
    /// Provider name
    pub name: Option<String>,
    /// API base URL
    pub base_url: Option<String>,
    /// API key
    pub api_key: String,
    /// Default model
    pub default_model: Option<String>,
    /// Additional configuration
    pub extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_config() {
        let config = ProviderConfig {
            provider_type: "openai".to_string(),
            name: Some("Test OpenAI".to_string()),
            base_url: Some("https://api.openai.com".to_string()),
            api_key: "test-key".to_string(),
            default_model: Some("gpt-3.5-turbo".to_string()),
            extra: HashMap::new(),
        };

        assert_eq!(config.provider_type, "openai");
        assert_eq!(config.api_key, "test-key");
    }

    #[tokio::test]
    async fn test_generic_provider_creation() {
        let provider = GenericProvider::new(
            "Test Provider".to_string(),
            "https://api.example.com".to_string(),
            "test-api-key".to_string(),
        );

        assert_eq!(provider.name(), "generic_provider");
        assert_eq!(provider.base_url, "https://api.example.com");
        assert_eq!(provider.api_key, "test-api-key");
    }
}