# PiCode Component Specifications

## Component 1: CLI Interface Layer

### Purpose and Responsibilities
The CLI Interface Layer serves as the primary entry point for PiCode, handling command-line argument parsing, validation, and initial routing to appropriate system components. This layer replicates Claude Code's CLI reference functionality while providing extensibility for OpenAPI-compatible LLM providers.

### Input/Output
- **Input**: Command-line arguments, environment variables, configuration files
- **Output**: Parsed command structures, help documentation, error messages
- **Side Effects**: Configuration loading, logging initialization, environment validation

### Key Data Structures

```rust
// Main command structure
#[derive(Debug, Clone, Parser)]
#[clap(name = "picode", version = env!("CARGO_PKG_VERSION"))]
pub struct CLIArgs {
    #[clap(subcommand)]
    pub command: Option<Command>,
    
    #[clap(short, long, global = true)]
    pub config: Option<PathBuf>,
    
    #[clap(short, long, global = true)]
    pub provider: Option<String>,
    
    #[clap(short, long, global = true)]
    pub model: Option<String>,
    
    #[clap(short, long, global = true)]
    pub debug: bool,
}

// Command enumeration matching Claude Code functionality
#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Interactive mode for conversational coding
    Interactive(InteractiveCommand),
    
    /// Execute single commands
    Run(RunCommand),
    
    /// Configuration management
    Config(ConfigCommand),
    
    /// Hooks and plugin management
    Hooks(HooksCommand),
    
    /// Session management
    Session(SessionCommand),
    
    /// Provider management
    Provider(ProviderCommand),
}

// Interactive command options
#[derive(Debug, Clone, Args)]
pub struct InteractiveCommand {
    #[clap(short, long)]
    pub project: Option<PathBuf>,
    
    #[clap(short, long)]
    pub context: Vec<PathBuf>,
    
    #[clap(long)]
    pub no_git: bool,
    
    #[clap(long)]
    pub read_only: bool,
}

// Configuration for command execution
pub struct CommandConfig {
    pub provider_config: ProviderConfig,
    pub file_permissions: FilePermissions,
    pub git_integration: bool,
    pub hook_registry: HookRegistry,
}
```

### Core Logic/Algorithms

1. **Argument Parsing Pipeline**:
   ```rust
   fn parse_and_validate() -> Result<CommandConfig, CLIError> {
       let args = CLIArgs::parse();
       let config = load_config(args.config.as_ref())?;
       let provider = resolve_provider(&args, &config)?;
       validate_permissions(&args)?;
       Ok(CommandConfig::new(args, config, provider))
   }
   ```

2. **Command Routing Logic**:
   ```rust
   fn route_command(config: CommandConfig) -> Result<(), ExecutionError> {
       match config.command {
           Command::Interactive(opts) => start_interactive_mode(opts, config),
           Command::Run(cmd) => execute_single_command(cmd, config),
           Command::Config(cfg) => handle_configuration(cfg),
           Command::Hooks(hooks) => manage_hooks(hooks),
           Command::Session(session) => handle_session(session),
           Command::Provider(provider) => manage_provider(provider),
       }
   }
   ```

### API/Interface Definitions

```rust
pub trait CLIHandler {
    type Output;
    type Error: std::error::Error;
    
    fn handle_command(&self, command: Command, config: &CommandConfig) -> Result<Self::Output, Self::Error>;
    fn get_help(&self) -> String;
    fn validate_args(&self, args: &CLIArgs) -> Result<(), ValidationError>;
}

pub struct CLIRouter {
    handlers: HashMap<CommandType, Box<dyn CLIHandler>>,
}

impl CLIRouter {
    pub fn new() -> Self;
    pub fn register_handler<H: CLIHandler + 'static>(&mut self, command_type: CommandType, handler: H);
    pub fn route(&self, config: CommandConfig) -> Result<(), RoutingError>;
}
```

### Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum CLIError {
    #[error("Invalid command arguments: {0}")]
    InvalidArguments(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),
    
    #[error("Provider error: {0}")]
    ProviderError(#[from] ProviderError),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),
}

pub type CLIResult<T> = Result<T, CLIError>;
```

## Component 2: Interactive Mode Engine

### Purpose and Responsibilities
The Interactive Mode Engine provides a conversational interface between users and LLM providers, managing session state, terminal interaction, and command execution. This component replicates Claude Code's interactive mode while supporting multiple LLM providers through OpenAPI integration.

### Input/Output
- **Input**: User text input, slash commands, file paths, terminal events
- **Output**: Formatted LLM responses, command results, file modifications, terminal rendering
- **Side Effects**: File system modifications, git operations, plugin execution

### Key Data Structures

```rust
// Session state management
pub struct InteractiveSession {
    pub session_id: Uuid,
    pub project_context: ProjectContext,
    pub conversation_history: Vec<ChatMessage>,
    pub llm_provider: Box<dyn LLMProvider>,
    pub terminal_interface: TerminalInterface,
    pub slash_processor: SlashCommandProcessor,
    pub hook_registry: Arc<HookRegistry>,
}

// Message types for conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: MessageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

// Slash command system
#[derive(Debug, Clone)]
pub enum SlashCommand {
    Edit { file: PathBuf, line: Option<u32> },
    Create { file: PathBuf, template: Option<String> },
    Delete { file: PathBuf },
    Git { operation: GitOperation },
    Search { query: String, scope: SearchScope },
    Run { command: String, args: Vec<String> },
    Help { command: Option<String> },
    Context { add: Vec<PathBuf>, remove: Vec<PathBuf> },
    Reset,
    Save { session_name: Option<String> },
    Load { session_name: String },
}

// Terminal interface abstraction
pub struct TerminalInterface {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub input_handler: InputHandler,
    pub renderer: InteractiveRenderer,
    pub layout: Layout,
}
```

### Core Logic/Algorithms

1. **Interactive Session Loop**:
   ```rust
   async fn run_interactive_loop(&mut self) -> Result<(), SessionError> {
       self.display_welcome();
       
       loop {
           match self.get_user_input().await? {
               Input::Text(text) => {
                   let response = self.process_text_input(text).await?;
                   self.display_response(response).await?;
               },
               Input::SlashCommand(cmd) => {
                   let result = self.execute_slash_command(cmd).await?;
                   self.display_command_result(result).await?;
               },
               Input::Exit => break,
               Input::Interrupt => self.handle_interrupt()?,
           }
           
           self.update_context()?;
       }
       
       Ok(())
   }
   ```

2. **LLM Communication Flow**:
   ```rust
   async fn process_text_input(&mut self, input: String) -> Result<LLMResponse, SessionError> {
       // Add user message to history
       self.add_message(ChatMessage::user(input.clone()));
       
       // Execute pre-llm hooks
       self.hook_registry.execute_hook("pre_llm_request", &input).await?;
       
       // Prepare context and tools
       let context = self.prepare_context()?;
       let tools = self.get_available_tools()?;
       
       // Send to LLM
       let response = self.llm_provider.send_message(
           &self.conversation_history,
           tools,
           context
       ).await?;
       
       // Execute post-llm hooks
       self.hook_registry.execute_hook("post_llm_response", &response).await?;
       
       // Add response to history
       self.add_message(ChatMessage::assistant(response.content.clone()));
       
       Ok(response)
   }
   ```

### Integration Points
- **Zellij Terminal Control**: Leverage Zellij's terminal handling for robust input/output
- **Plugin System**: Execute hooks and custom plugins during session lifecycle
- **File System**: Safe file operations with permission checking and backup creation
- **Git Integration**: Seamless git operations with status tracking

### API/Interface Definitions

```rust
pub trait InteractiveEngine {
    async fn start_session(&mut self, options: InteractiveOptions) -> Result<(), SessionError>;
    async fn process_input(&mut self, input: &str) -> Result<Response, ProcessError>;
    async fn execute_command(&mut self, command: SlashCommand) -> Result<CommandResult, CommandError>;
    fn get_session_state(&self) -> &SessionState;
    fn save_session(&self, path: &Path) -> Result<(), SaveError>;
    fn load_session(&mut self, path: &Path) -> Result<(), LoadError>;
}

pub struct InteractiveMode {
    session: InteractiveSession,
    config: InteractiveConfig,
}

impl InteractiveEngine for InteractiveMode {
    // Implementation details...
}
```

## Component 3: LLM Integration Layer

### Purpose and Responsibilities
The LLM Integration Layer provides a unified interface for communicating with any OpenAPI-compatible LLM provider. This component handles API communication, tool definition generation, authentication, and response processing while abstracting provider-specific details.

### Input/Output
- **Input**: Chat messages, tool definitions, provider configurations, OpenAPI specifications
- **Output**: LLM responses, generated tools, provider capabilities, error information
- **Side Effects**: HTTP API calls, authentication token refresh, usage tracking

### Key Data Structures

```rust
// Provider abstraction
#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn send_message(
        &self,
        messages: &[ChatMessage],
        tools: Option<&[LLMTool]>,
        context: Option<&RequestContext>
    ) -> Result<LLMResponse, LLMError>;
    
    async fn list_models(&self) -> Result<Vec<ModelInfo>, LLMError>;
    fn get_capabilities(&self) -> &ProviderCapabilities;
    fn get_name(&self) -> &str;
}

// OpenAPI-based provider implementation
pub struct OpenAPIProvider {
    pub client: reqwest::Client,
    pub base_url: Url,
    pub auth: AuthenticationConfig,
    pub spec: OpenAPISpec,
    pub model_config: ModelConfig,
    pub tools: Vec<LLMTool>,
}

// Tool definition for LLM function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMTool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub required: Vec<String>,
    pub endpoint: Option<EndpointInfo>,
}

// Authentication configuration
#[derive(Debug, Clone)]
pub enum AuthenticationConfig {
    ApiKey { header_name: String, key: String },
    Bearer { token: String },
    OAuth { client_id: String, client_secret: String, token_url: String },
    Custom { headers: HashMap<String, String> },
}

// Response handling
#[derive(Debug, Clone)]
pub struct LLMResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub usage: Option<UsageInfo>,
    pub model: String,
    pub timestamp: DateTime<Utc>,
}
```

### Core Logic/Algorithms

1. **OpenAPI Tool Generation**:
   ```rust
   impl OpenAPIProvider {
       pub fn generate_tools_from_spec(&self, spec: &OpenAPISpec) -> Result<Vec<LLMTool>, ToolError> {
           let mut tools = Vec::new();
           
           for (path, path_item) in &spec.paths {
               for (method, operation) in path_item.operations() {
                   let tool = self.convert_operation_to_tool(path, method, operation)?;
                   tools.push(tool);
               }
           }
           
           self.validate_tools(&tools)?;
           Ok(tools)
       }
       
       fn convert_operation_to_tool(&self, path: &str, method: &str, op: &Operation) -> Result<LLMTool, ToolError> {
           let parameters = self.extract_parameters(op)?;
           let schema = self.generate_json_schema(&parameters)?;
           
           Ok(LLMTool {
               name: self.generate_tool_name(path, method, op),
               description: op.description.clone().unwrap_or_default(),
               parameters: schema,
               required: self.extract_required_params(&parameters),
               endpoint: Some(EndpointInfo::new(path, method)),
           })
       }
   }
   ```

2. **Request Processing Pipeline**:
   ```rust
   async fn send_message(&self, messages: &[ChatMessage], tools: Option<&[LLMTool]>, context: Option<&RequestContext>) -> Result<LLMResponse, LLMError> {
       // Build request payload
       let payload = self.build_request_payload(messages, tools, context)?;
       
       // Add authentication
       let request = self.client
           .post(&self.chat_endpoint())
           .json(&payload);
       let request = self.add_authentication(request)?;
       
       // Execute request with retry logic
       let response = self.execute_with_retry(request).await?;
       
       // Parse and validate response
       let llm_response = self.parse_response(response).await?;
       self.validate_response(&llm_response)?;
       
       Ok(llm_response)
   }
   ```

### Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("API request failed: {status} - {message}")]
    APIError { status: u16, message: String },
    
    #[error("Invalid OpenAPI specification: {0}")]
    InvalidSpec(String),
    
    #[error("Tool generation failed: {0}")]
    ToolGenerationError(String),
    
    #[error("Response parsing failed: {0}")]
    ResponseParsingError(#[from] serde_json::Error),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
```

### WASM Compilation Considerations
- Use `wasm-bindgen` for browser compatibility
- Implement WASM-compatible HTTP client using `web-sys` fetch API
- Handle CORS restrictions in browser environments
- Optimize bundle size with conditional compilation

## Component 4: File System Operations Manager

### Purpose and Responsibilities
The File System Operations Manager provides safe, auditable file system operations with permission checking, backup creation, and rollback capabilities. This component ensures all file modifications are tracked and can be undone while maintaining security boundaries.

### Key Data Structures

```rust
pub struct FileManager {
    pub permissions: FilePermissions,
    pub backup_manager: BackupManager,
    pub operation_log: OperationLog,
    pub hook_registry: Arc<HookRegistry>,
}

#[derive(Debug, Clone)]
pub struct FileOperation {
    pub id: Uuid,
    pub operation_type: FileOpType,
    pub target_path: PathBuf,
    pub content: Option<String>,
    pub backup_path: Option<PathBuf>,
    pub timestamp: DateTime<Utc>,
    pub status: OperationStatus,
}

#[derive(Debug, Clone)]
pub enum FileOpType {
    Create,
    Read,
    Update,
    Delete,
    Move { from: PathBuf, to: PathBuf },
    Copy { from: PathBuf, to: PathBuf },
}
```

### Core Logic/Algorithms

```rust
impl FileManager {
    pub async fn execute_operation(&mut self, operation: FileOperation) -> Result<OperationResult, FileError> {
        // Pre-operation validation and hooks
        self.validate_permissions(&operation)?;
        self.hook_registry.execute_hook("pre_file_operation", &operation).await?;
        
        // Create backup if needed
        let backup_path = if operation.requires_backup() {
            Some(self.backup_manager.create_backup(&operation.target_path).await?)
        } else {
            None
        };
        
        // Execute the operation
        let result = match operation.operation_type {
            FileOpType::Create => self.create_file(&operation).await,
            FileOpType::Update => self.update_file(&operation).await,
            FileOpType::Delete => self.delete_file(&operation).await,
            // ... other operations
        };
        
        // Log operation and execute post-hooks
        self.operation_log.record(operation.clone(), &result);
        self.hook_registry.execute_hook("post_file_operation", &operation).await?;
        
        result
    }
}
```

## Component 5: Plugin & Hooks System

### Purpose and Responsibilities
The Plugin & Hooks System provides extensibility through WASM-based plugins and a comprehensive hook system. This component allows custom functionality to be injected at various points in the application lifecycle while maintaining security and isolation.

### Key Data Structures

```rust
pub struct PluginManager {
    pub plugins: HashMap<PluginId, LoadedPlugin>,
    pub hook_registry: HookRegistry,
    pub runtime: WASMRuntime,
}

pub struct LoadedPlugin {
    pub id: PluginId,
    pub metadata: PluginMetadata,
    pub instance: wasmtime::Instance,
    pub exports: PluginExports,
}

pub struct HookRegistry {
    pub hooks: HashMap<String, Vec<HookHandler>>,
    pub execution_order: HashMap<String, Vec<PluginId>>,
}

#[derive(Debug, Clone)]
pub enum HookHandler {
    Native(Box<dyn Fn(&HookData) -> Result<HookResult, HookError> + Send + Sync>),
    Plugin { plugin_id: PluginId, function_name: String },
}
```

This comprehensive component specification provides the foundation for implementing PiCode with clear interfaces, robust error handling, and extensible architecture that can accommodate various LLM providers while maintaining security and performance requirements.