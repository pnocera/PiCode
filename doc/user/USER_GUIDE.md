# PiCode User Guide

This comprehensive guide covers all aspects of using PiCode effectively in your development workflow.

## Table of Contents

1. [Core Concepts](#core-concepts)
2. [Interface Overview](#interface-overview)
3. [Slash Commands](#slash-commands)
4. [Project Management](#project-management)
5. [Code Assistance](#code-assistance)
6. [Git Integration](#git-integration)
7. [Configuration](#configuration)
8. [Customization](#customization)
9. [Performance Optimization](#performance-optimization)
10. [Best Practices](#best-practices)

## Core Concepts

### AI-First Development

PiCode is designed around the principle of AI-assisted development, where the AI becomes your coding partner rather than just a tool. Key concepts:

- **Context Awareness**: PiCode understands your entire project structure
- **Proactive Assistance**: Suggests improvements and catches issues early
- **Workflow Integration**: Seamlessly fits into existing development processes
- **Multi-Modal Interaction**: Combines text, code, and project metadata

### Terminal-Native Experience

Unlike web-based AI tools, PiCode operates directly in your terminal:

- **No Context Switching**: Stay in your development environment
- **File System Access**: Direct manipulation of your project files  
- **Shell Integration**: Execute commands and see results immediately
- **Session Persistence**: Maintain context across disconnections

### Provider Agnostic

PiCode works with any OpenAPI-compatible LLM provider:

- **Vendor Independence**: No lock-in to specific AI providers
- **Model Flexibility**: Switch between models based on task requirements
- **Cost Optimization**: Use different providers for different use cases
- **Fallback Support**: Configure multiple providers for reliability

## Interface Overview

### Layout System

PiCode uses a flexible pane-based layout system built on Zellij:

#### Default Layout
```
┌─ File Explorer ─────────┐  ┌─ AI Assistant ─────────┐
│ project/                │  │ Ready to help with     │
│ ├─ src/                 │  │ your development       │
│ │  ├─ main.rs          │  │ tasks!                 │
│ │  └─ lib.rs           │  │                        │
│ ├─ tests/               │  │ Recent activity:       │
│ ├─ Cargo.toml           │  │ • Analyzed 15 files    │
│ └─ README.md            │  │ • Generated 3 tests    │
│                         │  │ • Fixed 2 warnings    │
└─────────────────────────┘  └─────────────────────────┘

┌─ Terminal ──────────────────────────────────────────┐
│ $ cargo test                                        │
│ running 12 tests                                    │
│ test result: ok. 12 passed; 0 failed; 0 ignored   │
│                                                     │
│ >  _                                                │
└─────────────────────────────────────────────────────┘
```

#### Available Layouts

- **`default`**: Balanced view with explorer, assistant, and terminal
- **`coding`**: Maximized assistant with minimal explorer
- **`debugging`**: Split terminal views with compact assistant
- **`review`**: Side-by-side code and AI feedback
- **`minimal`**: Single-pane assistant only

Switch layouts with:
```bash
picode --layout coding
# or within PiCode:
/layout debugging
```

### Status Information

The interface displays real-time information:

- **Current Provider**: Which LLM is active
- **Token Usage**: Current session consumption
- **Project Context**: Files and directories being tracked
- **Git Status**: Branch, modified files, staging area
- **Background Tasks**: Long-running operations

## Slash Commands

Slash commands are PiCode's primary interaction method. They're designed to be intuitive and discoverable.

### Command Categories

#### Project Analysis
- `/analyze [path]` - Analyze code structure and quality
- `/summary` - Generate project overview
- `/dependencies` - Show dependency tree and issues  
- `/metrics` - Display code complexity and statistics
- `/search <query>` - Intelligent code search across project

#### Code Assistance  
- `/edit <file>` - AI-assisted code editing
- `/review <file>` - Comprehensive code review
- `/explain <file:line>` - Explain specific code sections
- `/optimize <file>` - Performance and efficiency suggestions
- `/refactor <file>` - Suggest refactoring opportunities
- `/docs <function>` - Generate or improve documentation

#### Testing
- `/test` - Run existing test suite
- `/test-gen <file>` - Generate tests for specific file
- `/test-fix` - Fix failing tests
- `/coverage` - Show test coverage report
- `/benchmark` - Run performance benchmarks

#### Git Operations
- `/status` - Enhanced git status with AI insights
- `/diff [file]` - Explain git differences
- `/commit [message]` - Smart commit with generated message
- `/branch <name>` - Create and switch to new branch
- `/merge <branch>` - Intelligent merge with conflict resolution
- `/rebase [branch]` - Interactive rebase assistance

#### Workflow
- `/todo` - Extract TODO items from codebase
- `/plan <feature>` - Generate implementation plan
- `/scaffold <type>` - Generate boilerplate code
- `/deploy` - Deployment readiness check

#### System
- `/help [command]` - Show help information
- `/config <key> [value]` - View or modify configuration
- `/provider <name>` - Switch LLM provider
- `/layout <name>` - Change interface layout
- `/history` - Show command history
- `/clear` - Clear assistant conversation
- `/exit` - Quit PiCode

### Command Examples

#### Advanced Analysis

```bash
# Analyze specific subdirectory
/analyze src/handlers

# Search for security vulnerabilities
/search "sql injection vulnerability"

# Find performance bottlenecks
/metrics --focus performance

# Dependency security audit
/dependencies --security-check
```

#### Sophisticated Code Assistance

```bash
# Edit with specific instructions
/edit src/auth.rs --add-jwt-validation --secure

# Review with focus areas
/review src/database.rs --focus security,performance

# Explain complex algorithm
/explain src/algorithms/sorting.rs:45-67

# Optimize for specific criteria
/optimize src/api.rs --target memory --benchmark
```

#### Advanced Git Operations

```bash
# Commit with detailed analysis
/commit --analyze-impact --conventional

# Interactive rebase help
/rebase main --interactive --resolve-conflicts

# Merge with strategy
/merge feature-branch --strategy recursive --no-ff
```

## Project Management

### Project Discovery

PiCode automatically detects project types and configures accordingly:

- **Rust**: Cargo.toml detection, crate analysis
- **Python**: requirements.txt, setup.py, pyproject.toml
- **JavaScript/TypeScript**: package.json, tsconfig.json  
- **Go**: go.mod detection and module analysis
- **Java**: pom.xml, build.gradle detection
- **C/C++**: Makefile, CMakeLists.txt analysis

### Context Management

PiCode maintains project context through:

#### File Indexing
- Automatic indexing of source files
- Dependency graph construction  
- Symbol table maintenance
- Change tracking and delta analysis

#### Selective Analysis
```bash
# Include specific directories
/config set include_paths "src,tests,docs"

# Exclude patterns
/config set exclude_patterns "target,node_modules,*.tmp"

# File size limits
/config set max_file_size "1MB"

# Binary file handling
/config set skip_binary true
```

### Project Templates

Generate new projects with built-in templates:

```bash
# Create new Rust project
picode new rust-api --template axum-api

# Available templates
picode templates list

# Custom template
picode new my-project --template-url https://github.com/user/template.git
```

## Code Assistance

### Code Generation

PiCode can generate various types of code:

#### Function Generation
```
/generate function --name calculate_tax --params amount:f64,rate:f64 --return f64
```

#### Struct/Class Generation
```
/generate struct User --fields name:String,email:String,age:u32 --derive Clone,Debug
```

#### Test Generation
```
/test-gen src/calculator.rs --coverage-target 90% --include-edge-cases
```

#### Documentation Generation
```
/docs src/api.rs --style rustdoc --include-examples
```

### Code Analysis

#### Static Analysis
- Syntax error detection
- Type checking assistance
- Unused variable identification
- Dead code detection
- Security vulnerability scanning

#### Quality Metrics
- Cyclomatic complexity
- Code duplication
- Maintainability index
- Technical debt assessment

#### Performance Analysis  
- Algorithmic complexity assessment
- Memory usage patterns
- Bottleneck identification
- Optimization recommendations

### Code Transformation

#### Refactoring Operations
```bash
# Extract function
/refactor extract-function src/main.rs:45-60 --name validate_input

# Rename symbol
/refactor rename calculate_total --to compute_total --scope project

# Move code
/refactor move-module src/utils.rs --to src/utils/mod.rs

# Extract interface
/refactor extract-trait src/database.rs --name Repository
```

#### Code Style
```bash
# Format code
/format src/ --style rustfmt

# Apply linting fixes
/lint --fix --rules all

# Enforce conventions
/style-check --enforce-naming --enforce-comments
```

## Git Integration

### Enhanced Git Commands

PiCode provides AI-powered enhancements to standard Git operations:

#### Intelligent Commits
```bash
# Generate commit message from changes
/commit --generate-message

# Conventional commit format
/commit --conventional --type feat --scope auth

# Include impact analysis
/commit --analyze-impact --breaking-changes
```

#### Smart Diff Analysis
```bash
# Explain changes
/diff HEAD~1 --explain-changes

# Review changes
/diff --staged --review-quality

# Impact assessment
/diff main..feature --assess-risk
```

#### Conflict Resolution
```bash
# Analyze merge conflicts
/merge-conflicts analyze

# Suggest resolutions
/merge-conflicts resolve --strategy preserve-both

# Apply AI-suggested fixes
/merge-conflicts apply-suggestions
```

### Workflow Integration

#### Branch Management
```bash
# Feature branch creation
/branch feature/user-auth --from main --track

# Branch comparison
/compare-branches main feature/payments --show-divergence

# Cleanup merged branches
/branch cleanup --merged --dry-run
```

#### Release Management
```bash
# Pre-release checklist
/release check --version 1.2.0

# Generate changelog
/changelog --since v1.1.0 --format markdown

# Tag release
/release tag --version 1.2.0 --sign
```

## Configuration

### Configuration Hierarchy

PiCode uses a hierarchical configuration system:

1. **Command-line arguments** (highest precedence)
2. **Environment variables**
3. **Project configuration** (`./picode.toml`)
4. **User configuration** (`~/.config/picode/config.toml`)
5. **System defaults** (lowest precedence)

### Provider Configuration

#### OpenAI
```toml
[provider.openai]
api_key = "${OPENAI_API_KEY}"
base_url = "https://api.openai.com/v1"
models = ["gpt-4", "gpt-3.5-turbo", "gpt-4-turbo"]
default_model = "gpt-4"
temperature = 0.2
max_tokens = 4096
timeout = 30
```

#### Anthropic
```toml
[provider.anthropic]
api_key = "${ANTHROPIC_API_KEY}"
base_url = "https://api.anthropic.com"
models = ["claude-3-opus-20240229", "claude-3-sonnet-20240229"]
default_model = "claude-3-sonnet-20240229"
max_tokens = 4096
```

#### Custom Providers
```toml
[provider.custom]
name = "my-provider"
api_key = "${MY_PROVIDER_KEY}"
base_url = "https://api.my-provider.com/v1"
openapi_spec = "https://api.my-provider.com/openapi.json"
auth_type = "bearer" # or "api_key", "oauth2"
models = ["my-model-1", "my-model-2"]
```

### Workspace Configuration

```toml
[workspace]
# Project analysis
auto_analyze = true
analysis_depth = "deep"  # "shallow", "medium", "deep"
max_file_size = "10MB"
include_patterns = ["src/**", "tests/**", "*.md"]
exclude_patterns = ["target/**", "node_modules/**"]

# File watching
watch_files = true
watch_directories = ["src", "tests"]
ignore_hidden = true

# Context management
max_context_files = 100
context_window = 8192
smart_truncation = true

# Performance
parallel_analysis = true
cache_results = true
cache_ttl = "1h"
```

### UI Configuration

```toml
[ui]
# Theme and colors
theme = "dark"  # "dark", "light", "auto"
color_scheme = "default"  # or custom color scheme name

# Layout
default_layout = "default"
show_line_numbers = true
show_file_tree = true
show_git_status = true

# Terminal
shell = "/bin/bash"
terminal_scrollback = 10000
clear_on_start = false

# Notifications
show_notifications = true
notification_duration = 3000
sound_notifications = false
```

### Git Configuration

```toml
[git]
# Commit behavior
auto_stage = false
auto_commit = false
commit_template = "${type}(${scope}): ${description}"
sign_commits = false

# Push behavior  
push_after_commit = false
push_tags = false
remote_name = "origin"

# Analysis
diff_algorithm = "histogram"
show_word_diff = true
context_lines = 3

# Hooks integration
pre_commit_hooks = ["format", "lint", "test"]
pre_push_hooks = ["test", "security-scan"]
```

### Hooks Configuration

```toml
[hooks]
# Pre-operation hooks
pre_edit = ["backup", "format"]
pre_commit = ["format", "lint", "test"] 
pre_push = ["test", "security-scan"]

# Post-operation hooks
post_edit = ["analyze", "test-related"]
post_commit = ["notify", "backup"]
post_merge = ["cleanup", "analyze"]

# Custom hooks
[hooks.custom]
backup = "cp ${file} ${file}.bak"
notify = "echo 'Operation completed' | notify-send"
security-scan = "cargo audit"
```

## Customization

### Custom Commands

Define custom slash commands:

```toml
[commands.custom]
[commands.custom.deploy]
description = "Deploy application"
command = "cargo build --release && docker build -t myapp ."
confirm = true
async = true

[commands.custom.benchmark]
description = "Run performance benchmarks"  
command = "cargo bench --features bench"
capture_output = true
timeout = 300
```

### Plugin System

PiCode supports WebAssembly plugins for extensibility:

#### Installing Plugins
```bash
# Install from repository
picode plugin install code-formatter

# Install from URL
picode plugin install https://github.com/user/picode-plugin.wasm

# Install local plugin
picode plugin install ./my-plugin.wasm
```

#### Plugin Configuration
```toml
[plugins]
enabled = ["code-formatter", "security-scanner", "performance-analyzer"]

[plugins.code-formatter]
languages = ["rust", "javascript", "python"]
style = "standard"

[plugins.security-scanner]
severity_threshold = "medium"
scan_dependencies = true
```

### Themes and Styling

#### Custom Color Schemes
```toml
[ui.themes.custom-dark]
background = "#1e1e1e"
foreground = "#d4d4d4"
accent = "#007acc"
warning = "#ff9500"
error = "#f14c4c"
success = "#89d185"
muted = "#6a9955"
```

#### Syntax Highlighting
```toml
[ui.syntax]
theme = "monokai"
highlight_current_line = true
show_whitespace = false
indent_guides = true
```

## Performance Optimization

### Context Management

#### Selective File Loading
```bash
# Only analyze specific file types
/config set file_types "rs,py,js,ts,go"

# Limit context size
/config set max_context_tokens 4000

# Smart context selection
/config set context_strategy "relevant"  # "all", "relevant", "minimal"
```

#### Caching Strategy
```bash
# Enable aggressive caching
/config set cache_analysis true
/config set cache_completions true
/config set cache_ttl "2h"

# Cache location
/config set cache_dir "~/.cache/picode"
```

### LLM Optimization

#### Model Selection
```bash
# Use faster model for simple tasks
/provider set-model gpt-3.5-turbo

# Switch to powerful model for complex analysis
/provider set-model gpt-4

# Auto-select model based on task
/config set auto_model_selection true
```

#### Request Optimization
```bash
# Reduce response tokens for speed
/config set max_completion_tokens 1024

# Batch similar requests
/config set batch_requests true

# Use streaming for long responses
/config set stream_responses true
```

### System Resources

#### Memory Management
```bash
# Limit memory usage
/config set max_memory "2GB"

# Clear cache periodically
/config set auto_cleanup true
/config set cleanup_interval "1h"

# Garbage collection tuning
/config set gc_aggressive false
```

#### CPU Optimization
```bash
# Parallel processing
/config set max_workers 4

# Background processing priority
/config set background_priority "low"

# Async operation timeouts
/config set operation_timeout 60
```

## Best Practices

### Effective AI Interaction

#### Clear Instructions
- Be specific about what you want
- Provide context about your goals
- Ask for explanations when needed
- Iterate on suggestions

#### Code Review Process
1. Use `/analyze` before major changes
2. Review AI suggestions carefully
3. Test generated code thoroughly
4. Document AI-assisted changes

#### Context Management
- Keep project structure clean
- Use meaningful file and variable names
- Maintain up-to-date documentation
- Regularly clean up dead code

### Workflow Integration

#### Daily Development
1. Start with `/analyze` to understand current state
2. Use `/todo` to identify pending tasks
3. Leverage `/test-gen` for comprehensive coverage
4. End sessions with `/commit` for clean history

#### Code Review
1. Use `/review` for self-assessment
2. Generate documentation with `/docs`
3. Check for security issues with `/security-scan`
4. Validate changes with comprehensive tests

#### Team Collaboration
- Maintain consistent configuration across team
- Use conventional commit messages
- Share custom commands and hooks
- Document AI-assisted decisions

### Security Considerations

#### API Key Management
- Use environment variables for secrets
- Rotate API keys regularly
- Implement key access restrictions
- Monitor API usage and costs

#### Code Security
- Review AI-generated code for vulnerabilities
- Don't commit sensitive information
- Use security scanning plugins
- Validate external dependencies

#### Data Privacy
- Understand what data is sent to LLM providers
- Use local models for sensitive projects
- Implement data retention policies
- Audit AI assistance logs

This user guide provides comprehensive coverage of PiCode's features and capabilities. For more specific information, see the specialized documentation files for [Slash Commands](SLASH_COMMANDS.md), [Hooks](HOOKS.md), and [Advanced Features](ADVANCED_FEATURES.md).