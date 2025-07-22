# PiCode WebAssembly Compilation Strategy

## Overview

PiCode's WebAssembly compilation strategy enables deployment as both an MCP (Model Context Protocol) server for LLM integration and as a browser-executable application. This dual-target approach maximizes the utility of PiCode across different execution environments while maintaining security and performance.

## Compilation Targets

### 1. Native Binary (`x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`, `x86_64-apple-darwin`)

**Features**:
- Full file system access with permission management
- Process spawning for git operations and external tools
- Terminal control via Zellij integration
- Plugin loading from file system
- Comprehensive CLI functionality

**Build Configuration**:
```toml
[target.'cfg(not(target_arch = "wasm32"))']
dependencies = [
    "tokio",
    "zellij-client",
    "zellij-server", 
    "zellij-utils",
    "clap",
    "reqwest",
    "wasmtime"
]
```

### 2. WASM32-WASI (MCP Server Target)

**Purpose**: Enable PiCode to run as an MCP server tool for other LLMs

**Features**:
- WASI (WebAssembly System Interface) for file system access
- stdio-based MCP protocol communication
- Sandboxed execution environment
- Limited process spawning capabilities

**Build Configuration**:
```toml
[target.'cfg(all(target_arch = "wasm32", target_os = "wasi"))']
dependencies = [
    "wasi",
    "tokio-wasi",
    "serde_json"
]

[dependencies.reqwest]
version = "0.11"
default-features = false
features = ["json", "rustls-tls"]
```

### 3. WASM32-Unknown-Unknown (Browser Target)

**Purpose**: Browser-executable version with limited capabilities

**Features**:
- JavaScript interoperability via `wasm-bindgen`
- Browser-based HTTP client using fetch API
- Local storage for session persistence
- UI rendering via HTML/CSS/JavaScript

**Build Configuration**:
```toml
[target.'cfg(all(target_arch = "wasm32", not(target_os = "wasi")))']
dependencies = [
    "wasm-bindgen",
    "web-sys",
    "js-sys",
    "wasm-bindgen-futures"
]
```

## MCP Server Architecture

### Protocol Implementation

The MCP server interface follows the JSON-RPC 2.0 specification for communication:

```rust
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
pub mod mcp_server {
    use serde_json::{json, Value};
    use std::io::{self, BufRead, Write};
    
    pub struct MCPServer {
        picode: PiCodeCore,
        request_id: u64,
    }
    
    impl MCPServer {
        pub fn new() -> Self {
            Self {
                picode: PiCodeCore::new(),
                request_id: 0,
            }
        }
        
        pub async fn run(&mut self) -> Result<(), MCPError> {
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            
            for line in stdin.lock().lines() {
                let request = line?;
                let response = self.handle_request(&request).await?;
                writeln!(stdout, "{}", response)?;
                stdout.flush()?;
            }
            
            Ok(())
        }
        
        async fn handle_request(&mut self, request: &str) -> Result<String, MCPError> {
            let req: JsonRpcRequest = serde_json::from_str(request)?;
            
            let result = match req.method.as_str() {
                "initialize" => self.initialize(req.params).await,
                "tools/list" => self.list_tools().await,
                "tools/call" => self.call_tool(req.params).await,
                "prompts/list" => self.list_prompts().await,
                "resources/list" => self.list_resources().await,
                _ => return Err(MCPError::MethodNotFound(req.method)),
            };
            
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: result.ok(),
                error: result.err().map(|e| JsonRpcError::from(e)),
            };
            
            Ok(serde_json::to_string(&response)?)
        }
    }
}
```

### Tool Definitions for MCP

PiCode exposes its functionality as MCP tools:

```rust
pub fn get_mcp_tools() -> Vec<MCPTool> {
    vec![
        MCPTool {
            name: "picode_edit_file".to_string(),
            description: "Edit a file with AI assistance".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "file_path": {"type": "string"},
                    "instructions": {"type": "string"},
                    "backup": {"type": "boolean", "default": true}
                },
                "required": ["file_path", "instructions"]
            }),
        },
        MCPTool {
            name: "picode_analyze_project".to_string(),
            description: "Analyze project structure and dependencies".to_string(),
            input_schema: json!({
                "type": "object", 
                "properties": {
                    "project_path": {"type": "string"},
                    "include_deps": {"type": "boolean", "default": true}
                },
                "required": ["project_path"]
            }),
        },
        MCPTool {
            name: "picode_git_operations".to_string(),
            description: "Perform git operations (status, commit, branch)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "operation": {"type": "string", "enum": ["status", "commit", "branch", "diff"]},
                    "args": {"type": "array", "items": {"type": "string"}}
                },
                "required": ["operation"]
            }),
        },
    ]
}
```

## Browser Deployment Architecture

### JavaScript Interoperability

```rust
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PiCodeBrowser {
    core: PiCodeCore,
    session: Option<InteractiveSession>,
}

#[wasm_bindgen]
impl PiCodeBrowser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        
        Self {
            core: PiCodeCore::new(),
            session: None,
        }
    }
    
    #[wasm_bindgen]
    pub async fn initialize(&mut self, config: &str) -> Result<(), JsValue> {
        let config: BrowserConfig = serde_json::from_str(config)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.core.initialize_browser(config).await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(())
    }
    
    #[wasm_bindgen]
    pub async fn start_interactive_session(&mut self) -> Result<String, JsValue> {
        let session = self.core.create_interactive_session().await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let session_id = session.id().to_string();
        self.session = Some(session);
        Ok(session_id)
    }
    
    #[wasm_bindgen]
    pub async fn send_message(&mut self, message: &str) -> Result<String, JsValue> {
        if let Some(ref mut session) = self.session {
            let response = session.process_message(message).await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
            Ok(serde_json::to_string(&response)
                .map_err(|e| JsValue::from_str(&e.to_string()))?)
        } else {
            Err(JsValue::from_str("No active session"))
        }
    }
}
```

### Browser Integration Files

**index.html**:
```html
<!DOCTYPE html>
<html>
<head>
    <title>PiCode Web Interface</title>
    <meta charset="utf-8">
    <style>
        /* Terminal-style interface styling */
    </style>
</head>
<body>
    <div id="picode-container">
        <div id="terminal"></div>
        <div id="input-area">
            <input type="text" id="command-input" placeholder="Enter command or message...">
            <button id="send-btn">Send</button>
        </div>
    </div>
    
    <script src="./picode_bg.js"></script>
    <script src="./picode.js"></script>
    <script>
        // Initialize PiCode browser interface
        import('./picode.js').then(module => {
            const picode = new module.PiCodeBrowser();
            // Setup event handlers and UI
        });
    </script>
</body>
</html>
```

## Build Pipeline Configuration

### Cargo Configuration

```toml
# Cargo.toml
[package]
name = "picode"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

# Native binary target
[[bin]]
name = "picode"
path = "src/main.rs"
required-features = ["native"]

[features]
default = ["native"]
native = ["clap", "tokio", "zellij-client"]
wasm = ["wasm-bindgen", "web-sys", "js-sys"]
mcp-server = ["wasi", "tokio-wasi"]

[dependencies]
# Core dependencies for all targets
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"

# Conditional dependencies
clap = { version = "4.0", optional = true }
tokio = { version = "1.0", optional = true, features = ["full"] }
wasm-bindgen = { version = "0.2", optional = true }
web-sys = { version = "0.3", optional = true }
js-sys = { version = "0.3", optional = true }
wasi = { version = "0.11", optional = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen-futures = "0.4"
console_error_panic_hook = "0.1"

[dependencies.reqwest]
version = "0.11"
default-features = false
features = ["json", "rustls-tls"]
```

### Build Scripts

**build_all.sh**:
```bash
#!/bin/bash

# Build native binaries
echo "Building native binaries..."
cargo build --release --features native

# Build WASM MCP server
echo "Building WASM MCP server..."
cargo build --release --target wasm32-wasi --features mcp-server

# Build browser WASM
echo "Building browser WASM..."
wasm-pack build --target web --features wasm --out-dir pkg

# Copy assets
echo "Copying web assets..."
cp web/index.html pkg/
cp web/style.css pkg/

echo "Build complete!"
```

## Security Considerations

### WASM Sandbox Security

1. **Memory Isolation**: WASM modules run in isolated memory spaces
2. **Capability-Based Security**: Limited API surface through WASI imports
3. **Network Restrictions**: Browser WASM limited by CORS policies
4. **File System Access**: Sandboxed file system access in WASI environments

### MCP Security Model

```rust
pub struct MCPSecurityContext {
    pub allowed_paths: HashSet<PathBuf>,
    pub allowed_operations: HashSet<MCPOperation>,
    pub rate_limits: RateLimits,
}

impl MCPSecurityContext {
    pub fn validate_request(&self, request: &MCPRequest) -> Result<(), SecurityError> {
        // Validate path access
        if let Some(path) = request.get_file_path() {
            if !self.is_path_allowed(&path) {
                return Err(SecurityError::PathNotAllowed(path));
            }
        }
        
        // Check operation permissions
        if !self.allowed_operations.contains(&request.operation) {
            return Err(SecurityError::OperationNotAllowed(request.operation));
        }
        
        // Apply rate limiting
        self.rate_limits.check_and_update(&request.client_id)?;
        
        Ok(())
    }
}
```

## Performance Optimization

### WASM Size Optimization

```toml
[profile.release]
# Enable link-time optimization
lto = true
# Strip debug info
strip = true
# Optimize for size
opt-level = "z"
# Enable additional optimizations
codegen-units = 1
panic = "abort"

[profile.release.package."*"]
opt-level = "z"
```

### Runtime Optimization

1. **Memory Management**: Use `wee_alloc` for minimal memory allocator in WASM
2. **Code Splitting**: Lazy loading of components in browser environment
3. **Request Batching**: Batch multiple API calls to reduce latency
4. **Caching**: Implement response caching for repeated operations

## Deployment Strategies

### MCP Server Deployment

```yaml
# Docker deployment for MCP server
FROM scratch
COPY target/wasm32-wasi/release/picode.wasm /picode.wasm
ENTRYPOINT ["wasmtime", "/picode.wasm"]
```

### Browser Deployment

```json
{
  "scripts": {
    "build": "wasm-pack build --target web --features wasm",
    "serve": "python -m http.server 8000 --directory pkg",
    "deploy": "rsync -av pkg/ user@server:/var/www/picode/"
  }
}
```

### CDN Integration

```javascript
// Load PiCode from CDN
const PICODE_CDN = 'https://cdn.picode.dev/v1.0.0/';

async function loadPiCode() {
    const module = await import(`${PICODE_CDN}picode.js`);
    return new module.PiCodeBrowser();
}
```

This comprehensive WASM compilation strategy provides multiple deployment targets while maintaining security, performance, and functionality across all environments.