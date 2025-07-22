# AI System Context

> 🤖 **Structured System Information** - Machine-readable context for AI systems

This section provides structured, parseable information about PiCode's capabilities, constraints, and integration patterns for AI systems and language models.

## System Context Overview

PiCode is a universal LLM-powered development assistant that replicates Claude Code's functionality while supporting any OpenAPI-compatible language model provider. The system is built in Rust with Zellij integration and compiles to both native binaries and WebAssembly.

## Quick Reference

### 📊 [System Overview](system-overview.md)
High-level system description and metadata
```yaml
system:
  name: "PiCode"
  type: "development_assistant"
  language: "rust"
  architecture: "modular_plugin_based"
  deployment_targets: ["native", "wasm", "container"]
```

### ⚡ [Capabilities Matrix](capabilities.md)
Complete feature and capability listing
```yaml
capabilities:
  cli_interface: true
  interactive_mode: true
  multi_llm_support: true
  plugin_system: true
  zellij_integration: true
  wasm_compilation: true
```

### ⚠️ [Limitations & Constraints](limitations.md)
Known system boundaries and constraints
```yaml
limitations:
  max_context_size: "provider_dependent"
  concurrent_requests: 10
  plugin_memory_limit: "64MB"
  supported_platforms: ["linux", "macos", "windows"]
```

### 🎯 [Use Case Patterns](use-cases.md)
Common usage scenarios and patterns
```yaml
primary_use_cases:
  - "code_review_automation"
  - "interactive_development"
  - "codebase_analysis"
  - "documentation_generation"
  - "debugging_assistance"
```

## AI Integration Patterns

### Direct API Integration
For systems that interact with PiCode's REST API:
```yaml
integration_type: "rest_api"
protocol: "http"
authentication: "bearer_token"
data_format: "json"
streaming_support: true
rate_limits:
  requests_per_minute: 100
  concurrent_connections: 10
```

### Plugin Integration
For AI systems running as PiCode plugins:
```yaml
integration_type: "wasm_plugin"
runtime: "wasmtime"
interface: "wasi"
sandbox: "limited_host_access"
permissions:
  - "file_read"
  - "network_outbound"
  - "environment_read"
memory_limit: "64MB"
execution_timeout: "30s"
```

### MCP Server Integration
For Model Context Protocol server implementations:
```yaml
integration_type: "mcp_server"
transport: ["stdio", "http", "websocket"]
version: "mcp-1.0"
capabilities:
  - "tools"
  - "resources" 
  - "prompts"
features:
  - "streaming_responses"
  - "context_management"
  - "session_persistence"
```

## System Metadata

### Document Metadata Standards
All AI context documents include:
```yaml
metadata:
  version: "1.0.0"
  schema_version: "ai-context-v1"
  last_updated: "2024-01-15T10:30:00Z"
  target_audience: ["llm", "ai_assistant", "automation"]
  content_type: "system_specification"
  complexity_level: "intermediate"
  prerequisites: ["rust_knowledge", "cli_familiarity"]
  estimated_reading_time: "5min"
  estimated_tokens: 2500
```

### Content Classification
```yaml
classification:
  domain: "software_development"
  subdomain: "development_tools"
  technology_stack: ["rust", "webassembly", "openapi"]
  integration_category: "llm_tooling"
  security_level: "standard"
  maturity_level: "alpha"
```

## Structured Data Formats

### System Configuration Schema
```yaml
# System configuration structure
configuration:
  general:
    provider: string           # LLM provider identifier
    model: string             # Model name
    temperature: float        # Response creativity (0.0-1.0)
    max_tokens: integer       # Maximum response tokens
  
  providers:
    openai:
      api_key_env: string     # Environment variable name
      base_url: string        # API endpoint URL
      timeout: integer        # Request timeout (seconds)
    
    anthropic:
      api_key_env: string
      base_url: string
      timeout: integer
  
  interface:
    interactive_mode: boolean  # Enable interactive mode
    auto_context: boolean     # Automatic context detection
    syntax_highlighting: boolean # Code syntax highlighting
```

### Context Definition Schema
```yaml
# Project context structure
context:
  project:
    name: string             # Project name
    language: string         # Primary programming language
    type: string            # Project type (cli, web-api, library)
    
  include_patterns:          # Files/directories to include
    - "src/**/*.rs"
    - "Cargo.toml"
    - "README.md"
    
  exclude_patterns:          # Files/directories to exclude  
    - "target/"
    - "*.lock"
    - ".git/"
    
  custom_contexts:           # Custom context definitions
    architecture:
      description: string
      files: array
      priority: integer
```

### Response Format Specification
```yaml
# Expected response structure
response_format:
  success:
    status: "success"
    data:
      type: string           # Response data type
      content: any           # Response content
      metadata:
        tokens_used: integer
        processing_time: float
        model_used: string
        
  error:
    status: "error" 
    error:
      code: string           # Error code
      message: string        # Human-readable message
      details: object        # Additional error details
      suggestions: array     # Potential solutions
```

## Performance Characteristics

### Latency Expectations
```yaml
performance_metrics:
  cold_start: "2-5s"        # Initial startup time
  warm_request: "100-500ms" # Subsequent request processing
  context_loading: "50-200ms" # Project context loading
  llm_request: "1-10s"      # Provider-dependent
  plugin_execution: "10-100ms" # WASM plugin overhead
```

### Resource Usage
```yaml
resource_requirements:
  memory:
    minimum: "128MB"
    recommended: "512MB"
    with_large_context: "1GB+"
  
  disk_space:
    binary_size: "50MB"
    cache_storage: "100MB"
    logs_and_data: "variable"
  
  network:
    api_requests: "provider_dependent"
    bandwidth: "1-10MB/request"
```

## Error Handling Patterns

### Error Categories
```yaml
error_categories:
  system_errors:
    range: "100-199"
    examples:
      - "101: Configuration file not found"
      - "102: Invalid configuration format"
      
  provider_errors:
    range: "300-399" 
    examples:
      - "301: Authentication failed"
      - "302: Rate limit exceeded"
      
  plugin_errors:
    range: "400-499"
    examples:
      - "401: Plugin load failed"
      - "402: Plugin execution timeout"
```

### Recovery Strategies
```yaml
recovery_patterns:
  configuration_errors:
    strategy: "fallback_to_defaults"
    retry: false
    user_action_required: true
    
  provider_errors:
    strategy: "exponential_backoff"
    retry: true
    max_retries: 3
    fallback_provider: "local"
    
  plugin_errors:
    strategy: "disable_plugin"
    retry: false
    continue_without_plugin: true
```

## Integration Guidelines

### For LLM Systems
1. **Context Loading**: Use `system-overview.md` for initial system understanding
2. **Capability Discovery**: Reference `capabilities.md` for available functions  
3. **Error Handling**: Check `limitations.md` for known constraints
4. **Task Execution**: Use structured formats for consistent interaction

### For AI Development
1. **Training Data**: Analysis documents provide rich technical context
2. **Fine-tuning**: Use prompt templates for domain-specific adaptation
3. **Evaluation**: Performance metrics enable benchmarking
4. **Integration**: Specifications enable automated system integration

---

**Next Steps**: Explore [System Overview](system-overview.md) or review [Technical Specifications](../specifications/index.md)