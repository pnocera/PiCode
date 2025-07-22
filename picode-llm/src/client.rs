use anyhow::Result;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;

/// HTTP client for LLM providers
#[derive(Debug, Clone)]
pub struct LlmClient {
    client: Client,
    timeout_duration: Duration,
    default_headers: HashMap<String, String>,
}

/// Request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestConfig {
    /// API endpoint URL
    pub url: String,
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Request body
    pub body: Option<serde_json::Value>,
}

/// Response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body as JSON
    pub body: serde_json::Value,
    /// Response time in milliseconds
    pub response_time_ms: u128,
}

/// Client errors
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Request timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("Invalid JSON response: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Invalid URL: {url}")]
    InvalidUrl { url: String },
    
    #[error("Authentication failed: {message}")]
    AuthenticationError { message: String },
    
    #[error("Rate limit exceeded: retry after {retry_after_seconds}s")]
    RateLimitError { retry_after_seconds: u64 },
}

impl LlmClient {
    /// Create a new LLM client
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("PiCode/0.1.0")
            .build()?;

        Ok(Self {
            client,
            timeout_duration: Duration::from_secs(30),
            default_headers: HashMap::new(),
        })
    }

    /// Set default timeout for requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout_duration = timeout;
        self
    }

    /// Add a default header
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    /// Add multiple default headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.default_headers.extend(headers);
        self
    }

    /// Execute a request
    pub async fn execute(&self, config: RequestConfig) -> Result<LlmResponse, ClientError> {
        let start_time = std::time::Instant::now();

        // Build request
        let mut request = match config.method.to_uppercase().as_str() {
            "GET" => self.client.get(&config.url),
            "POST" => self.client.post(&config.url),
            "PUT" => self.client.put(&config.url),
            "DELETE" => self.client.delete(&config.url),
            "PATCH" => self.client.patch(&config.url),
            _ => return Err(ClientError::InvalidUrl { url: config.url }),
        };

        // Add default headers
        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }

        // Add request-specific headers
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }

        // Add body if present
        if let Some(body) = &config.body {
            request = request.json(body);
        }

        // Set timeout
        let timeout_duration = config
            .timeout_seconds
            .map(Duration::from_secs)
            .unwrap_or(self.timeout_duration);

        // Execute request with timeout
        let response = timeout(timeout_duration, request.send()).await
            .map_err(|_| ClientError::Timeout {
                timeout_ms: timeout_duration.as_millis() as u64,
            })?
            .map_err(ClientError::HttpError)?;

        let response_time_ms = start_time.elapsed().as_millis();

        // Handle common HTTP errors
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(ClientError::AuthenticationError {
                message: "Invalid API key or authentication failed".to_string(),
            });
        }

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            
            return Err(ClientError::RateLimitError {
                retry_after_seconds: retry_after,
            });
        }

        // Extract headers
        let mut response_headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                response_headers.insert(name.to_string(), value_str.to_string());
            }
        }

        // Parse response body
        let body_text = response.text().await.map_err(ClientError::HttpError)?;
        let body: serde_json::Value = if body_text.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_str(&body_text).map_err(ClientError::JsonError)?
        };

        Ok(LlmResponse {
            status: status.as_u16(),
            headers: response_headers,
            body,
            response_time_ms,
        })
    }

    /// Convenience method for GET requests
    pub async fn get(&self, url: &str) -> Result<LlmResponse, ClientError> {
        self.execute(RequestConfig {
            url: url.to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            timeout_seconds: None,
            body: None,
        }).await
    }

    /// Convenience method for POST requests with JSON body
    pub async fn post_json(&self, url: &str, body: serde_json::Value) -> Result<LlmResponse, ClientError> {
        self.execute(RequestConfig {
            url: url.to_string(),
            method: "POST".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers
            },
            timeout_seconds: None,
            body: Some(body),
        }).await
    }
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default LLM client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = LlmClient::new().expect("Should create client");
        assert_eq!(client.timeout_duration, Duration::from_secs(30));
        assert!(client.default_headers.is_empty());
    }

    #[test]
    fn test_client_configuration() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let client = LlmClient::new()
            .expect("Should create client")
            .with_timeout(Duration::from_secs(60))
            .with_headers(headers);

        assert_eq!(client.timeout_duration, Duration::from_secs(60));
        assert_eq!(client.default_headers.len(), 1);
        assert_eq!(
            client.default_headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }
}