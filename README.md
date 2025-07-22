# PiCode - Universal AI Terminal Workspace

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.84+-orange.svg)](https://www.rust-lang.org)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-enabled-blue.svg)](https://webassembly.org/)
[![OpenAPI](https://img.shields.io/badge/OpenAPI-3.x-green.svg)](https://swagger.io/specification/)

PiCode is a powerful terminal-native AI workspace that brings Claude Code's capabilities to any OpenAPI-compatible Large Language Model provider. Built in Rust with WebAssembly compilation support, it offers a developer-centric experience that seamlessly integrates into existing workflows.

## ✨ Key Features

### 🤖 Universal AI Integration
- **Any OpenAPI LLM**: Works with OpenAI, Anthropic, Cohere, Google AI Studio, HuggingFace, and more
- **Dynamic Configuration**: Switch between providers and models effortlessly
- **Function Calling**: Advanced tool support for complex development tasks
- **Streaming Responses**: Real-time AI interactions with minimal latency

### 🏗️ Zellij Foundation
- **Terminal Multiplexer**: Built on Zellij's robust terminal management
- **Multi-Pane Interface**: Organize your workspace with flexible layouts
- **Plugin Ecosystem**: Extend functionality through WebAssembly plugins
- **Session Persistence**: Detachable sessions that survive disconnects

### ⚡ Developer Experience
- **Terminal-Native**: No context switching from your preferred environment
- **Project Awareness**: Deep understanding of codebases and file structures
- **Git Integration**: Seamless version control operations
- **Interactive Mode**: Conversational AI assistance with slash commands

### 🌐 WebAssembly Deployment
- **MCP Server**: Serve as a tool for other AI systems
- **Browser Support**: Run directly in web browsers
- **Cross-Platform**: Single binary that works everywhere
- **Performance**: Near-native speed with memory safety

## 🚀 Quick Start

### Installation

#### From Releases (Recommended)
```bash
# Download latest release for your platform
curl -L https://github.com/pnocera/PiCode/releases/latest/download/picode-linux-x64.tar.gz | tar xz
sudo mv picode /usr/local/bin/
```

#### From Source
```bash
# Prerequisites: Rust 1.84+, Git
git clone https://github.com/pnocera/PiCode.git
cd PiCode
cargo install --path .
```

#### WebAssembly
```bash
# Install with WASM support
cargo install --path . --features wasm
```

### Basic Usage

#### 1. Configure Your LLM Provider
```bash
# OpenAI
picode config set provider openai
picode config set api_key YOUR_OPENAI_KEY
picode config set model gpt-4

# Anthropic Claude
picode config set provider anthropic
picode config set api_key YOUR_ANTHROPIC_KEY
picode config set model claude-3-sonnet-20241022

# Custom OpenAPI provider
picode config set provider custom
picode config set api_url https://api.your-provider.com/v1
picode config set api_key YOUR_API_KEY
```

#### 2. Start Interactive Mode
```bash
# Launch PiCode workspace
picode

# Or start with a specific project
picode --project /path/to/your/project
```

#### 3. Use Slash Commands
```
/help          - Show all available commands
/analyze       - Analyze current codebase
/edit <file>   - AI-assisted code editing
/test          - Generate and run tests
/commit        - Generate commit messages
/search <query> - Intelligent code search
```

### Example Session

```bash
$ picode
PiCode v0.1.0 - Terminal AI Workspace
Connected to: claude-3-sonnet-20241022

┌─ Project: my-rust-app ─┐  ┌─ AI Assistant ──────┐
│ src/                   │  │ Hi! I'm ready to     │
│ ├─ main.rs            │  │ help you with your   │
│ ├─ lib.rs             │  │ Rust project. Type   │
│ └─ tests/             │  │ /help for commands.  │
│     └─ integration.rs │  │                      │
└─ Cargo.toml           ┘  └──────────────────────┘

> /analyze src/main.rs

Analyzing src/main.rs...
✓ Rust syntax valid
✓ No obvious bugs detected
⚠ Consider adding error handling on line 23
ℹ Suggestion: Extract HTTP client to separate module

> /edit src/main.rs --add-error-handling

I'll add proper error handling to your main.rs file...
[AI proceeds to show code changes and apply them]
```

## 📁 Project Structure

```
picode/
├─ src/                    # Main application code
├─ picode-core/           # Core terminal workspace logic
├─ picode-cli/            # Command-line interface
├─ picode-llm/            # OpenAPI LLM integration
├─ picode-hooks/          # Extension system
├─ picode-wasm/           # WebAssembly bindings
└─ doc/                   # Documentation
   ├─ user/              # User guides
   ├─ developer/         # Development documentation
   └─ ai/               # AI reference documentation
```

## 🔧 Configuration

PiCode supports multiple configuration sources with the following precedence:

1. Command-line arguments
2. Environment variables
3. Configuration files
4. Defaults

### Configuration File Locations

- **Linux/macOS**: `~/.config/picode/config.toml`
- **Windows**: `%APPDATA%\picode\config.toml`
- **Project**: `./picode.toml` (project-specific overrides)

### Example Configuration

```toml
[provider]
name = "openai"
api_key = "${OPENAI_API_KEY}"
model = "gpt-4"
base_url = "https://api.openai.com/v1"

[workspace]
auto_save = true
session_persistence = true
default_layout = "development"

[ui]
theme = "dark"
show_line_numbers = true
syntax_highlighting = true

[git]
auto_commit = false
commit_template = "feat: ${description}"

[hooks]
pre_edit = ["format", "lint"]
post_edit = ["test"]
```

## 🌟 Advanced Features

### Multi-Provider Support
Switch between AI providers without reconfiguration:
```bash
picode --provider openai
picode --provider anthropic  
picode --provider local      # Self-hosted models
```

### Plugin Development
Create custom WASM plugins:
```rust
use picode_wasm::prelude::*;

#[plugin_export]
fn custom_analyzer(code: &str) -> AnalysisResult {
    // Your custom analysis logic
    AnalysisResult::new()
}
```

### MCP Server Mode
Use PiCode as a tool for other AI systems:
```bash
# Start MCP server
picode --mode mcp --port 3000

# Or as WASM module
wasmtime picode.wasm --mcp-server
```

## 📚 Documentation

- **[User Guide](doc/user/)** - Complete user documentation
- **[Developer Guide](doc/developer/)** - Development and contribution guide
- **[AI Reference](doc/ai/)** - Detailed technical reference
- **[API Documentation](https://docs.rs/picode)** - Rust API documentation

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](doc/developer/CONTRIBUTING.md) for details.

### Development Setup
```bash
git clone https://github.com/pnocera/PiCode.git
cd PiCode
cargo build
cargo test
```

### Testing
```bash
# Unit tests
cargo test

# Integration tests  
cargo test --test integration

# WASM tests
wasm-pack test --node
```

## 🐛 Troubleshooting

### Common Issues

**Connection failures**: Check your API key and network connectivity
```bash
picode doctor   # Run diagnostics
```

**Performance issues**: Try reducing context window
```bash
picode config set max_tokens 2048
```

**WASM compilation errors**: Ensure you have wasm-pack installed
```bash
cargo install wasm-pack
```

For more help, see our [Troubleshooting Guide](doc/user/TROUBLESHOOTING.md).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[Zellij](https://github.com/zellij-org/zellij)** - Terminal multiplexer foundation
- **[Claude Code](https://docs.anthropic.com/en/docs/claude-code/overview)** - Inspiration and feature reference  
- **Rust Community** - Amazing ecosystem and tools
- **OpenAPI Initiative** - Standardized API specifications

## 🔗 Links

- **Homepage**: https://picode.org
- **Documentation**: https://docs.picode.org
- **GitHub**: https://github.com/pnocera/PiCode
- **Issues**: https://github.com/pnocera/PiCode/issues
- **Discussions**: https://github.com/pnocera/PiCode/discussions

---

Made with ❤️ by the PiCode team. Happy coding!