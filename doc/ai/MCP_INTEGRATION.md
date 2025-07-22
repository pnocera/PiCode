# PiCode MCP Integration Guide

This document covers PiCode's Model Context Protocol (MCP) integration, including both client usage of MCP servers and serving as an MCP server for other AI systems.

## Table of Contents

1. [MCP Overview](#mcp-overview)
2. [Using MCP Servers](#using-mcp-servers)
3. [PiCode as MCP Server](#picode-as-mcp-server)
4. [Protocol Implementation](#protocol-implementation)
5. [Configuration](#configuration)
6. [Development Guide](#development-guide)
7. [Troubleshooting](#troubleshooting)

## MCP Overview

The Model Context Protocol (MCP) is an open standard for connecting AI assistants to external tools and data sources. PiCode supports MCP in two ways:

1. **MCP Client**: Connect to external MCP servers to extend functionality
2. **MCP Server**: Serve PiCode's capabilities to other AI systems

### MCP Architecture in PiCode

```rust
pub struct MCPIntegration {
    client_manager: MCPClientManager,
    server: Option<MCPServer>,
    transport_layer: TransportLayer,
    protocol_handler: ProtocolHandler,
}

pub enum TransportType {
    Stdio,      // Standard I/O transport
    WebSocket,  // WebSocket transport  
    HTTP,       // HTTP transport (SSE + POST)
    InProcess,  // In-process for embedded usage
}
```

## Using MCP Servers

### Configuration

Configure MCP servers in your PiCode configuration:

```toml
[mcp.servers]
# File system server
[mcp.servers.filesystem]
command = "npx"
args = ["@modelcontextprotocol/server-filesystem", "/path/to/workspace"]
transport = "stdio"
enabled = true

# GitHub server  
[mcp.servers.github]
command = "npx"
args = ["@modelcontextprotocol/server-github"]
transport = "stdio"
env = { GITHUB_PERSONAL_ACCESS_TOKEN = "${GITHUB_TOKEN}" }
enabled = true

# Custom server
[mcp.servers.custom]
url = "ws://localhost:3001/mcp"
transport = "websocket"
auth_token = "${CUSTOM_SERVER_TOKEN}"
enabled = true

# Global MCP settings
[mcp.client]
timeout = 30
retry_attempts = 3
discovery_enabled = true
```

### Programmatic Configuration

```rust
use picode::mcp::{MCPClientManager, ServerConfig, TransportConfig};

let mut client_manager = MCPClientManager::new();

// Add filesystem server
let filesystem_config = ServerConfig {
    name: "filesystem".to_string(),
    transport: TransportConfig::Stdio {
        command: "npx".to_string(),
        args: vec![
            "@modelcontextprotocol/server-filesystem".to_string(),
            "/path/to/workspace".to_string(),
        ],
        env: HashMap::new(),
    },
    enabled: true,
    timeout: Duration::from_secs(30),
};

client_manager.add_server(filesystem_config).await?;
```

### Using MCP Tools

Once configured, MCP tools are available through the function calling system:

```rust
// List available MCP tools
let tools = mcp_client.list_tools().await?;
for tool in tools {
    println!("Tool: {} - {}", tool.name, tool.description.unwrap_or_default());
}

// Execute MCP tool
let result = mcp_client.call_tool("filesystem/read_file", json!({
    "path": "/path/to/file.txt"
})).await?;
```

### Slash Commands with MCP

MCP tools are automatically available as slash commands:

```
/mcp-list-tools                    # List all MCP tools
/mcp-filesystem-read-file /path/to/file.txt
/mcp-github-create-issue --repo owner/repo --title "Bug report"
/mcp-call-tool server:tool_name --param value
```

## PiCode as MCP Server

### Server Implementation

PiCode can serve as an MCP server, exposing its capabilities to other AI systems:

```rust
pub struct PiCodeMCPServer {
    workspace: Arc<WorkspaceManager>,
    tool_registry: ToolRegistry,
    resource_manager: ResourceManager,
    prompt_manager: PromptManager,
}

impl MCPServer for PiCodeMCPServer {
    async fn list_tools(&self) -> Result<Vec<Tool>> {
        Ok(vec![
            Tool {
                name: "picode_analyze".to_string(),
                description: Some("Analyze code structure and quality".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to analyze"
                        },
                        "depth": {
                            "type": "string",
                            "enum": ["shallow", "medium", "deep"],
                            "default": "medium"
                        }
                    },
                    "required": ["path"]
                }),
            },
            Tool {
                name: "picode_edit".to_string(),
                description: Some("AI-assisted code editing".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "file": {
                            "type": "string",
                            "description": "File to edit"
                        },
                        "instructions": {
                            "type": "string",
                            "description": "Editing instructions"
                        }
                    },
                    "required": ["file", "instructions"]
                }),
            },
            Tool {
                name: "picode_git_commit".to_string(),
                description: Some("Create intelligent Git commits".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "Commit message (optional, will be generated if not provided)"
                        },
                        "auto_stage": {
                            "type": "boolean",
                            "default": false
                        }
                    }
                }),
            },
        ])
    }
    
    async fn call_tool(&self, name: &str, arguments: serde_json::Value) -> Result<ToolResult> {
        match name {
            "picode_analyze" => self.handle_analyze_tool(arguments).await,
            "picode_edit" => self.handle_edit_tool(arguments).await,
            "picode_git_commit" => self.handle_git_commit_tool(arguments).await,
            _ => Err(Error::UnknownTool(name.to_string())),
        }
    }
    
    async fn list_resources(&self) -> Result<Vec<Resource>> {
        let mut resources = Vec::new();
        
        // Expose project files as resources
        let files = self.workspace.get_tracked_files().await?;
        for file in files {
            resources.push(Resource {
                uri: format!("file://{}", file.display()),
                name: file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                description: Some(format!("Project file: {}", file.display())),
                mime_type: self.detect_mime_type(&file),
            });
        }
        
        // Expose Git information as resources
        if let Ok(git_info) = self.workspace.get_git_info().await {
            resources.push(Resource {
                uri: "git://status".to_string(),
                name: "Git Status".to_string(),
                description: Some("Current Git repository status".to_string()),
                mime_type: Some("application/json".to_string()),
            });
        }
        
        Ok(resources)
    }
    
    async fn read_resource(&self, uri: &str) -> Result<ResourceContent> {
        if uri.starts_with("file://") {
            let path = uri.strip_prefix("file://").unwrap();
            let content = tokio::fs::read_to_string(path).await?;
            Ok(ResourceContent {
                uri: uri.to_string(),
                mime_type: self.detect_mime_type(Path::new(path)),
                text: Some(content),
                blob: None,
            })
        } else if uri == "git://status" {
            let git_status = self.workspace.get_git_status().await?;
            Ok(ResourceContent {
                uri: uri.to_string(),
                mime_type: Some("application/json".to_string()),
                text: Some(serde_json::to_string_pretty(&git_status)?),
                blob: None,
            })
        } else {
            Err(Error::ResourceNotFound(uri.to_string()))
        }
    }
}
```

### Starting the MCP Server

```bash
# Start as stdio server (for direct integration)
picode --mcp-server stdio

# Start as WebSocket server
picode --mcp-server websocket --mcp-port 3001

# Start as HTTP server with Server-Sent Events
picode --mcp-server http --mcp-port 3002

# Start with custom configuration
picode --mcp-server stdio --mcp-config /path/to/mcp-config.toml
```

### Server Configuration

```toml
[mcp.server]
enabled = true
transport = "stdio"  # or "websocket", "http"
port = 3001
host = "127.0.0.1"
auth_required = false
auth_token = "${MCP_AUTH_TOKEN}"

# Tool configuration
[mcp.server.tools]
enabled = ["analyze", "edit", "git_commit", "test", "format"]
disabled = []

# Resource configuration  
[mcp.server.resources]
expose_files = true
expose_git = true
file_patterns = ["src/**", "tests/**", "*.md"]
max_file_size = "1MB"

# Security settings
[mcp.server.security]
allowed_paths = ["./"]
denied_paths = [".git/", "target/", "node_modules/"]
sandbox_enabled = true
```

## Protocol Implementation

### Transport Layer

PiCode supports multiple MCP transport protocols:

#### Stdio Transport

```rust
pub struct StdioTransport {
    stdin: tokio::io::Stdin,
    stdout: tokio::io::Stdout,
    message_queue: Arc<Mutex<VecDeque<MCPMessage>>>,
}

impl Transport for StdioTransport {
    async fn send_message(&mut self, message: MCPMessage) -> Result<()> {
        let json = serde_json::to_string(&message)?;
        self.stdout.write_all(json.as_bytes()).await?;
        self.stdout.write_all(b"\n").await?;
        self.stdout.flush().await?;
        Ok(())
    }
    
    async fn receive_message(&mut self) -> Result<MCPMessage> {
        let mut buffer = String::new();
        self.stdin.read_line(&mut buffer).await?;
        let message: MCPMessage = serde_json::from_str(&buffer.trim())?;
        Ok(message)
    }
}
```

#### WebSocket Transport

```rust
pub struct WebSocketTransport {
    socket: WebSocketStream<TcpStream>,
}

impl Transport for WebSocketTransport {
    async fn send_message(&mut self, message: MCPMessage) -> Result<()> {
        let json = serde_json::to_string(&message)?;
        self.socket.send(Message::Text(json)).await?;
        Ok(())
    }
    
    async fn receive_message(&mut self) -> Result<MCPMessage> {
        while let Some(msg) = self.socket.next().await {
            match msg? {
                Message::Text(text) => {
                    let message: MCPMessage = serde_json::from_str(&text)?;
                    return Ok(message);
                },
                Message::Close(_) => return Err(Error::ConnectionClosed),
                _ => continue,
            }
        }
        Err(Error::ConnectionClosed)
    }
}
```

#### HTTP/SSE Transport

```rust
pub struct HTTPTransport {
    client: reqwest::Client,
    base_url: Url,
    event_source: Option<EventSource>,
}

impl Transport for HTTPTransport {
    async fn send_message(&mut self, message: MCPMessage) -> Result<()> {
        let response = self.client
            .post(&format!("{}/mcp", self.base_url))
            .json(&message)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(Error::HttpError(response.status()));
        }
        
        Ok(())
    }
    
    async fn receive_message(&mut self) -> Result<MCPMessage> {
        if let Some(event_source) = &mut self.event_source {
            while let Some(event) = event_source.next().await {
                if let Ok(event) = event {
                    if event.event_type == "message" {
                        let message: MCPMessage = serde_json::from_str(&event.data)?;
                        return Ok(message);
                    }
                }
            }
        }
        Err(Error::ConnectionClosed)
    }
}
```

### Message Handling

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "jsonrpc")]
pub enum MCPMessage {
    #[serde(rename = "2.0")]
    Request(MCPRequest),
    Response(MCPResponse),
    Notification(MCPNotification),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub id: serde_json::Value,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResponse {
    pub id: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub error: Option<MCPError>,
}

pub struct MCPHandler {
    transport: Box<dyn Transport>,
    tool_registry: ToolRegistry,
    resource_manager: ResourceManager,
    request_handlers: HashMap<String, Box<dyn RequestHandler>>,
}

impl MCPHandler {
    pub async fn handle_message(&mut self, message: MCPMessage) -> Result<Option<MCPMessage>> {
        match message {
            MCPMessage::Request(req) => {
                let response = self.handle_request(req).await;
                Ok(Some(MCPMessage::Response(response)))
            },
            MCPMessage::Response(resp) => {
                self.handle_response(resp).await?;
                Ok(None)
            },
            MCPMessage::Notification(notif) => {
                self.handle_notification(notif).await?;
                Ok(None)
            },
        }
    }
    
    async fn handle_request(&mut self, request: MCPRequest) -> MCPResponse {
        let result = match request.method.as_str() {
            "tools/list" => self.list_tools().await,
            "tools/call" => self.call_tool(request.params).await,
            "resources/list" => self.list_resources().await,
            "resources/read" => self.read_resource(request.params).await,
            "prompts/list" => self.list_prompts().await,
            "prompts/get" => self.get_prompt(request.params).await,
            _ => Err(Error::MethodNotFound(request.method.clone())),
        };
        
        match result {
            Ok(result) => MCPResponse {
                id: request.id,
                result: Some(result),
                error: None,
            },
            Err(error) => MCPResponse {
                id: request.id,
                result: None,
                error: Some(MCPError::from(error)),
            },
        }
    }
}
```

## Configuration

### Client Configuration

```toml
[mcp.client]
# Global client settings
timeout = 30
retry_attempts = 3
concurrent_connections = 10
discovery_enabled = true

# Server configurations
[mcp.servers.filesystem]
command = "npx"
args = ["@modelcontextprotocol/server-filesystem", "./workspace"]
transport = "stdio"
enabled = true
auto_restart = true
health_check_interval = 60

[mcp.servers.github]
command = "mcp-server-github"
transport = "stdio"
env = { GITHUB_TOKEN = "${GITHUB_TOKEN}" }
enabled = true

[mcp.servers.database]
url = "ws://localhost:3001/mcp"
transport = "websocket"
auth_token = "${DB_MCP_TOKEN}"
reconnect_attempts = 5
```

### Server Configuration

```toml
[mcp.server]
# Basic server settings
enabled = true
transport = "stdio"  # stdio, websocket, http
bind_address = "127.0.0.1"
port = 3001

# Authentication
auth_required = false
auth_token = "${MCP_AUTH_TOKEN}"
allowed_origins = ["*"]

# Capabilities
[mcp.server.capabilities]
tools = true
resources = true
prompts = true
experimental_features = []

# Tool configuration
[mcp.server.tools]
# Include all tools by default
include = ["*"]
# Exclude specific tools
exclude = ["dangerous_operation"]
# Custom tool timeout
timeout = 120

# Resource configuration
[mcp.server.resources]
# File system resources
expose_filesystem = true
allowed_paths = ["./workspace", "./docs"]
denied_paths = [".git", "target", "node_modules"]
max_file_size = "10MB"
follow_symlinks = false

# Git resources
expose_git = true
git_operations = ["status", "log", "diff"]

# Prompt configuration
[mcp.server.prompts]
enabled = true
template_directory = "./prompts"
dynamic_prompts = true
```

## Development Guide

### Creating MCP-Compatible Tools

```rust
use picode::mcp::{Tool, ToolHandler, ToolResult};

pub struct CustomTool;

#[async_trait]
impl ToolHandler for CustomTool {
    async fn execute(&self, args: serde_json::Value) -> Result<ToolResult> {
        // Tool implementation
        let input = args["input"].as_str()
            .ok_or_else(|| Error::InvalidArguments("Missing 'input' parameter".to_string()))?;
        
        // Process the input
        let result = process_input(input).await?;
        
        Ok(ToolResult {
            content: vec![
                ToolContent::Text {
                    text: result,
                    mime_type: Some("text/plain".to_string()),
                }
            ],
            is_error: false,
        })
    }
    
    fn schema(&self) -> Tool {
        Tool {
            name: "custom_tool".to_string(),
            description: Some("A custom tool for demonstration".to_string()),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "input": {
                        "type": "string",
                        "description": "Input to process"
                    }
                },
                "required": ["input"]
            }),
        }
    }
}
```

### Testing MCP Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_mcp_tool_execution() {
        let mut server = MCPTestServer::new().await;
        
        // List tools
        let tools = server.list_tools().await.unwrap();
        assert!(!tools.is_empty());
        
        // Execute a tool
        let result = server.call_tool("picode_analyze", json!({
            "path": "./test_file.rs"
        })).await.unwrap();
        
        assert!(!result.is_error);
        assert!(!result.content.is_empty());
    }
    
    #[tokio::test]
    async fn test_mcp_client_connection() {
        let client = MCPClient::connect("stdio", StdioConfig {
            command: "picode".to_string(),
            args: vec!["--mcp-server".to_string(), "stdio".to_string()],
        }).await.unwrap();
        
        let tools = client.list_tools().await.unwrap();
        assert!(tools.iter().any(|t| t.name == "picode_analyze"));
    }
}
```

### Custom Transport Implementation

```rust
pub struct CustomTransport {
    // Your transport implementation
}

#[async_trait]
impl Transport for CustomTransport {
    async fn send_message(&mut self, message: MCPMessage) -> Result<()> {
        // Implement message sending
        todo!()
    }
    
    async fn receive_message(&mut self) -> Result<MCPMessage> {
        // Implement message receiving
        todo!()
    }
    
    async fn close(&mut self) -> Result<()> {
        // Implement cleanup
        Ok(())
    }
}

// Register the transport
let transport_factory = Box::new(|config| {
    Box::new(CustomTransport::new(config)) as Box<dyn Transport>
});

transport_registry.register("custom", transport_factory);
```

## Troubleshooting

### Common Issues

#### Server Connection Failures

**Symptoms**: Cannot connect to MCP server, timeout errors

**Diagnostics**:
```bash
# Test server connectivity
picode mcp test-connection --server filesystem

# Check server logs
picode logs --filter mcp --server filesystem

# Verify server command
npx @modelcontextprotocol/server-filesystem --help
```

**Solutions**:
```toml
# Increase timeout
[mcp.servers.filesystem]
timeout = 60

# Enable auto-restart
auto_restart = true
restart_delay = 5

# Check server path
command = "/full/path/to/server"
```

#### Tool Execution Errors

**Symptoms**: Tools fail to execute, parameter validation errors

**Diagnostics**:
```bash
# List available tools
picode mcp list-tools --server github

# Test tool execution
picode mcp call-tool github:create_issue --dry-run --params '{"repo":"owner/repo","title":"Test"}'

# Validate tool schema
picode mcp validate-tool github:create_issue
```

**Solutions**:
- Check parameter types and required fields
- Verify authentication tokens
- Ensure proper permissions

#### Resource Access Issues

**Symptoms**: Cannot read resources, permission denied

**Diagnostics**:
```bash
# List available resources
picode mcp list-resources --server filesystem

# Test resource access
picode mcp read-resource file:///path/to/file --server filesystem
```

**Solutions**:
```toml
# Configure allowed paths
[mcp.servers.filesystem]
args = ["@modelcontextprotocol/server-filesystem", "/allowed/path"]

# Set proper permissions
env = { MCP_FILESYSTEM_ALLOWED_PATHS = "/workspace:/docs" }
```

#### Authentication Problems

**Symptoms**: Authentication failed, token errors

**Solutions**:
```bash
# Set environment variables
export GITHUB_TOKEN="your_token_here"
export MCP_AUTH_TOKEN="server_auth_token"

# Or in configuration
```

```toml
[mcp.servers.github]
env = { GITHUB_PERSONAL_ACCESS_TOKEN = "${GITHUB_TOKEN}" }

[mcp.server]
auth_token = "${MCP_AUTH_TOKEN}"
```

### Debug Mode

Enable detailed MCP debugging:

```bash
# Enable MCP debug logging
RUST_LOG=picode::mcp=debug picode

# Save MCP messages to file
picode --mcp-debug --mcp-log-file mcp-debug.log

# Test with verbose output
picode mcp test --verbose --server-name filesystem
```

### Performance Issues

**Large Resource Access**:
```toml
[mcp.server.resources]
max_file_size = "1MB"
cache_resources = true
cache_ttl = 300
```

**Too Many Concurrent Connections**:
```toml
[mcp.client]
concurrent_connections = 5
connection_pool_size = 10
```

**Slow Tool Execution**:
```toml
[mcp.server.tools]
timeout = 300
parallel_execution = true
max_concurrent_tools = 3
```

PiCode's MCP integration provides powerful extensibility while maintaining security and performance. Use MCP to connect with external tools and services, and expose PiCode's capabilities to other AI systems for enhanced collaborative development workflows.