# Getting Started with PiCode

Welcome to PiCode! This guide will help you get up and running with your new AI-powered terminal workspace in just a few minutes.

## What is PiCode?

PiCode is a terminal-native AI assistant that brings the power of Large Language Models directly into your development workflow. Unlike web-based AI tools, PiCode:

- **Lives in your terminal** - No context switching required
- **Understands your codebase** - Analyzes project structure and dependencies
- **Works with any OpenAPI LLM** - Not locked to a single provider
- **Integrates with Git** - Helps with commits, reviews, and conflict resolution
- **Runs everywhere** - Native binary, WebAssembly, or as an MCP server

## Prerequisites

- **Operating System**: Linux, macOS, or Windows
- **Rust** (if building from source): 1.84 or later
- **LLM API Access**: OpenAI, Anthropic, or any OpenAPI-compatible provider

## Installation

### Option 1: Download Pre-built Binary (Recommended)

```bash
# Linux x64
curl -L https://github.com/pnocera/PiCode/releases/latest/download/picode-linux-x64.tar.gz | tar xz
sudo mv picode /usr/local/bin/

# macOS x64
curl -L https://github.com/pnocera/PiCode/releases/latest/download/picode-macos-x64.tar.gz | tar xz
sudo mv picode /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/pnocera/PiCode/releases/latest/download/picode-windows-x64.zip" -OutFile "picode.zip"
Expand-Archive -Path "picode.zip" -DestinationPath "."
# Move picode.exe to a directory in your PATH
```

### Option 2: Install with Cargo

```bash
# Install from crates.io (when available)
cargo install picode

# Or from source
git clone https://github.com/pnocera/PiCode.git
cd PiCode
cargo install --path .
```

### Option 3: WebAssembly Version

```bash
# Install with WASM support for browser/MCP usage
cargo install --path . --features wasm
```

### Verify Installation

```bash
picode --version
# Should output: picode 0.1.0
```

## Initial Configuration

### Step 1: Set Up Your LLM Provider

PiCode works with any OpenAPI-compatible LLM provider. Choose one below:

#### OpenAI (GPT-4, GPT-3.5)

```bash
picode config set provider openai
picode config set api_key YOUR_OPENAI_API_KEY
picode config set model gpt-4
```

#### Anthropic (Claude)

```bash
picode config set provider anthropic  
picode config set api_key YOUR_ANTHROPIC_API_KEY
picode config set model claude-3-sonnet-20241022
```

#### Custom OpenAPI Provider

```bash
picode config set provider custom
picode config set api_url https://api.your-provider.com/v1
picode config set api_key YOUR_API_KEY  
picode config set model your-model-name
```

#### Environment Variables (Alternative)

You can also use environment variables:

```bash
export PICODE_PROVIDER=openai
export PICODE_API_KEY=your-api-key-here
export PICODE_MODEL=gpt-4
```

### Step 2: Test Your Configuration

```bash
picode test-connection
# Should output: ✓ Successfully connected to [provider] using [model]
```

## Your First Session

### Launch PiCode

```bash
# Start PiCode in current directory
picode

# Or specify a project directory
picode --project /path/to/your/project

# Start with a specific layout
picode --layout coding
```

### The PiCode Interface

When you first launch PiCode, you'll see a split-pane interface:

```
┌─ Project Explorer ──────┐  ┌─ AI Assistant ──────────┐
│ my-project/             │  │ Hello! I'm your AI      │
│ ├─ src/                 │  │ assistant. I can help   │
│ │  ├─ main.rs           │  │ with:                   │
│ │  └─ lib.rs            │  │                         │
│ ├─ tests/               │  │ • Code analysis         │
│ ├─ Cargo.toml           │  │ • Writing functions     │
│ └─ README.md            │  │ • Debugging issues      │
│                         │  │ • Git operations        │
│ Working Directory:      │  │ • Testing assistance    │
│ /home/user/my-project   │  │                         │
└─────────────────────────┘  │ Type /help for commands │
                              └─────────────────────────┘

> _
```

### Essential Slash Commands

PiCode uses slash commands for quick actions. Here are the most important ones:

#### Get Help
```
/help           # Show all available commands
/help search    # Get help on specific command
```

#### Analyze Your Project
```
/analyze              # Analyze entire project
/analyze src/main.rs  # Analyze specific file
/summary             # Get project summary
```

#### Code Assistance
```
/edit src/main.rs           # AI-assisted editing
/review src/main.rs         # Code review
/explain src/main.rs:23     # Explain specific line
/optimize src/main.rs       # Performance suggestions
```

#### Testing
```
/test                 # Run existing tests
/test-gen src/main.rs # Generate tests for file
/coverage            # Show test coverage
```

#### Git Integration
```
/status              # Git status with AI insights
/commit             # Generate commit message
/diff               # Explain git diff
/branch feature-x   # Create and switch branch
```

## Example Workflow

Let's walk through a typical development session:

### 1. Project Analysis

```bash
$ picode --project my-rust-api

> /analyze
Analyzing project structure...
✓ Cargo.toml found - Rust project detected
✓ 15 source files analyzed
✓ 8 dependencies identified
✓ Test coverage: 67%

Key insights:
- Main API routes in src/routes/
- Database models in src/models/
- Missing error handling in src/handlers/users.rs:45
- Consider adding integration tests

> /summary
This appears to be a REST API built with Axum framework...
[Detailed project summary]
```

### 2. Code Review and Improvement

```
> /review src/handlers/users.rs

Reviewing src/handlers/users.rs...

Findings:
🔴 Line 45: Unwrap() call could panic - use proper error handling
🟡 Line 23: Consider using structured logging instead of println!
🟢 Overall structure looks good

Would you like me to fix these issues?

> yes

Applying fixes...
✓ Replaced unwrap() with proper error handling
✓ Added structured logging with tracing
✓ Updated function documentation
```

### 3. Test Generation

```
> /test-gen src/handlers/users.rs

Generated comprehensive tests for users.rs:
- test_create_user_success()
- test_create_user_validation_error()  
- test_get_user_not_found()
- test_update_user_permissions()

Tests saved to tests/handlers/users.rs
Would you like me to run them?

> yes

Running tests...
✓ All 4 tests passed
```

### 4. Git Operations

```
> /status

Git Status with AI Analysis:
Modified files:
- src/handlers/users.rs (improved error handling)
- tests/handlers/users.rs (new file)

Suggested commit message:
"feat(users): improve error handling and add comprehensive tests"

> /commit
Creating commit with suggested message...
✓ Commit created: a1b2c3d "feat(users): improve error handling and add comprehensive tests"
```

## Configuration Options

### Basic Settings

```bash
# View current configuration
picode config show

# Set workspace preferences
picode config set auto_save true
picode config set theme dark
picode config set show_line_numbers true

# Git integration
picode config set git.auto_stage false
picode config set git.commit_template "feat: ${description}"

# Performance tuning
picode config set max_tokens 4096
picode config set timeout_seconds 30
```

### Advanced Configuration File

Create `~/.config/picode/config.toml`:

```toml
[provider]
name = "openai"
api_key = "${OPENAI_API_KEY}"
model = "gpt-4"
temperature = 0.2
max_tokens = 4096

[workspace]
auto_save = true
session_persistence = true
project_analysis_depth = "deep"
file_watcher = true

[ui]
theme = "dark"
show_line_numbers = true
syntax_highlighting = true
word_wrap = true

[git]
auto_stage = false
auto_commit = false
commit_template = "${type}: ${description}"
push_after_commit = false

[hooks]
pre_edit = ["format", "lint"]
post_edit = ["test"]
pre_commit = ["format", "test"]

[shortcuts]
# Custom key bindings
analyze = "Ctrl+A"
test = "Ctrl+T"
commit = "Ctrl+G"
```

## Next Steps

Now that you have PiCode set up, you might want to:

1. **Read the [User Guide](USER_GUIDE.md)** for detailed feature documentation
2. **Explore [Slash Commands](SLASH_COMMANDS.md)** for comprehensive command reference
3. **Learn about [Hooks](HOOKS.md)** to customize your workflow
4. **Check out [Advanced Features](ADVANCED_FEATURES.md)** for power-user capabilities
5. **Join our [Community](https://github.com/pnocera/PiCode/discussions)** for tips and support

## Troubleshooting

### Common Issues

**"Connection failed"**
- Verify your API key: `picode config get api_key`
- Test connectivity: `picode test-connection`
- Check your internet connection

**"Command not found"**
- Ensure PiCode is in your PATH: `which picode`
- Try absolute path: `/usr/local/bin/picode`

**Slow responses**
- Reduce context: `picode config set max_tokens 2048`
- Try a faster model: `picode config set model gpt-3.5-turbo`

**Permission errors**
- Check file permissions: `ls -la ~/.config/picode/`
- Ensure you have write access to project directory

### Getting Help

- **Documentation**: [docs.picode.org](https://docs.picode.org)
- **GitHub Issues**: [Report bugs](https://github.com/pnocera/PiCode/issues)
- **Discussions**: [Community support](https://github.com/pnocera/PiCode/discussions)
- **Built-in help**: Type `/help` in PiCode

### Diagnostic Commands

```bash
picode doctor        # Run system diagnostics
picode config check  # Validate configuration
picode logs          # View recent logs
picode --verbose     # Enable debug output
```

Welcome to PiCode! We hope you enjoy your new AI-powered development experience. 🚀