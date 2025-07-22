# PiCode Development Setup Guide

This guide helps new contributors set up a complete development environment for PiCode, including all necessary tools, dependencies, and development workflows.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Repository Setup](#repository-setup)
4. [Development Tools](#development-tools)
5. [Building and Testing](#building-and-testing)
6. [Development Workflow](#development-workflow)
7. [IDE Configuration](#ide-configuration)
8. [Debugging](#debugging)
9. [Performance Profiling](#performance-profiling)
10. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Software

#### Rust Toolchain
```bash
# Install rustup (Rust installer and version management tool)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install required Rust version
rustup install 1.84
rustup default 1.84

# Add required components
rustup component add rustfmt clippy
rustup component add rust-src rust-analysis

# Add WASM target
rustup target add wasm32-unknown-unknown
```

#### System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libgit2-dev \
    cmake \
    git \
    curl \
    nodejs \
    npm
```

**RHEL/CentOS/Fedora:**
```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    pkgconfig \
    openssl-devel \
    libgit2-devel \
    cmake \
    git \
    curl \
    nodejs \
    npm
```

**macOS:**
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install pkg-config openssl libgit2 cmake git curl node
```

**Windows:**
```powershell
# Install Rust via rustup-init.exe from https://rustup.rs/

# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

# Install Git for Windows
# Download from: https://git-scm.com/download/win

# Install Node.js
# Download from: https://nodejs.org/
```

#### Additional Tools
```bash
# Install cargo tools
cargo install cargo-watch cargo-audit cargo-expand wasm-pack

# Install development tools
cargo install cargo-make cargo-tarpaulin cargo-deny

# Install WASM tools
cargo install wasm-pack
npm install -g @webassembly/wabt

# Install documentation tools
cargo install mdbook mdbook-linkcheck
```

### Optional but Recommended

#### Docker (for testing and deployment)
```bash
# Ubuntu/Debian
sudo apt install docker.io docker-compose

# macOS
brew install docker docker-compose

# Or install Docker Desktop from https://www.docker.com/products/docker-desktop
```

#### Development Database (for testing)
```bash
# SQLite (usually included)
sudo apt install sqlite3  # Ubuntu/Debian
brew install sqlite       # macOS

# PostgreSQL (for advanced testing)
sudo apt install postgresql postgresql-client  # Ubuntu
brew install postgresql                        # macOS
```

## Environment Setup

### Environment Variables

Create a `.env` file in your project root:

```bash
# .env file for development

# Rust environment
RUST_BACKTRACE=1
RUST_LOG=debug

# PiCode development settings
PICODE_ENV=development
PICODE_DEBUG=true
PICODE_LOG_LEVEL=debug

# API keys for testing (use your own)
OPENAI_API_KEY=sk-your-openai-key
ANTHROPIC_API_KEY=sk-ant-your-anthropic-key

# Database for testing
DATABASE_URL=sqlite:./test.db

# Performance profiling
CARGO_PROFILE_DEV_DEBUG=true
```

### Shell Configuration

Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
# Rust environment
export RUST_BACKTRACE=1
export CARGO_TARGET_DIR="$HOME/.cache/cargo-target"

# Path additions
export PATH="$HOME/.cargo/bin:$PATH"

# Development aliases
alias pdev="cargo watch -x 'run -- --dev'"
alias ptest="cargo test -- --nocapture"
alias pcheck="cargo clippy -- -D warnings"
alias pfmt="cargo fmt --all"

# PiCode specific
export PICODE_DEV_MODE=true
export PICODE_CONFIG_DIR="$HOME/.config/picode-dev"
```

### Git Configuration

Set up Git hooks and configuration:

```bash
# Configure Git for the project
git config --local user.name "Your Name"
git config --local user.email "your.email@example.com"

# Install pre-commit hooks
cp scripts/hooks/* .git/hooks/
chmod +x .git/hooks/*

# Or use the provided script
./scripts/setup-git-hooks.sh
```

## Repository Setup

### Clone and Initial Setup

```bash
# Clone the repository
git clone https://github.com/pnocera/PiCode.git
cd PiCode

# Or if you forked it
git clone https://github.com/YOUR_USERNAME/PiCode.git
cd PiCode
git remote add upstream https://github.com/pnocera/PiCode.git

# Run the setup script
./scripts/setup-dev.sh
```

### Development Configuration

Create a development configuration:

```bash
# Create development config directory
mkdir -p ~/.config/picode-dev

# Create development configuration
cat > ~/.config/picode-dev/config.toml << 'EOF'
[provider]
name = "openai"
api_key = "${OPENAI_API_KEY}"
model = "gpt-4"

[workspace]
auto_save = true
debug_mode = true
log_level = "debug"

[development]
hot_reload = true
mock_llm = false
test_mode = false

[hooks]
pre_edit = ["format_check"]
post_edit = ["test_related"]
EOF
```

### Workspace Structure

Understand the project structure:

```
PiCode/
├── .github/              # GitHub Actions and templates
├── .vscode/              # VS Code configuration
├── benches/              # Benchmark tests
├── doc/                  # Documentation
│   ├── user/             # User documentation
│   ├── developer/        # Developer documentation
│   └── ai/               # AI integration docs
├── examples/             # Example code and configurations
├── picode-cli/           # CLI components
├── picode-core/          # Core workspace logic
├── picode-hooks/         # Hook system
├── picode-llm/           # LLM integration
├── picode-wasm/          # WebAssembly bindings
├── scripts/              # Build and utility scripts
├── src/                  # Main application code
├── tests/                # Integration tests
├── Cargo.toml            # Root Cargo configuration
└── README.md             # Project overview
```

## Development Tools

### Cargo Make

We use `cargo-make` for task automation. Available tasks:

```bash
# List all available tasks
cargo make --list-all-steps

# Development tasks
cargo make dev              # Development build with watching
cargo make test-all         # Run all tests
cargo make lint             # Run linters
cargo make format           # Format code
cargo make docs             # Generate documentation

# Build tasks
cargo make build-release    # Release build
cargo make build-wasm      # WebAssembly build
cargo make build-all       # All targets

# Quality tasks
cargo make audit           # Security audit
cargo make coverage        # Test coverage
cargo make bench          # Benchmarks
```

### Pre-commit Checks

Set up automated quality checks:

```bash
# Install pre-commit (Python)
pip install pre-commit

# Install hooks
pre-commit install

# Or use our script
./scripts/install-precommit.sh
```

### Development Scripts

Available development scripts:

```bash
# Setup
./scripts/setup-dev.sh        # Initial development setup
./scripts/setup-git-hooks.sh  # Install Git hooks
./scripts/install-deps.sh     # Install all dependencies

# Development
./scripts/dev-server.sh       # Start development server
./scripts/watch-tests.sh      # Watch and run tests
./scripts/hot-reload.sh       # Hot reload development

# Testing
./scripts/run-tests.sh        # Run all tests
./scripts/integration-test.sh # Integration tests only
./scripts/wasm-test.sh        # WASM-specific tests

# Quality
./scripts/lint-all.sh         # Run all linters
./scripts/format-all.sh       # Format all code
./scripts/check-all.sh        # Comprehensive checks

# Documentation
./scripts/build-docs.sh       # Build documentation
./scripts/serve-docs.sh       # Serve docs locally
./scripts/update-readme.sh    # Update README
```

## Building and Testing

### Development Builds

```bash
# Fast development build
cargo build

# Development build with features
cargo build --features "wasm,debug"

# Build specific crate
cargo build -p picode-core

# Build with watching (auto-rebuild on changes)
cargo watch -x build
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_workspace_initialization

# Run tests for specific crate
cargo test -p picode-llm

# Run integration tests
cargo test --test integration

# Run with coverage
cargo tarpaulin --out Html

# Performance tests
cargo test --release -- --ignored perf_
```

### WebAssembly Build

```bash
# Build for WASM
cargo build --target wasm32-unknown-unknown

# Build WASM package
wasm-pack build --target web

# Test WASM build
wasm-pack test --node

# Build for specific WASM features
cargo build --target wasm32-unknown-unknown --features wasm-only
```

### Release Builds

```bash
# Optimized release build
cargo build --release

# Release build with all features
cargo build --release --all-features

# Cross-compilation for different targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu
```

## Development Workflow

### Daily Development

1. **Start development session:**
   ```bash
   # Pull latest changes
   git pull upstream main
   
   # Start development environment
   ./scripts/dev-session.sh
   
   # Start watching tests
   cargo watch -x test
   ```

2. **Create feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Development cycle:**
   ```bash
   # Make changes
   # Auto-format on save (if configured)
   
   # Check your work
   cargo check
   cargo clippy
   cargo test
   
   # Run integration tests
   cargo test --test integration
   ```

4. **Before committing:**
   ```bash
   # Format code
   cargo fmt --all
   
   # Run lints
   cargo clippy -- -D warnings
   
   # Run all tests
   cargo test
   
   # Check documentation
   cargo doc --no-deps
   ```

5. **Commit and push:**
   ```bash
   git add .
   git commit -m "feat: add new feature"
   git push origin feature/your-feature-name
   ```

### Feature Development Workflow

#### 1. Planning Phase
```bash
# Create design document
cp templates/feature-design.md doc/developer/features/your-feature.md

# Discuss in issue or PR
# Create tracking issue
```

#### 2. Implementation Phase
```bash
# Create tests first (TDD)
touch tests/integration/test_your_feature.rs

# Implement core functionality
# Start with picode-core if needed

# Add CLI interface
# Update picode-cli as needed

# Add documentation
# Update relevant .md files
```

#### 3. Testing Phase
```bash
# Unit tests
cargo test your_feature

# Integration tests
cargo test --test integration -- your_feature

# Manual testing
cargo run -- --feature your-feature --test-data
```

#### 4. Review Phase
```bash
# Self-review checklist
./scripts/pre-review-check.sh

# Create pull request
gh pr create --title "feat: your feature" --body-file pr-template.md
```

## IDE Configuration

### Visual Studio Code

Recommended extensions (`.vscode/extensions.json`):

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "vadimcn.vscode-lldb",
    "serayuzgur.crates",
    "tamasfe.even-better-toml",
    "usernamehw.errorlens",
    "ms-vscode.test-adapter-converter"
  ]
}
```

VS Code settings (`.vscode/settings.json`):

```json
{
  "rust-analyzer.server.path": "rust-analyzer",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.allTargets": false,
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

### Vim/Neovim

For Neovim with LSP support:

```lua
-- ~/.config/nvim/init.lua or appropriate config file

-- Rust analyzer setup
local lspconfig = require('lspconfig')
lspconfig.rust_analyzer.setup({
  settings = {
    ['rust-analyzer'] = {
      cargo = {
        features = 'all'
      },
      checkOnSave = {
        command = 'clippy'
      }
    }
  }
})

-- Key mappings
vim.keymap.set('n', '<leader>t', '<cmd>!cargo test<CR>')
vim.keymap.set('n', '<leader>b', '<cmd>!cargo build<CR>')
vim.keymap.set('n', '<leader>r', '<cmd>!cargo run<CR>')
```

### Emacs

For Emacs with rust-mode:

```elisp
;; ~/.emacs.d/init.el

(use-package rust-mode
  :ensure t
  :hook (rust-mode . lsp-deferred)
  :config
  (setq rust-format-on-save t))

(use-package lsp-mode
  :ensure t
  :commands lsp
  :config
  (setq lsp-rust-analyzer-cargo-watch-command "clippy"))

;; Key bindings
(define-key rust-mode-map (kbd "C-c C-c") 'rust-run)
(define-key rust-mode-map (kbd "C-c C-t") 'rust-test)
```

## Debugging

### Rust Debugging

#### Using GDB/LLDB

```bash
# Build with debug symbols
cargo build --features debug-symbols

# Debug with GDB
gdb target/debug/picode
(gdb) run --config test-config.toml

# Debug with LLDB (macOS)
lldb target/debug/picode
(lldb) run --config test-config.toml
```

#### Using VS Code Debugger

Launch configuration (`.vscode/launch.json`):

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug PiCode",
      "cargo": {
        "args": ["build", "--bin=picode"],
        "filter": {
          "name": "picode",
          "kind": "bin"
        }
      },
      "args": ["--debug", "--config", "test-config.toml"],
      "cwd": "${workspaceFolder}",
      "environment": [
        {"name": "RUST_LOG", "value": "debug"}
      ]
    }
  ]
}
```

#### Debugging Tests

```bash
# Run single test with debugger
cargo test test_name -- --nocapture

# Debug with environment
RUST_LOG=debug cargo test test_name -- --nocapture

# Debug integration test
cargo test --test integration test_function -- --nocapture
```

### Log-based Debugging

```rust
// Add to your code for debugging
use tracing::{debug, info, warn, error};

debug!("Processing request: {:?}", request);
info!("Operation completed successfully");
warn!("Retrying failed operation: {}", retry_count);
error!("Fatal error occurred: {}", error);
```

Enable detailed logging:

```bash
RUST_LOG=debug cargo run
RUST_LOG=picode::llm=trace cargo run
RUST_LOG=picode=debug,hyper=info cargo run
```

## Performance Profiling

### CPU Profiling

```bash
# Install profiling tools
cargo install flamegraph
cargo install cargo-profiler

# Generate flame graph
cargo flamegraph --bin picode -- --test-workload

# Profile with perf (Linux)
perf record target/release/picode --test-workload
perf report

# Profile specific function
cargo bench --bench main_bench
```

### Memory Profiling

```bash
# Install memory profilers
cargo install heaptrack

# Memory profiling with heaptrack
heaptrack target/release/picode --test-workload

# Valgrind (Linux)
valgrind --tool=memcheck --leak-check=full target/debug/picode

# Memory usage tracking
cargo build --features memory-profiling
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench analysis_benchmark

# Compare benchmarks
cargo bench --save-baseline main
# Make changes
cargo bench --baseline main

# Criterion HTML reports
cargo bench -- --save-baseline before
# After changes
cargo bench -- --baseline before
```

### Performance Testing

```bash
# Load testing
./scripts/load-test.sh

# Stress testing
./scripts/stress-test.sh

# Memory leak detection
./scripts/memory-leak-test.sh

# Performance regression testing
./scripts/perf-regression-test.sh
```

## Troubleshooting

### Common Build Issues

#### Dependency Compilation Errors

```bash
# Clear cargo cache
cargo clean

# Update dependencies
cargo update

# Force rebuild
cargo build --offline --frozen
```

#### OpenSSL Issues (Linux)

```bash
# Install OpenSSL development headers
sudo apt install libssl-dev pkg-config  # Ubuntu/Debian
sudo dnf install openssl-devel pkgconf  # Fedora

# Set PKG_CONFIG_PATH if needed
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig"
```

#### Git2 Compilation Issues

```bash
# Install libgit2 development files
sudo apt install libgit2-dev  # Ubuntu/Debian
sudo dnf install libgit2-devel # Fedora
brew install libgit2           # macOS

# Use system libgit2
export LIBGIT2_SYS_USE_PKG_CONFIG=1
```

### Runtime Issues

#### Configuration Problems

```bash
# Validate configuration
cargo run -- config validate

# Reset to defaults
cargo run -- config reset

# Check config paths
cargo run -- config paths
```

#### API Connection Issues

```bash
# Test API connectivity
cargo run -- test-connection --verbose

# Debug with proxy
HTTP_PROXY=http://proxy:port cargo run

# Skip TLS verification (development only)
cargo run -- --ignore-tls-errors
```

### Testing Issues

#### Flaky Tests

```bash
# Run tests multiple times
for i in {1..10}; do cargo test test_name || break; done

# Run with different thread counts
cargo test -- --test-threads=1

# Ignore timing-dependent tests
cargo test -- --skip timing_test
```

#### Integration Test Failures

```bash
# Check test data
ls -la tests/fixtures/

# Reset test environment
./scripts/reset-test-env.sh

# Run with clean state
cargo clean && cargo test --test integration
```

### Development Environment Issues

#### Slow Compilation

```bash
# Use shared target directory
export CARGO_TARGET_DIR="$HOME/.cache/cargo-target"

# Parallel compilation
export CARGO_BUILD_JOBS=8

# Use faster linker (Linux)
sudo apt install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Enable incremental compilation
export CARGO_INCREMENTAL=1
```

#### IDE Problems

```bash
# Restart rust-analyzer
# VS Code: Ctrl+Shift+P -> "Rust Analyzer: Restart Server"

# Clear IDE caches
rm -rf .vscode/settings.json
cargo clean

# Check rust-analyzer logs
# VS Code: View -> Output -> Rust Analyzer Language Server
```

### Getting Help

If you encounter issues not covered here:

1. **Check existing issues**: [GitHub Issues](https://github.com/pnocera/PiCode/issues)
2. **Search documentation**: Use grep or your editor's search
3. **Ask in discussions**: [GitHub Discussions](https://github.com/pnocera/PiCode/discussions)
4. **Contact maintainers**: See CONTRIBUTING.md for contact info

### Creating Bug Reports

When reporting development setup issues:

```bash
# Gather system information
./scripts/collect-debug-info.sh > debug-info.txt

# Include in bug report:
# - Operating system and version
# - Rust version (rustc --version)
# - Output of debug-info.txt
# - Steps to reproduce
# - Expected vs actual behavior
```

This development setup guide should get you ready for productive PiCode development. Remember to keep your tools updated and don't hesitate to ask for help when needed!