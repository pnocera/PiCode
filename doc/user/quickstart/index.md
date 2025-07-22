# Quick Start Guide

> 🚀 **Get Started with PiCode** - From zero to productive in minutes

Welcome to PiCode! This guide will get you up and running quickly with the universal LLM-powered development assistant.

## Prerequisites

Before you begin, ensure you have:
- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Terminal access** - Command line interface
- **LLM API access** - OpenAI, Anthropic Claude, or compatible provider

## Quick Installation

### Option 1: Install from Crates.io (Recommended)
```bash
cargo install picode-cli
```

### Option 2: Build from Source
```bash
git clone https://github.com/pnocera/PiCode.git
cd PiCode
cargo install --path .
```

### Option 3: Download Binary
```bash
# Linux/macOS
curl -L https://github.com/pnocera/PiCode/releases/latest/download/picode-linux -o picode
chmod +x picode
sudo mv picode /usr/local/bin/

# Windows
# Download from: https://github.com/pnocera/PiCode/releases/latest
```

## Verify Installation

```bash
picode --version
# Should output: picode 0.1.0
```

## Quick Start Steps

### 1. Initialize Configuration
```bash
picode init
```

This creates a configuration file at `~/.config/picode/config.toml`.

### 2. Configure Your LLM Provider

#### For OpenAI
```bash
export OPENAI_API_KEY="your-api-key-here"
picode config set provider openai
picode config set model gpt-4
```

#### For Anthropic Claude
```bash
export ANTHROPIC_API_KEY="your-api-key-here"
picode config set provider anthropic
picode config set model claude-3-sonnet-20240229
```

#### For Local Models (Ollama)
```bash
picode config set provider ollama
picode config set base_url http://localhost:11434
picode config set model llama2
```

### 3. Test Your Setup
```bash
picode test-connection
```

Should output:
```
✅ Connection successful!
✅ Model: gpt-4 (OpenAI)
✅ Ready to use PiCode
```

### 4. Your First Command

#### Interactive Mode
```bash
picode interactive
```

This starts a conversational session where you can:
- Ask questions about your codebase
- Request code generation
- Get debugging help
- Perform refactoring tasks

#### Direct Command
```bash
picode ask "How do I implement a REST API in Rust?"
```

#### Analyze Current Project
```bash
cd your-project-directory
picode analyze
```

## Essential Commands

### Interactive Mode
```bash
# Start interactive session
picode interactive

# Interactive with specific context
picode interactive --context ./src

# Interactive with custom prompt
picode interactive --system-prompt "You are a Rust expert"
```

### Project Analysis
```bash
# Analyze current directory
picode analyze

# Analyze specific directory
picode analyze ./src

# Generate project summary
picode analyze --output summary.md
```

### Code Generation
```bash
# Generate from description
picode generate "Create a CLI parser using Clap"

# Generate with template
picode generate --template rest-api --name user-service

# Generate tests
picode generate tests --file src/lib.rs
```

### Code Review
```bash
# Review changes
picode review

# Review specific files
picode review src/main.rs src/lib.rs

# Review with focus
picode review --focus security
```

## Configuration Quick Reference

### Configuration File Location
- **Linux/macOS**: `~/.config/picode/config.toml`
- **Windows**: `%APPDATA%\picode\config.toml`

### Basic Configuration
```toml
[general]
provider = "openai"
model = "gpt-4"
temperature = 0.7
max_tokens = 2000

[providers.openai]
api_key_env = "OPENAI_API_KEY"
base_url = "https://api.openai.com/v1"

[providers.anthropic]
api_key_env = "ANTHROPIC_API_KEY"
base_url = "https://api.anthropic.com"

[interface]
interactive_mode = true
auto_context = true
syntax_highlighting = true
```

### Environment Variables
```bash
# API Keys
export OPENAI_API_KEY="your-key"
export ANTHROPIC_API_KEY="your-key"
export GOOGLE_API_KEY="your-key"

# Configuration
export PICODE_PROVIDER="openai"
export PICODE_MODEL="gpt-4"
export PICODE_CONFIG_PATH="/custom/config/path"
```

## Interactive Mode Basics

Once in interactive mode, you can use:

### Slash Commands
```bash
/help           # Show available commands
/context ./src  # Add directory to context
/model gpt-4    # Switch model
/provider openai # Switch provider
/clear          # Clear conversation
/save           # Save conversation
/load           # Load conversation
/exit           # Exit interactive mode
```

### Natural Language Commands
```bash
"Analyze this codebase"
"Create a new function to handle HTTP requests"
"Review this code for security issues"
"Generate tests for the user module"
"Refactor this function to be more efficient"
"Explain how this algorithm works"
```

## Project Integration

### Initialize in Existing Project
```bash
cd your-project
picode project init
```

This creates:
- `.picode/config.toml` - Project-specific configuration
- `.picode/context.yaml` - Project context definitions
- `.picode/prompts/` - Custom prompts directory

### Add to Git
```bash
# Add to .gitignore
echo ".picode/sessions/" >> .gitignore
echo ".picode/cache/" >> .gitignore

# Commit configuration
git add .picode/config.toml .picode/context.yaml
git commit -m "Add PiCode configuration"
```

## Common Workflows

### 1. Code Review Workflow
```bash
# Make changes to your code
git add .

# Review changes before commit
picode review --staged

# Interactive review
picode interactive
# Then: "Review my staged changes"
```

### 2. Debug Assistance
```bash
# Analyze error
picode analyze --error "error message here"

# Interactive debugging
picode interactive --context ./src
# Then: "Help me debug this function"
```

### 3. Learning New Codebase
```bash
# High-level analysis
picode analyze --output overview.md

# Interactive exploration
picode interactive
# Then: "Explain the architecture of this project"
```

## Troubleshooting

### Common Issues

#### "API key not found"
```bash
# Check environment variable
echo $OPENAI_API_KEY

# Set if missing
export OPENAI_API_KEY="your-key"

# Or use config command
picode config set api_key "your-key"
```

#### "Connection failed"
```bash
# Test connection
picode test-connection --verbose

# Check configuration
picode config show

# Reset configuration
picode config reset
```

#### "Command not found: picode"
```bash
# Check installation
which picode

# Reinstall if needed
cargo install picode-cli --force

# Check PATH
echo $PATH
```

### Get Help
- 📖 [Full User Guide](../guides/index.md)
- 🔧 [Configuration Reference](../reference/config.md)
- 💬 [Community Support](https://github.com/pnocera/PiCode/discussions)
- 🐛 [Report Issues](https://github.com/pnocera/PiCode/issues)

## Next Steps

Once you're up and running:

1. **Explore Interactive Mode** - [Interactive Mode Guide](../guides/interactive-mode.md)
2. **Configure Your LLM** - [LLM Setup Guide](../guides/llm-setup.md)
3. **Set Up Your Project** - [Project Integration Guide](../guides/project-setup.md)
4. **Learn Workflows** - [Common Workflows](../examples/workflows.md)

---

**Need help?** Join our [community discussions](https://github.com/pnocera/PiCode/discussions) or check the [troubleshooting guide](../guides/workflows.md#troubleshooting).