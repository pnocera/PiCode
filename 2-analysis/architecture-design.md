# PiCode Architecture Design Document

## Executive Summary

PiCode is designed as a modular Rust CLI application that replicates core Claude Code functionality while supporting any OpenAPI-compatible LLM provider. The architecture leverages Zellij's terminal multiplexer capabilities as a foundation and compiles to WebAssembly for browser/MCP server deployment.

## High-Level Architecture

### Core Design Principles

1. **Modularity**: Clean separation between CLI, LLM integration, and core logic
2. **Extensibility**: Plugin-based architecture supporting custom hooks and tools
3. **Interoperability**: OpenAPI-first design for LLM provider flexibility
4. **Performance**: Rust's memory safety and speed with WASM compilation targets
5. **User Experience**: Terminal-native interface with interactive capabilities

### System Components Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     PiCode CLI Application                  │
├─────────────────────────────────────────────────────────────┤
│  CLI Interface Layer (clap-based argument parsing)          │
├─────────────────────────────────────────────────────────────┤
│  Interactive Mode Engine (Zellij-based terminal control)    │
├─────────────────────────────────────────────────────────────┤
│  Core Engine                                                │
│  ├─ Command Router                                          │
│  ├─ Session Manager                                         │
│  ├─ File System Operations                                  │
│  └─ Git Integration                                         │
├─────────────────────────────────────────────────────────────┤
│  LLM Integration Layer                                      │
│  ├─ OpenAPI Client                                         │
│  ├─ Provider Management                                     │
│  ├─ Tool Definition Generator                               │
│  └─ Response Handler                                        │
├─────────────────────────────────────────────────────────────┤
│  Plugin & Hooks System                                     │
│  ├─ WASM Plugin Runtime                                     │
│  ├─ Hook Registry                                           │
│  └─ Event System                                           │
├─────────────────────────────────────────────────────────────┤
│  WASM Compilation Target                                    │
│  ├─ MCP Server Interface                                    │
│  ├─ Browser Runtime Support                                 │
│  └─ Terminal WASM Runner                                    │
└─────────────────────────────────────────────────────────────┘
```

## Detailed Component Architecture

### 1. CLI Interface Layer

**Purpose**: Handle command-line argument parsing and initial routing

**Key Components**:
- `CLIParser`: Clap-based argument parser following Claude Code's command structure
- `CommandRouter`: Routes parsed commands to appropriate handlers
- `HelpSystem`: Comprehensive help and documentation system

**API Interface**:
```rust
pub struct CLIParser {
    pub fn parse_args() -> Result<PiCodeCommand, CLIError>;
    pub fn generate_help(command: Option<&str>) -> String;
}

pub enum PiCodeCommand {
    Interactive(InteractiveOptions),
    OneShot(OneShotCommand),
    Config(ConfigCommand),
    Hooks(HooksCommand),
}
```

### 2. Interactive Mode Engine

**Purpose**: Provide conversational interface with LLM integration

**Key Components**:
- `InteractiveSession`: Manages conversational state and history
- `TerminalInterface`: Zellij-based terminal control and rendering
- `SlashCommandProcessor`: Handles special slash commands
- `InputHandler`: Processes user input and commands

**Integration with Zellij**:
- Leverage Zellij's pane and tab management for multiple conversations
- Use Zellij's plugin system for UI components
- Adopt Zellij's layout system for complex interface arrangements

**API Interface**:
```rust
pub struct InteractiveSession {
    pub fn new(llm_provider: Box<dyn LLMProvider>) -> Self;
    pub fn start_session(&mut self) -> Result<(), SessionError>;
    pub fn process_input(&mut self, input: &str) -> Result<Response, ProcessError>;
    pub fn execute_slash_command(&mut self, command: SlashCommand) -> Result<(), CommandError>;
}
```

### 3. Core Engine

**Purpose**: Central business logic and file system operations

**Key Components**:
- `ProjectAnalyzer`: Code understanding and project structure analysis
- `FileManager`: Safe file system operations with permission management
- `GitIntegration`: Git operations (status, commit, branch management)
- `CodeEditor`: Code modification and refactoring capabilities

**Data Structures**:
```rust
pub struct ProjectContext {
    pub root_path: PathBuf,
    pub language: ProgrammingLanguage,
    pub dependencies: Vec<Dependency>,
    pub git_status: Option<GitStatus>,
}

pub struct FileOperation {
    pub operation_type: FileOpType,
    pub target_path: PathBuf,
    pub content: Option<String>,
    pub backup_created: bool,
}
```

### 4. LLM Integration Layer

**Purpose**: Manage OpenAPI-compatible LLM provider communications

**Key Components**:
- `OpenAPIClient`: Generic OpenAPI 3.0/3.1 client implementation
- `ProviderRegistry`: Registry of configured LLM providers
- `ToolGenerator`: Converts OpenAPI specs to LLM tool definitions
- `ResponseParser`: Handles and validates LLM responses

**OpenAPI Integration Strategy**:
1. **Dynamic Schema Loading**: Parse OpenAPI specifications at runtime
2. **Tool Generation**: Convert API endpoints to LLM-compatible function definitions
3. **Authentication Handling**: Support multiple auth methods (API keys, OAuth, bearer tokens)
4. **Provider Abstraction**: Common interface for different LLM providers

**API Interface**:
```rust
pub trait LLMProvider: Send + Sync {
    fn name(&self) -> &str;
    fn send_message(&self, message: &ChatMessage) -> Result<LLMResponse, LLMError>;
    fn list_models(&self) -> Result<Vec<ModelInfo>, LLMError>;
    fn get_capabilities(&self) -> ProviderCapabilities;
}

pub struct OpenAPILLMProvider {
    pub fn from_spec(spec: OpenAPISpec, config: ProviderConfig) -> Result<Self, ConfigError>;
    pub fn generate_tools(&self) -> Result<Vec<LLMTool>, ToolError>;
}
```

### 5. Plugin & Hooks System

**Purpose**: Extensible plugin architecture for custom functionality

**Key Components**:
- `WASMRuntime`: WebAssembly runtime for plugin execution
- `HookRegistry`: Manages hook registration and execution
- `EventBus`: Event-driven communication between components
- `PluginManager`: Plugin lifecycle management

**Hook Points** (inspired by Claude Code):
- `pre_file_edit`: Before file modifications
- `post_file_edit`: After file modifications
- `pre_git_commit`: Before git operations
- `post_git_commit`: After git operations
- `llm_request`: Before LLM API calls
- `llm_response`: After LLM responses

**API Interface**:
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn handle_hook(&self, hook: &str, data: &HookData) -> Result<HookResult, PluginError>;
}

pub struct PluginManager {
    pub fn load_plugin(&mut self, path: &Path) -> Result<PluginId, PluginError>;
    pub fn execute_hook(&self, hook: &str, data: HookData) -> Result<Vec<HookResult>, HookError>;
}
```

### 6. WASM Compilation Target

**Purpose**: Enable deployment as MCP server and browser execution

**Key Components**:
- `MCPServerInterface`: MCP protocol implementation for LLM integration
- `BrowserRuntime`: Browser-compatible runtime environment
- `WASMExports`: WebAssembly export interface

**MCP Server Architecture**:
- Expose PiCode functionality as MCP tools for other LLMs
- Handle MCP protocol communication (JSON-RPC over stdio/HTTP)
- Provide sandboxed file system access for security

**WASM Interface**:
```rust
#[wasm_bindgen]
pub struct PiCodeWASM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PiCodeWASM;
    
    #[wasm_bindgen]
    pub fn execute_command(&mut self, command: &str) -> Result<String, JsValue>;
    
    #[wasm_bindgen]
    pub fn start_interactive_session(&mut self) -> Result<(), JsValue>;
}
```

## Data Flow Architecture

### Command Processing Flow

1. **CLI Input** → CLIParser → CommandRouter
2. **Interactive Mode** → TerminalInterface → InteractiveSession
3. **LLM Communication** → LLMProvider → OpenAPIClient
4. **File Operations** → FileManager → HookRegistry (pre-hooks) → FileSystem → HookRegistry (post-hooks)
5. **Plugin Execution** → PluginManager → WASMRuntime → Plugin Code

### LLM Integration Flow

1. **User Input** → Context Analysis → Tool Selection
2. **API Call** → OpenAPIClient → LLM Provider
3. **Response Processing** → ResponseParser → Action Execution
4. **Result Display** → TerminalInterface → User

## Security Architecture

### File System Security
- **Permission Model**: Explicit user consent for file modifications
- **Backup System**: Automatic backup creation before modifications
- **Sandboxing**: Restrict plugin access to designated directories

### LLM Provider Security
- **Credential Management**: Secure storage of API keys and tokens
- **Request Validation**: Sanitize and validate all LLM requests
- **Response Filtering**: Filter potentially harmful LLM responses

### WASM Security
- **Capability-Based Security**: Limited API surface for WASM modules
- **Resource Limits**: Memory and execution time constraints
- **Network Isolation**: Controlled network access for browser deployment

## Compilation Targets and Deployment

### Native Binary
- **Target**: `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`, `x86_64-apple-darwin`
- **Features**: Full functionality including file system access and process spawning
- **Deployment**: Direct installation via package managers

### WebAssembly (Browser)
- **Target**: `wasm32-unknown-unknown`
- **Features**: Limited file system access, UI-focused functionality
- **Deployment**: Serve via HTTP with JavaScript glue code

### WebAssembly (MCP Server)
- **Target**: `wasm32-wasi`
- **Features**: Full CLI functionality with WASI system interface
- **Deployment**: Wasmtime runtime for MCP protocol communication

## Technology Stack

### Core Dependencies
- **CLI Parsing**: `clap` (v4.x) for argument parsing and help generation
- **Async Runtime**: `tokio` for asynchronous operations
- **HTTP Client**: `reqwest` for OpenAPI client implementation
- **JSON/YAML**: `serde_json`, `serde_yaml` for configuration and API communication
- **WASM Runtime**: `wasmtime` for plugin execution
- **Terminal Control**: Leverage Zellij's `termwiz` and terminal handling

### LLM Integration
- **OpenAPI**: `openapiv3` for specification parsing
- **HTTP Client**: `reqwest` with configurable authentication
- **JSON Schema**: `schemars` for tool definition generation

### WASM Compilation
- **WASM Bindgen**: `wasm-bindgen` for JavaScript interop
- **WASI**: `wasi` for system interface in WASM environments
- **Web Sys**: `web-sys` for browser APIs

## Integration with Zellij

### Leveraging Zellij Components
1. **Terminal Handling**: Use Zellij's robust terminal control and VT100 parsing
2. **Layout System**: Adopt Zellij's layout configuration for multi-pane interfaces
3. **Plugin Architecture**: Build on Zellij's WASM plugin infrastructure
4. **Session Management**: Leverage Zellij's session persistence and management

### Adaptation Strategy
- **Fork vs Integration**: Initially build as standalone application inspired by Zellij's architecture
- **Code Reuse**: Import relevant Zellij crates (`zellij-utils`, `zellij-tile`)
- **UI Components**: Create PiCode-specific UI components using Zellij's rendering framework

## Performance Considerations

### Memory Management
- **Rust Ownership**: Leverage Rust's zero-cost abstractions and memory safety
- **WASM Optimization**: Use `wee_alloc` for minimal memory allocator in WASM
- **Plugin Isolation**: Separate memory spaces for plugin execution

### Network Optimization
- **Connection Pooling**: Reuse HTTP connections for LLM providers
- **Request Caching**: Cache LLM responses where appropriate
- **Parallel Requests**: Concurrent API calls for tool execution

### Compilation Optimization
- **Link-Time Optimization**: Enable LTO for release builds
- **Code Generation**: Target-specific optimizations
- **Size Optimization**: Minimize WASM bundle size for browser deployment

This architecture provides a solid foundation for implementing PiCode with the flexibility to support multiple LLM providers while maintaining the user experience quality of Claude Code.