# PiCode Troubleshooting Guide

This guide helps you diagnose and resolve common issues with PiCode. Whether you're experiencing installation problems, connection issues, or performance concerns, you'll find solutions here.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Configuration Problems](#configuration-problems)
3. [Connection Issues](#connection-issues)
4. [Performance Problems](#performance-problems)
5. [Command Issues](#command-issues)
6. [Git Integration Problems](#git-integration-problems)
7. [WebAssembly Issues](#webassembly-issues)
8. [Getting Help](#getting-help)

## Installation Issues

### Binary Installation Problems

#### "Command not found" after installation

**Problem**: PiCode binary is not in PATH

**Solutions**:
```bash
# Check if binary exists
ls -la /usr/local/bin/picode

# If not found, reinstall
curl -L https://github.com/pnocera/PiCode/releases/latest/download/picode-linux-x64.tar.gz | tar xz
sudo mv picode /usr/local/bin/

# Add to PATH if using custom location
echo 'export PATH="$PATH:/path/to/picode"' >> ~/.bashrc
source ~/.bashrc
```

#### Permission denied errors

**Problem**: Insufficient permissions for installation

**Solutions**:
```bash
# Fix permissions
sudo chown $(whoami):$(whoami) /usr/local/bin/picode
sudo chmod +x /usr/local/bin/picode

# Or install to user directory
mkdir -p ~/.local/bin
mv picode ~/.local/bin/
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.bashrc
```

#### Architecture mismatch errors

**Problem**: Downloaded wrong binary for your architecture

**Solutions**:
```bash
# Check your architecture
uname -m

# Download correct binary
# For x86_64
curl -L https://github.com/pnocera/PiCode/releases/latest/download/picode-linux-x64.tar.gz | tar xz

# For ARM64
curl -L https://github.com/pnocera/PiCode/releases/latest/download/picode-linux-arm64.tar.gz | tar xz
```

### Source Installation Problems

#### Rust version conflicts

**Problem**: Outdated Rust version

**Solutions**:
```bash
# Update Rust
rustup update stable

# Check version
rustc --version
# Should be 1.84 or later

# Set default toolchain
rustup default stable
```

#### Dependency compilation errors

**Problem**: Missing system dependencies

**Solutions**:

**Ubuntu/Debian**:
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev libgit2-dev
```

**RHEL/CentOS/Fedora**:
```bash
sudo dnf install gcc gcc-c++ pkgconfig openssl-devel libgit2-devel
# or for older versions:
sudo yum install gcc gcc-c++ pkgconfig openssl-devel libgit2-devel
```

**macOS**:
```bash
# Install Xcode command line tools
xcode-select --install

# Or install via Homebrew
brew install pkg-config openssl libgit2
```

#### Linking errors

**Problem**: Linker cannot find libraries

**Solutions**:
```bash
# Set library path
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH"

# For macOS with Homebrew
export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:$PKG_CONFIG_PATH"

# Force static linking
OPENSSL_STATIC=1 cargo install --path .
```

## Configuration Problems

### Configuration File Issues

#### Cannot find configuration file

**Problem**: PiCode cannot locate configuration

**Diagnostic**:
```bash
# Check configuration search paths
picode config paths

# Show current configuration
picode config show
```

**Solutions**:
```bash
# Create configuration directory
mkdir -p ~/.config/picode

# Create basic configuration
cat > ~/.config/picode/config.toml << 'EOF'
[provider]
name = "openai"
api_key = "${OPENAI_API_KEY}"
model = "gpt-4"

[workspace]
auto_save = true
session_persistence = true
EOF
```

#### Invalid configuration format

**Problem**: TOML syntax errors in configuration

**Diagnostic**:
```bash
# Validate configuration
picode config validate
```

**Solutions**:
```bash
# Check TOML syntax
toml-check ~/.config/picode/config.toml

# Or use online TOML validator
# Copy configuration content to https://www.toml-lint.com/
```

#### Environment variable substitution fails

**Problem**: Environment variables not being substituted

**Diagnostic**:
```bash
# Check if variable is set
echo $OPENAI_API_KEY

# Test configuration loading
picode config get provider.api_key
```

**Solutions**:
```bash
# Set environment variable
export OPENAI_API_KEY="your-key-here"

# Or set in configuration file directly
picode config set provider.api_key "your-key-here"

# Check shell profile
echo 'export OPENAI_API_KEY="your-key-here"' >> ~/.bashrc
source ~/.bashrc
```

### Permission Issues

#### Cannot write to configuration directory

**Problem**: Permission denied when saving configuration

**Solutions**:
```bash
# Fix directory permissions
sudo chown -R $(whoami):$(whoami) ~/.config/picode
chmod -R 755 ~/.config/picode

# Or use alternative location
picode config set-location ./picode.toml
```

#### API key storage problems

**Problem**: Cannot store API keys securely

**Solutions**:
```bash
# Check keyring availability
keyring --help

# Install keyring for your system
# Ubuntu/Debian
sudo apt install python3-keyring

# Store key manually
picode config set-secure provider.api_key "your-key-here"

# Or use environment variable
export PICODE_API_KEY="your-key-here"
```

## Connection Issues

### LLM Provider Connection Problems

#### API key authentication failures

**Problem**: "Invalid API key" or "Authentication failed"

**Diagnostic**:
```bash
# Test connection
picode test-connection

# Check API key format
picode config get provider.api_key --show-masked
```

**Solutions**:
```bash
# Verify API key is correct
# For OpenAI: should start with "sk-"
# For Anthropic: should start with "sk-ant-"

# Set correct API key
picode config set provider.api_key "sk-your-actual-key"

# Or use environment variable
export OPENAI_API_KEY="sk-your-actual-key"
```

#### Network connectivity issues

**Problem**: Cannot reach API endpoints

**Diagnostic**:
```bash
# Test basic connectivity
curl -I https://api.openai.com/v1/models

# Check DNS resolution
nslookup api.openai.com

# Test with proxy
curl -I --proxy http://proxy:port https://api.openai.com/v1/models
```

**Solutions**:
```bash
# Configure proxy
picode config set network.proxy "http://proxy:port"

# Or set environment variables
export HTTP_PROXY=http://proxy:port
export HTTPS_PROXY=http://proxy:port

# For corporate networks, configure certificates
picode config set network.ca_bundle "/path/to/certificates"
```

#### Rate limiting errors

**Problem**: "Rate limit exceeded" errors

**Diagnostic**:
```bash
# Check current usage
picode usage stats

# View rate limit headers
picode test-connection --verbose
```

**Solutions**:
```bash
# Configure rate limiting
picode config set provider.rate_limit.requests_per_minute 60
picode config set provider.rate_limit.tokens_per_minute 40000

# Use retry with backoff
picode config set provider.retry.enabled true
picode config set provider.retry.max_attempts 3
picode config set provider.retry.backoff_seconds 5

# Switch to less rate-limited model
picode config set provider.model "gpt-3.5-turbo"
```

### Custom Provider Issues

#### OpenAPI specification problems

**Problem**: Cannot parse custom provider OpenAPI spec

**Diagnostic**:
```bash
# Validate OpenAPI spec
picode validate-openapi https://api.example.com/openapi.json

# Show parsed functions
picode provider functions --provider custom
```

**Solutions**:
```bash
# Use supported OpenAPI version
# PiCode supports OpenAPI 3.0.x and 3.1.x

# Validate spec format
openapi-generator validate -i https://api.example.com/openapi.json

# Use local file if URL fails
picode config set provider.custom.openapi_spec "./openapi.json"
```

#### Authentication method detection

**Problem**: Cannot detect authentication method from OpenAPI spec

**Solutions**:
```bash
# Manually specify authentication
picode config set provider.custom.auth_type "bearer"
picode config set provider.custom.api_key "your-key"

# Or for OAuth2
picode config set provider.custom.auth_type "oauth2"
picode config set provider.custom.client_id "your-client-id"
picode config set provider.custom.client_secret "your-secret"
```

## Performance Problems

### Slow Response Times

#### High latency to AI providers

**Problem**: AI responses take too long

**Diagnostic**:
```bash
# Measure response times
picode benchmark --provider current

# Check network latency
ping api.openai.com
```

**Solutions**:
```bash
# Use faster model
picode config set provider.model "gpt-3.5-turbo"

# Reduce context window
picode config set max_tokens 2048

# Enable response streaming
picode config set provider.stream_responses true

# Use local cache
picode config set cache.enabled true
picode config set cache.ttl "1h"
```

#### Large project analysis slowdown

**Problem**: Project analysis takes too long

**Diagnostic**:
```bash
# Analyze performance
picode analyze --benchmark

# Check project size
picode project stats
```

**Solutions**:
```bash
# Limit analysis scope
picode config set analysis.max_files 1000
picode config set analysis.max_file_size "1MB"

# Exclude large directories
picode config set exclude_patterns "target,node_modules,*.log"

# Use shallow analysis
picode config set analysis.depth "shallow"

# Enable parallel processing
picode config set analysis.parallel true
picode config set analysis.max_workers 4
```

### Memory Usage Issues

#### High memory consumption

**Problem**: PiCode uses too much memory

**Diagnostic**:
```bash
# Check memory usage
picode status --memory

# Profile memory usage
picode profile --memory --duration 60s
```

**Solutions**:
```bash
# Limit context cache
picode config set cache.max_size "100MB"

# Reduce context window
picode config set max_context_files 50

# Enable garbage collection
picode config set gc.enabled true
picode config set gc.interval "5m"

# Use memory-efficient models
picode config set provider.model "gpt-3.5-turbo"
```

#### Memory leaks

**Problem**: Memory usage grows over time

**Solutions**:
```bash
# Enable aggressive cleanup
picode config set cleanup.aggressive true

# Clear caches periodically
picode cache clear

# Restart session periodically
picode config set session.auto_restart true
picode config set session.max_duration "2h"
```

## Command Issues

### Slash Command Problems

#### Commands not recognized

**Problem**: Slash commands don't work

**Diagnostic**:
```bash
# List available commands
/help

# Check command registration
picode commands list
```

**Solutions**:
```bash
# Ensure interactive mode
picode --interactive

# Check for typos
/help analyze  # instead of /analyz

# Update command cache
picode commands refresh
```

#### Command execution failures

**Problem**: Commands fail to execute

**Diagnostic**:
```bash
# Enable debug mode
picode --debug

# Check command logs
picode logs --filter commands
```

**Solutions**:
```bash
# Check permissions
ls -la ~/.config/picode/

# Verify workspace state
picode workspace status

# Reset workspace if corrupted
picode workspace reset --backup
```

### Context Issues

#### Context too large

**Problem**: "Context window exceeded" errors

**Solutions**:
```bash
# Reduce context automatically
picode config set context.smart_truncation true

# Set smaller context window
picode config set max_context_tokens 4000

# Use selective context
picode config set context.strategy "relevant"

# Exclude large files
picode config set context.max_file_size "100KB"
```

#### Missing context

**Problem**: AI doesn't have enough context

**Solutions**:
```bash
# Force context refresh
/analyze --force-refresh

# Include more files
picode config set context.include_patterns "src/**,tests/**,docs/**"

# Increase context window
picode config set max_context_tokens 8000

# Use deep analysis
picode config set analysis.depth "deep"
```

## Git Integration Problems

### Git Repository Issues

#### Git not detected

**Problem**: PiCode doesn't recognize Git repository

**Diagnostic**:
```bash
# Check Git status
git status

# Check PiCode Git detection
picode git status
```

**Solutions**:
```bash
# Initialize Git repository
git init

# Or specify Git directory
picode config set git.directory ".git"

# Check Git installation
git --version
```

#### Git operations fail

**Problem**: Git commands through PiCode fail

**Diagnostic**:
```bash
# Test Git operations
picode git test

# Check Git configuration
git config --list
```

**Solutions**:
```bash
# Configure Git user
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Fix Git permissions
sudo chown -R $(whoami):$(whoami) .git/

# Reset Git configuration
picode config reset git
```

### Commit Message Generation Issues

#### Poor commit message quality

**Problem**: Generated commit messages are not helpful

**Solutions**:
```bash
# Use conventional commits
picode config set git.commit_format "conventional"

# Provide better context
/commit --analyze-impact

# Customize commit template
picode config set git.commit_template "${type}(${scope}): ${description}"

# Train on good examples
picode git learn-from-history --last 50
```

#### Commit message generation fails

**Problem**: Cannot generate commit messages

**Diagnostic**:
```bash
# Check staged changes
git diff --cached

# Test commit generation
picode git commit-message --dry-run
```

**Solutions**:
```bash
# Ensure changes are staged
git add .

# Use manual message if AI fails
/commit "manual commit message"

# Check provider connection
picode test-connection
```

## WebAssembly Issues

### WASM Compilation Problems

#### Compilation failures

**Problem**: Cannot compile to WebAssembly

**Diagnostic**:
```bash
# Check WASM toolchain
wasm-pack --version

# Test basic compilation
wasm-pack build --target web
```

**Solutions**:
```bash
# Install/update wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install required targets
rustup target add wasm32-unknown-unknown

# Clear build cache
rm -rf target/
cargo clean
```

#### Runtime errors in browser

**Problem**: WASM module fails to run in browser

**Diagnostic**:
Check browser console for errors

**Solutions**:
```bash
# Enable debugging
wasm-pack build --debug

# Check WASM features
picode features --wasm

# Use compatible features only
cargo build --target wasm32-unknown-unknown --no-default-features --features wasm
```

### MCP Server Issues

#### Server won't start

**Problem**: MCP server fails to start

**Diagnostic**:
```bash
# Check port availability
netstat -tulpn | grep 3000

# Test server start
picode --mode mcp --port 3000 --debug
```

**Solutions**:
```bash
# Use different port
picode --mode mcp --port 8080

# Fix permissions
sudo setcap 'cap_net_bind_service=+ep' /usr/local/bin/picode

# Check firewall
sudo ufw allow 3000
```

#### Connection issues

**Problem**: Cannot connect to MCP server

**Diagnostic**:
```bash
# Test server connection
curl http://localhost:3000/mcp/status

# Check server logs
picode logs --filter mcp
```

**Solutions**:
```bash
# Check server address
picode config set mcp.bind_address "0.0.0.0"

# Verify authentication
picode config set mcp.auth_token "your-token"

# Test with direct connection
telnet localhost 3000
```

## Diagnostic Commands

### Built-in Diagnostics

```bash
# Run comprehensive diagnostics
picode doctor

# Test all connections
picode test-all

# Check system status
picode status --verbose

# Validate configuration
picode config validate

# Show version and build info
picode --version --verbose

# Export diagnostic report
picode doctor --export diagnostic-report.json
```

### Debugging Modes

```bash
# Enable debug logging
RUST_LOG=debug picode

# Enable trace logging
RUST_LOG=trace picode

# Debug specific module
RUST_LOG=picode::llm=debug picode

# Save debug output
picode --debug > debug.log 2>&1
```

### Log Analysis

```bash
# View recent logs
picode logs

# Filter by component
picode logs --filter llm,git,commands

# Follow logs in real-time
picode logs --tail

# Export logs
picode logs --export logs-$(date +%Y%m%d).txt
```

## Getting Help

### Documentation Resources

- **User Guide**: [doc/user/USER_GUIDE.md](USER_GUIDE.md)
- **Architecture Documentation**: [doc/developer/ARCHITECTURE.md](../developer/ARCHITECTURE.md)
- **API Reference**: Generated with `cargo doc --open`

### Community Support

- **GitHub Issues**: [Report bugs and request features](https://github.com/pnocera/PiCode/issues)
- **GitHub Discussions**: [Community Q&A](https://github.com/pnocera/PiCode/discussions)
- **Discord**: Real-time community chat (link in README)

### Professional Support

For enterprise users or complex issues:

- **Email Support**: support@picode.org
- **Consulting**: Available for custom implementations
- **Training**: Team training sessions available

### Before Asking for Help

When reporting issues, please include:

1. **PiCode version**: `picode --version`
2. **Operating system**: `uname -a`
3. **Configuration**: `picode config show` (remove sensitive data)
4. **Error logs**: `picode logs --recent`
5. **Steps to reproduce**: Detailed reproduction steps
6. **Expected vs actual behavior**: What you expected vs what happened

### Reporting Bugs

Use this template for bug reports:

```markdown
## Bug Description
Brief description of the bug

## Environment
- OS: [e.g., Ubuntu 22.04]
- PiCode version: [from picode --version]
- Provider: [e.g., OpenAI GPT-4]

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Expected Behavior
What should have happened

## Actual Behavior
What actually happened

## Logs
```
Paste relevant logs here (remove sensitive information)
```

## Additional Context
Any other relevant information
```

Remember: The PiCode community is here to help! Don't hesitate to ask questions or report issues. Your feedback helps make PiCode better for everyone.