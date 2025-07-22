# PiCode AI Integration Documentation

This document provides comprehensive technical documentation for PiCode's AI integration architecture, designed for developers who need to understand, extend, or integrate with PiCode's AI capabilities.

## Table of Contents

1. [Integration Architecture](#integration-architecture)
2. [Provider System](#provider-system)
3. [OpenAPI Implementation](#openapi-implementation)
4. [Function Calling](#function-calling)
5. [Context Management](#context-management)
6. [Performance Optimization](#performance-optimization)
7. [Security Considerations](#security-considerations)
8. [Extending the System](#extending-the-system)

## Integration Architecture

### Overview

PiCode's AI integration is built on a provider-agnostic architecture that abstracts different LLM providers behind a common interface. This design allows seamless switching between providers and models while maintaining consistent functionality.

```rust
// Core AI integration components
pub struct AIIntegration {
    provider_registry: ProviderRegistry,
    active_provider: Box<dyn LLMProvider>,
    context_manager: ContextManager,
    function_registry: FunctionRegistry,
    response_processor: ResponseProcessor,
    cache_manager: CacheManager,
}

impl AIIntegration {
    pub async fn initialize(config: &AIConfig) -> Result<Self> {
        let mut registry = ProviderRegistry::new();
        
        // Register built-in providers
        Self::register_builtin_providers(&mut registry, config).await?;
        
        // Register custom providers from OpenAPI specs
        Self::register_custom_providers(&mut registry, config).await?;
        
        let active_provider = registry.get_provider(&config.default_provider)?;
        
        Ok(Self {
            provider_registry: registry,
            active_provider,
            context_manager: ContextManager::new(config)?,
            function_registry: FunctionRegistry::new(),
            response_processor: ResponseProcessor::new(),
            cache_manager: CacheManager::new(config)?,
        })
    }
}
```

### Provider Interface

All AI providers implement the standardized `LLMProvider` trait:

```rust
#[async_trait]
pub trait LLMProvider: Send + Sync + Debug {
    // Basic text completion
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    
    // Chat-based interactions
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse>;
    
    // Function calling capabilities
    async fn function_call(
        &self, 
        functions: Vec<FunctionDefinition>, 
        messages: Vec<ChatMessage>
    ) -> Result<FunctionCallResponse>;
    
    // Streaming responses
    async fn stream_completion(
        &self, 
        request: CompletionRequest
    ) -> Result<Pin<Box<dyn Stream<Item = Result<CompletionChunk>>>>>;
    
    // Provider capabilities
    fn capabilities(&self) -> ProviderCapabilities;
    fn max_context_tokens(&self) -> usize;
    fn supported_models(&self) -> Vec<String>;
    
    // Configuration
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn health_check(&self) -> HealthStatus;
}
```

### Provider Capabilities

Each provider exposes its capabilities through a standardized structure:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub supports_streaming: bool,
    pub supports_function_calling: bool,
    pub supports_vision: bool,
    pub supports_code_generation: bool,
    pub max_context_tokens: usize,
    pub max_output_tokens: usize,
    pub supported_languages: Vec<String>,
    pub rate_limits: RateLimits,
    pub pricing: Option<PricingInfo>,
}

#[derive(Debug, Clone)]
pub struct RateLimits {
    pub requests_per_minute: Option<u32>,
    pub tokens_per_minute: Option<u32>,
    pub concurrent_requests: Option<u32>,
}
```

## Provider System

### Built-in Providers

#### OpenAI Provider

```rust
pub struct OpenAIProvider {
    client: reqwest::Client,
    config: OpenAIConfig,
    api_key: String,
    base_url: Url,
    rate_limiter: RateLimiter,
}

impl OpenAIProvider {
    pub fn new(config: &OpenAIConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()?;
            
        Ok(Self {
            client,
            config: config.clone(),
            api_key: config.api_key.clone(),
            base_url: config.base_url.parse()?,
            rate_limiter: RateLimiter::new(
                config.rate_limit.requests_per_minute,
                config.rate_limit.tokens_per_minute
            ),
        })
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse> {
        self.rate_limiter.acquire().await?;
        
        let request_body = json!({
            "model": self.config.model,
            "messages": messages,
            "temperature": self.config.temperature,
            "max_tokens": self.config.max_tokens,
        });
        
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
            
        self.handle_response(response).await
    }
    
    async fn function_call(
        &self,
        functions: Vec<FunctionDefinition>,
        messages: Vec<ChatMessage>
    ) -> Result<FunctionCallResponse> {
        let tools: Vec<serde_json::Value> = functions
            .into_iter()
            .map(|f| json!({
                "type": "function",
                "function": {
                    "name": f.name,
                    "description": f.description,
                    "parameters": f.parameters
                }
            }))
            .collect();
            
        let request_body = json!({
            "model": self.config.model,
            "messages": messages,
            "tools": tools,
            "tool_choice": "auto"
        });
        
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;
            
        self.parse_function_call_response(response).await
    }
}
```

#### Anthropic Provider

```rust
pub struct AnthropicProvider {
    client: reqwest::Client,
    config: AnthropicConfig,
    api_key: String,
}

#[async_trait]
impl LLMProvider for AnthropicProvider {
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse> {
        // Convert messages to Anthropic format
        let (system_message, messages) = self.convert_messages(messages);
        
        let request_body = json!({
            "model": self.config.model,
            "max_tokens": self.config.max_tokens,
            "system": system_message,
            "messages": messages
        });
        
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&request_body)
            .send()
            .await?;
            
        self.handle_anthropic_response(response).await
    }
    
    fn convert_messages(&self, messages: Vec<ChatMessage>) -> (Option<String>, Vec<serde_json::Value>) {
        let mut system_message = None;
        let mut converted_messages = Vec::new();
        
        for message in messages {
            match message.role.as_str() {
                "system" => system_message = Some(message.content),
                "user" | "assistant" => {
                    converted_messages.push(json!({
                        "role": message.role,
                        "content": message.content
                    }));
                }
                _ => {} // Skip unknown roles
            }
        }
        
        (system_message, converted_messages)
    }
}
```

### Dynamic Provider Registration

#### Provider Registry

```rust
pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn LLMProvider>>,
    configurations: HashMap<String, ProviderConfig>,
    health_checker: HealthChecker,
}

impl ProviderRegistry {
    pub fn register<P>(&mut self, name: String, provider: P) -> Result<()>
    where
        P: LLMProvider + 'static,
    {
        // Validate provider
        let capabilities = provider.capabilities();
        self.validate_capabilities(&capabilities)?;
        
        // Health check
        match provider.health_check() {
            HealthStatus::Healthy => {},
            HealthStatus::Degraded(warning) => {
                tracing::warn!("Provider {} is degraded: {}", name, warning);
            },
            HealthStatus::Unhealthy(error) => {
                return Err(Error::ProviderUnhealthy { name, error });
            }
        }
        
        self.providers.insert(name.clone(), Box::new(provider));
        tracing::info!("Registered provider: {}", name);
        
        Ok(())
    }
    
    pub fn get_provider(&self, name: &str) -> Result<&Box<dyn LLMProvider>> {
        self.providers
            .get(name)
            .ok_or_else(|| Error::ProviderNotFound(name.to_string()))
    }
    
    pub async fn health_check_all(&self) -> HashMap<String, HealthStatus> {
        let mut results = HashMap::new();
        
        for (name, provider) in &self.providers {
            let status = provider.health_check();
            results.insert(name.clone(), status);
        }
        
        results
    }
}
```

## OpenAPI Implementation

### Specification Parsing

```rust
pub struct OpenAPIParser {
    spec_cache: Arc<RwLock<LruCache<String, OpenAPISpec>>>,
}

impl OpenAPIParser {
    pub async fn parse_specification(&self, spec_url: &str) -> Result<OpenAPISpec> {
        // Check cache first
        if let Some(cached_spec) = self.spec_cache.read().await.get(spec_url) {
            return Ok(cached_spec.clone());
        }
        
        // Fetch and parse specification
        let spec_content = if spec_url.starts_with("http") {
            self.fetch_remote_spec(spec_url).await?
        } else {
            tokio::fs::read_to_string(spec_url).await?
        };
        
        let spec: OpenAPISpec = if spec_url.ends_with(".yaml") || spec_url.ends_with(".yml") {
            serde_yaml::from_str(&spec_content)?
        } else {
            serde_json::from_str(&spec_content)?
        };
        
        // Validate specification
        self.validate_spec(&spec)?;
        
        // Cache the parsed spec
        self.spec_cache.write().await.put(spec_url.to_string(), spec.clone());
        
        Ok(spec)
    }
    
    fn validate_spec(&self, spec: &OpenAPISpec) -> Result<()> {
        // Check OpenAPI version
        let version = &spec.openapi;
        if !version.starts_with("3.0") && !version.starts_with("3.1") {
            return Err(Error::UnsupportedOpenAPIVersion(version.clone()));
        }
        
        // Validate required fields
        if spec.info.title.is_empty() {
            return Err(Error::InvalidOpenAPISpec("Missing title".to_string()));
        }
        
        if spec.servers.is_empty() {
            return Err(Error::InvalidOpenAPISpec("No servers defined".to_string()));
        }
        
        Ok(())
    }
}
```

### Dynamic Client Generation

```rust
pub struct OpenAPIProvider {
    client: reqwest::Client,
    spec: OpenAPISpec,
    base_url: Url,
    auth_handler: Box<dyn AuthHandler>,
    function_definitions: Vec<FunctionDefinition>,
}

impl OpenAPIProvider {
    pub async fn from_spec(name: &str, spec_url: &str, auth_config: AuthConfig) -> Result<Self> {
        let parser = OpenAPIParser::new();
        let spec = parser.parse_specification(spec_url).await?;
        
        // Extract base URL
        let base_url = spec.servers
            .first()
            .ok_or_else(|| Error::InvalidOpenAPISpec("No servers defined".to_string()))?
            .url
            .parse()?;
        
        // Create authentication handler
        let auth_handler = Self::create_auth_handler(&spec, auth_config)?;
        
        // Generate function definitions
        let function_definitions = Self::generate_function_definitions(&spec)?;
        
        Ok(Self {
            client: reqwest::Client::new(),
            spec,
            base_url,
            auth_handler,
            function_definitions,
        })
    }
    
    fn generate_function_definitions(spec: &OpenAPISpec) -> Result<Vec<FunctionDefinition>> {
        let mut functions = Vec::new();
        
        for (path, path_item) in &spec.paths {
            for (method, operation) in path_item.operations() {
                if let Some(operation) = operation {
                    let function = FunctionDefinition {
                        name: operation.operation_id
                            .clone()
                            .unwrap_or_else(|| format!("{}_{}", method, Self::sanitize_path(path))),
                        description: operation.summary
                            .clone()
                            .or_else(|| operation.description.clone()),
                        parameters: Self::extract_parameters(&operation.parameters, &operation.request_body)?,
                    };
                    
                    functions.push(function);
                }
            }
        }
        
        Ok(functions)
    }
    
    fn extract_parameters(
        parameters: &Option<Vec<openapiv3::ReferenceOr<openapiv3::Parameter>>>,
        request_body: &Option<openapiv3::ReferenceOr<openapiv3::RequestBody>>
    ) -> Result<serde_json::Value> {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();
        
        // Extract path and query parameters
        if let Some(params) = parameters {
            for param_ref in params {
                if let openapiv3::ReferenceOr::Item(param) = param_ref {
                    let param_schema = Self::parameter_to_json_schema(param)?;
                    properties.insert(param.name.clone(), param_schema);
                    
                    if param.required {
                        required.push(param.name.clone());
                    }
                }
            }
        }
        
        // Extract request body schema
        if let Some(openapiv3::ReferenceOr::Item(body)) = request_body {
            if let Some(content) = body.content.get("application/json") {
                if let Some(openapiv3::ReferenceOr::Item(media_type)) = body.content.get("application/json") {
                    if let Some(schema_ref) = &media_type.schema {
                        let body_schema = Self::schema_to_json_schema(schema_ref)?;
                        properties.insert("body".to_string(), body_schema);
                        required.push("body".to_string());
                    }
                }
            }
        }
        
        Ok(json!({
            "type": "object",
            "properties": properties,
            "required": required
        }))
    }
}

#[async_trait]
impl LLMProvider for OpenAPIProvider {
    async fn function_call(
        &self,
        functions: Vec<FunctionDefinition>,
        messages: Vec<ChatMessage>
    ) -> Result<FunctionCallResponse> {
        // Find completion endpoint in the OpenAPI spec
        let completion_endpoint = self.find_completion_endpoint()?;
        
        let request_body = json!({
            "messages": messages,
            "functions": functions,
            "function_call": "auto"
        });
        
        let mut request = self.client
            .post(&format!("{}{}", self.base_url, completion_endpoint))
            .json(&request_body);
        
        // Apply authentication
        request = self.auth_handler.apply_auth(request).await?;
        
        let response = request.send().await?;
        
        if !response.status().is_success() {
            return Err(Error::ProviderError {
                provider: self.name().to_string(),
                message: format!("HTTP {}: {}", response.status(), response.text().await?),
            });
        }
        
        let response_data: serde_json::Value = response.json().await?;
        self.parse_function_call_response(response_data)
    }
}
```

### Authentication Handlers

```rust
#[async_trait]
pub trait AuthHandler: Send + Sync {
    async fn apply_auth(&self, request: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder>;
    fn auth_type(&self) -> &str;
}

pub struct BearerTokenAuth {
    token: String,
}

#[async_trait]
impl AuthHandler for BearerTokenAuth {
    async fn apply_auth(&self, request: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder> {
        Ok(request.header("Authorization", format!("Bearer {}", self.token)))
    }
    
    fn auth_type(&self) -> &str {
        "bearer"
    }
}

pub struct ApiKeyAuth {
    key: String,
    header_name: String,
}

#[async_trait]
impl AuthHandler for ApiKeyAuth {
    async fn apply_auth(&self, request: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder> {
        Ok(request.header(&self.header_name, &self.key))
    }
    
    fn auth_type(&self) -> &str {
        "api_key"
    }
}

pub struct OAuth2Auth {
    client_id: String,
    client_secret: String,
    access_token: Arc<RwLock<Option<String>>>,
    token_url: String,
}

#[async_trait]
impl AuthHandler for OAuth2Auth {
    async fn apply_auth(&self, request: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder> {
        let token = {
            let token_guard = self.access_token.read().await;
            token_guard.clone()
        };
        
        let access_token = if let Some(token) = token {
            token
        } else {
            self.refresh_token().await?
        };
        
        Ok(request.header("Authorization", format!("Bearer {}", access_token)))
    }
    
    fn auth_type(&self) -> &str {
        "oauth2"
    }
}
```

## Function Calling

### Function Definition System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: serde_json::Value, // JSON Schema
}

pub struct FunctionRegistry {
    functions: HashMap<String, RegisteredFunction>,
    categories: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct RegisteredFunction {
    pub definition: FunctionDefinition,
    pub handler: Box<dyn FunctionHandler>,
    pub category: String,
    pub enabled: bool,
}

#[async_trait]
pub trait FunctionHandler: Send + Sync {
    async fn execute(&self, args: serde_json::Value) -> Result<FunctionResult>;
    fn validate_args(&self, args: &serde_json::Value) -> Result<()>;
}
```

### Built-in Functions

#### File System Functions

```rust
pub struct FileSystemFunctions;

impl FileSystemFunctions {
    pub fn register_all(registry: &mut FunctionRegistry) -> Result<()> {
        registry.register(
            "read_file",
            FunctionDefinition {
                name: "read_file".to_string(),
                description: Some("Read contents of a file".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["path"]
                }),
            },
            Box::new(ReadFileHandler),
            "filesystem"
        )?;
        
        registry.register(
            "write_file",
            FunctionDefinition {
                name: "write_file".to_string(),
                description: Some("Write content to a file".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to write"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content to write to the file"
                        },
                        "create_dirs": {
                            "type": "boolean",
                            "description": "Create parent directories if they don't exist",
                            "default": false
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
            Box::new(WriteFileHandler),
            "filesystem"
        )?;
        
        Ok(())
    }
}

pub struct ReadFileHandler;

#[async_trait]
impl FunctionHandler for ReadFileHandler {
    async fn execute(&self, args: serde_json::Value) -> Result<FunctionResult> {
        let path = args["path"].as_str()
            .ok_or_else(|| Error::InvalidFunctionArgs("Missing 'path' parameter".to_string()))?;
        
        // Security check
        if path.contains("..") || path.starts_with("/etc") || path.starts_with("/proc") {
            return Err(Error::SecurityViolation(format!("Access denied to path: {}", path)));
        }
        
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| Error::FileSystemError(e.to_string()))?;
        
        Ok(FunctionResult {
            success: true,
            data: json!({
                "content": content,
                "path": path,
                "size": content.len()
            }),
            message: Some(format!("Successfully read {} bytes from {}", content.len(), path)),
        })
    }
    
    fn validate_args(&self, args: &serde_json::Value) -> Result<()> {
        if !args["path"].is_string() {
            return Err(Error::InvalidFunctionArgs("'path' must be a string".to_string()));
        }
        Ok(())
    }
}
```

#### Git Functions

```rust
pub struct GitFunctions;

impl GitFunctions {
    pub fn register_all(registry: &mut FunctionRegistry) -> Result<()> {
        registry.register(
            "git_status",
            FunctionDefinition {
                name: "git_status".to_string(),
                description: Some("Get Git repository status".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "repository_path": {
                            "type": "string",
                            "description": "Path to Git repository",
                            "default": "."
                        }
                    }
                }),
            },
            Box::new(GitStatusHandler),
            "git"
        )?;
        
        registry.register(
            "git_commit",
            FunctionDefinition {
                name: "git_commit".to_string(),
                description: Some("Create a Git commit".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "Commit message"
                        },
                        "files": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Files to include in commit (empty for all staged)"
                        }
                    },
                    "required": ["message"]
                }),
            },
            Box::new(GitCommitHandler),
            "git"
        )?;
        
        Ok(())
    }
}
```

#### Code Analysis Functions

```rust
pub struct CodeAnalysisFunctions;

impl CodeAnalysisFunctions {
    pub fn register_all(registry: &mut FunctionRegistry) -> Result<()> {
        registry.register(
            "analyze_code",
            FunctionDefinition {
                name: "analyze_code".to_string(),
                description: Some("Analyze code quality and structure".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to analyze (file or directory)"
                        },
                        "language": {
                            "type": "string",
                            "description": "Programming language (auto-detected if not specified)"
                        },
                        "metrics": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Metrics to analyze",
                            "default": ["complexity", "maintainability", "test_coverage"]
                        }
                    },
                    "required": ["path"]
                }),
            },
            Box::new(CodeAnalysisHandler),
            "analysis"
        )?;
        
        Ok(())
    }
}
```

## Context Management

### Context Engine

```rust
pub struct ContextManager {
    workspace_analyzer: WorkspaceAnalyzer,
    file_index: FileIndex,
    symbol_table: SymbolTable,
    dependency_graph: DependencyGraph,
    context_cache: Arc<RwLock<LruCache<ContextKey, Context>>>,
    config: ContextConfig,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ContextKey {
    pub operation: String,
    pub files: Vec<PathBuf>,
    pub scope: ContextScope,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub files: Vec<FileContext>,
    pub symbols: Vec<Symbol>,
    pub dependencies: Vec<Dependency>,
    pub git_context: GitContext,
    pub project_metadata: ProjectMetadata,
    pub token_count: usize,
}

impl ContextManager {
    pub async fn get_context_for_operation(&self, operation: &str, files: &[PathBuf]) -> Result<Context> {
        let key = ContextKey {
            operation: operation.to_string(),
            files: files.to_vec(),
            scope: self.determine_scope(operation, files),
        };
        
        // Check cache first
        if let Some(cached_context) = self.context_cache.read().await.get(&key) {
            if !self.is_context_stale(&cached_context) {
                return Ok(cached_context.clone());
            }
        }
        
        // Build fresh context
        let context = self.build_context(&key).await?;
        
        // Cache the context
        self.context_cache.write().await.put(key, context.clone());
        
        Ok(context)
    }
    
    async fn build_context(&self, key: &ContextKey) -> Result<Context> {
        let mut context = Context::new();
        
        // Add file contexts
        for file_path in &key.files {
            if let Ok(file_context) = self.get_file_context(file_path).await {
                context.files.push(file_context);
            }
        }
        
        // Add related symbols
        let symbols = self.get_relevant_symbols(&key.files, &key.scope).await?;
        context.symbols.extend(symbols);
        
        // Add dependency information
        let dependencies = self.get_relevant_dependencies(&key.files).await?;
        context.dependencies.extend(dependencies);
        
        // Add Git context
        context.git_context = self.get_git_context().await?;
        
        // Add project metadata
        context.project_metadata = self.workspace_analyzer.get_project_metadata().await?;
        
        // Calculate token count
        context.token_count = self.calculate_token_count(&context);
        
        // Trim context if too large
        if context.token_count > self.config.max_context_tokens {
            context = self.trim_context(context, key).await?;
        }
        
        Ok(context)
    }
    
    async fn trim_context(&self, mut context: Context, key: &ContextKey) -> Result<Context> {
        // Priority-based trimming
        let target_tokens = self.config.max_context_tokens;
        
        // 1. Remove least relevant files
        context.files.sort_by(|a, b| {
            self.calculate_relevance(a, key).partial_cmp(&self.calculate_relevance(b, key))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        while context.token_count > target_tokens && context.files.len() > 1 {
            context.files.pop();
            context.token_count = self.calculate_token_count(&context);
        }
        
        // 2. Truncate large file contents
        for file_context in &mut context.files {
            if file_context.content.len() > self.config.max_file_content_tokens {
                file_context.content = self.smart_truncate(
                    &file_context.content,
                    self.config.max_file_content_tokens,
                    &key.operation
                );
            }
        }
        
        // 3. Remove less relevant symbols
        context.symbols.sort_by(|a, b| {
            b.relevance_score.partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        context.symbols.truncate(self.config.max_symbols);
        
        // Recalculate final token count
        context.token_count = self.calculate_token_count(&context);
        
        Ok(context)
    }
}
```

### Smart Context Selection

```rust
impl ContextManager {
    fn determine_scope(&self, operation: &str, files: &[PathBuf]) -> ContextScope {
        match operation {
            "edit" | "refactor" => {
                if files.len() == 1 {
                    ContextScope::File
                } else {
                    ContextScope::Module
                }
            },
            "analyze" | "review" => {
                if files.iter().any(|f| f.is_dir()) {
                    ContextScope::Project
                } else if files.len() > 5 {
                    ContextScope::Module
                } else {
                    ContextScope::File
                }
            },
            "test" => ContextScope::Module,
            "commit" | "push" => ContextScope::Repository,
            _ => ContextScope::File,
        }
    }
    
    async fn get_relevant_symbols(&self, files: &[PathBuf], scope: &ContextScope) -> Result<Vec<Symbol>> {
        let mut symbols = Vec::new();
        
        match scope {
            ContextScope::File => {
                for file in files {
                    symbols.extend(self.symbol_table.get_symbols_in_file(file));
                }
            },
            ContextScope::Module => {
                let modules = self.get_related_modules(files).await?;
                for module in modules {
                    symbols.extend(self.symbol_table.get_symbols_in_module(&module));
                }
            },
            ContextScope::Project => {
                symbols.extend(self.symbol_table.get_public_symbols());
            },
            ContextScope::Repository => {
                symbols.extend(self.symbol_table.get_all_symbols());
            },
        }
        
        // Sort by relevance
        symbols.sort_by(|a, b| {
            b.relevance_score.partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(symbols)
    }
    
    fn calculate_relevance(&self, file_context: &FileContext, key: &ContextKey) -> f32 {
        let mut relevance = 0.0;
        
        // Base relevance from file type
        relevance += match file_context.file_type {
            FileType::Source => 1.0,
            FileType::Test => 0.8,
            FileType::Configuration => 0.6,
            FileType::Documentation => 0.4,
            FileType::Other => 0.2,
        };
        
        // Boost relevance for files mentioned in operation
        if key.files.contains(&file_context.path) {
            relevance += 2.0;
        }
        
        // Boost relevance for recently modified files
        if let Some(last_modified) = file_context.last_modified {
            let days_since_modified = last_modified.elapsed().unwrap_or_default().as_secs() / 86400;
            relevance += (1.0 / (1.0 + days_since_modified as f32 * 0.1)).max(0.1);
        }
        
        // Boost relevance for files with many dependencies
        relevance += (file_context.dependency_count as f32 * 0.1).min(1.0);
        
        relevance
    }
}
```

## Performance Optimization

### Response Caching

```rust
pub struct ResponseCache {
    cache: Arc<RwLock<LruCache<CacheKey, CacheEntry>>>,
    config: CacheConfig,
}

#[derive(Hash, PartialEq, Eq)]
pub struct CacheKey {
    pub provider: String,
    pub model: String,
    pub messages_hash: u64,
    pub functions_hash: Option<u64>,
}

#[derive(Clone)]
pub struct CacheEntry {
    pub response: CacheableResponse,
    pub timestamp: Instant,
    pub hits: u64,
}

impl ResponseCache {
    pub async fn get_or_compute<F, Fut>(
        &self,
        key: CacheKey,
        compute_fn: F,
    ) -> Result<CacheableResponse>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<CacheableResponse>> + Send,
    {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(&key) {
                if entry.timestamp.elapsed() < self.config.ttl {
                    // Update hit count
                    let mut entry = entry.clone();
                    entry.hits += 1;
                    drop(cache);
                    self.cache.write().await.put(key, entry.clone());
                    
                    tracing::debug!("Cache hit for key: {:?}", key);
                    return Ok(entry.response);
                }
            }
        }
        
        // Compute if not cached or expired
        let response = compute_fn().await?;
        
        // Store in cache
        let entry = CacheEntry {
            response: response.clone(),
            timestamp: Instant::now(),
            hits: 1,
        };
        
        self.cache.write().await.put(key, entry);
        tracing::debug!("Cached new response");
        
        Ok(response)
    }
}
```

### Connection Pooling

```rust
pub struct ConnectionPool {
    pools: HashMap<String, Arc<reqwest::Client>>,
    config: PoolConfig,
}

impl ConnectionPool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            pools: HashMap::new(),
            config,
        }
    }
    
    pub fn get_client(&mut self, provider: &str) -> Arc<reqwest::Client> {
        self.pools.entry(provider.to_string())
            .or_insert_with(|| {
                Arc::new(
                    reqwest::Client::builder()
                        .timeout(Duration::from_secs(self.config.timeout))
                        .pool_max_idle_per_host(self.config.max_idle_per_host)
                        .pool_idle_timeout(Some(Duration::from_secs(self.config.idle_timeout)))
                        .build()
                        .expect("Failed to create HTTP client")
                )
            })
            .clone()
    }
}
```

### Rate Limiting

```rust
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    token_bucket: Arc<Mutex<TokenBucket>>,
    request_times: Arc<Mutex<VecDeque<Instant>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_requests));
        let token_bucket = Arc::new(Mutex::new(
            TokenBucket::new(config.tokens_per_minute, Duration::from_secs(60))
        ));
        
        Self {
            semaphore,
            token_bucket,
            request_times: Arc::new(Mutex::new(VecDeque::new())),
            config,
        }
    }
    
    pub async fn acquire(&self) -> Result<RateLimitGuard> {
        // Acquire concurrency permit
        let permit = self.semaphore.acquire().await?;
        
        // Check request rate
        {
            let mut request_times = self.request_times.lock().await;
            let now = Instant::now();
            
            // Remove old requests
            while let Some(&front_time) = request_times.front() {
                if now.duration_since(front_time) > Duration::from_secs(60) {
                    request_times.pop_front();
                } else {
                    break;
                }
            }
            
            // Check if we're under the rate limit
            if request_times.len() >= self.config.requests_per_minute {
                let oldest_time = request_times.front().unwrap();
                let wait_duration = Duration::from_secs(60) - now.duration_since(*oldest_time);
                
                drop(request_times);
                tokio::time::sleep(wait_duration).await;
            }
            
            request_times.push_back(now);
        }
        
        // Acquire tokens
        {
            let mut bucket = self.token_bucket.lock().await;
            bucket.acquire(1).await?;
        }
        
        Ok(RateLimitGuard { _permit: permit })
    }
}
```

## Security Considerations

### Input Validation

```rust
pub struct SecurityManager {
    validators: HashMap<String, Box<dyn InputValidator>>,
    sanitizers: HashMap<String, Box<dyn InputSanitizer>>,
}

#[async_trait]
pub trait InputValidator: Send + Sync {
    async fn validate(&self, input: &str) -> Result<ValidationResult>;
}

#[async_trait]
pub trait InputSanitizer: Send + Sync {
    async fn sanitize(&self, input: &str) -> Result<String>;
}

pub struct PathValidator;

#[async_trait]
impl InputValidator for PathValidator {
    async fn validate(&self, input: &str) -> Result<ValidationResult> {
        // Check for path traversal
        if input.contains("..") {
            return Ok(ValidationResult::Invalid("Path traversal detected".to_string()));
        }
        
        // Check for absolute paths to sensitive directories
        let sensitive_paths = ["/etc", "/proc", "/sys", "/dev", "/root"];
        for sensitive in &sensitive_paths {
            if input.starts_with(sensitive) {
                return Ok(ValidationResult::Invalid(format!("Access to {} is forbidden", sensitive)));
            }
        }
        
        // Validate path exists and is accessible
        if !Path::new(input).exists() {
            return Ok(ValidationResult::Invalid("Path does not exist".to_string()));
        }
        
        Ok(ValidationResult::Valid)
    }
}
```

### API Key Protection

```rust
pub struct SecureCredentialStore {
    keyring: keyring::Entry,
    encryption_key: [u8; 32],
}

impl SecureCredentialStore {
    pub fn new(service_name: &str) -> Result<Self> {
        let keyring = keyring::Entry::new(service_name, "picode")?;
        let encryption_key = Self::derive_encryption_key()?;
        
        Ok(Self {
            keyring,
            encryption_key,
        })
    }
    
    pub async fn store_credential(&self, key: &str, value: &str) -> Result<()> {
        let encrypted_value = self.encrypt(value)?;
        let credential_data = CredentialData {
            key: key.to_string(),
            encrypted_value,
            created_at: SystemTime::now(),
        };
        
        let serialized = serde_json::to_string(&credential_data)?;
        self.keyring.set_password(&serialized)?;
        
        tracing::info!("Stored credential for key: {}", key);
        Ok(())
    }
    
    pub async fn retrieve_credential(&self, key: &str) -> Result<Option<String>> {
        match self.keyring.get_password() {
            Ok(serialized) => {
                let credential_data: CredentialData = serde_json::from_str(&serialized)?;
                if credential_data.key == key {
                    let decrypted = self.decrypt(&credential_data.encrypted_value)?;
                    Ok(Some(decrypted))
                } else {
                    Ok(None)
                }
            },
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(Error::CredentialStoreError(e.to_string())),
        }
    }
    
    fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>> {
        // Use AES-256-GCM for encryption
        use aes_gcm::{Aes256Gcm, Key, Nonce, aead::Aead};
        use aes_gcm::aead::KeyInit;
        
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.encryption_key));
        let nonce = Nonce::from_slice(&[0u8; 12]); // In practice, use a random nonce
        
        cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| Error::EncryptionError(e.to_string()))
    }
    
    fn decrypt(&self, ciphertext: &[u8]) -> Result<String> {
        use aes_gcm::{Aes256Gcm, Key, Nonce, aead::Aead};
        use aes_gcm::aead::KeyInit;
        
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&self.encryption_key));
        let nonce = Nonce::from_slice(&[0u8; 12]);
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| Error::DecryptionError(e.to_string()))?;
            
        String::from_utf8(plaintext)
            .map_err(|e| Error::DecryptionError(e.to_string()))
    }
}
```

## Extending the System

### Adding New Providers

To add a new AI provider:

1. **Implement the LLMProvider trait**:

```rust
pub struct MyCustomProvider {
    client: reqwest::Client,
    config: MyCustomConfig,
}

#[async_trait]
impl LLMProvider for MyCustomProvider {
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse> {
        // Implementation specific to your provider
        unimplemented!()
    }
    
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supports_streaming: true,
            supports_function_calling: true,
            max_context_tokens: 128000,
            // ... other capabilities
        }
    }
    
    // Implement other required methods
}
```

2. **Register the provider**:

```rust
let provider = MyCustomProvider::new(config)?;
registry.register("my_provider", provider)?;
```

### Creating Custom Functions

To add new function capabilities:

```rust
pub struct MyCustomFunction;

#[async_trait]
impl FunctionHandler for MyCustomFunction {
    async fn execute(&self, args: serde_json::Value) -> Result<FunctionResult> {
        // Your custom function logic
        Ok(FunctionResult {
            success: true,
            data: json!({"result": "custom_result"}),
            message: Some("Custom function executed".to_string()),
        })
    }
    
    fn validate_args(&self, args: &serde_json::Value) -> Result<()> {
        // Validate function arguments
        Ok(())
    }
}

// Register the function
registry.register(
    "my_custom_function",
    FunctionDefinition {
        name: "my_custom_function".to_string(),
        description: Some("My custom function".to_string()),
        parameters: json!({
            "type": "object",
            "properties": {
                "param1": {"type": "string"},
                "param2": {"type": "number"}
            },
            "required": ["param1"]
        }),
    },
    Box::new(MyCustomFunction),
    "custom"
)?;
```

This comprehensive AI integration documentation provides the foundation for understanding and extending PiCode's AI capabilities. The modular architecture ensures that new providers and functions can be easily added while maintaining compatibility with the existing system.