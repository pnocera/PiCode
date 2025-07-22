# Reference Documentation

> 📚 **Complete Feature Reference** - Detailed specifications for all PiCode functionality

Comprehensive reference documentation for commands, configuration, and features.

## Command Reference

### 🖥️ [CLI Commands](cli.md)
Complete command-line interface reference
- Global options and flags
- Subcommand specifications
- Exit codes and error handling
- Environment variable integration

### ⚙️ [Configuration](config.md)
All configuration options and formats
- Configuration file structure
- Provider-specific settings
- Performance and behavior tuning
- Validation and defaults

### 🪝 [Hooks System](hooks.md)
Pre and post-operation hook reference
- Available hook points
- Custom hook development
- Hook configuration
- Built-in hook examples

## System Reference

### 🌍 [Environment Variables](environment.md)
All supported environment variables
- API key management
- Runtime configuration
- Debug and logging options
- Platform-specific variables

### 📁 [File Formats](file-formats.md)
Supported file formats and structures
- Configuration files (.toml, .yaml, .json)
- Context definition format
- Project template structure
- Plugin manifest format

### 🔌 [Plugin API](plugin-api.md)
Plugin development interface reference
- WASM plugin specification
- Host function bindings
- Event system
- Resource management

## Error Reference

### 🚨 [Error Codes](error-codes.md)
Complete error code reference
- System errors (100-199)
- Configuration errors (200-299)
- Provider errors (300-399)
- Plugin errors (400-499)

### 🔍 [Diagnostic Messages](diagnostics.md)
Detailed diagnostic information
- Warning messages
- Performance hints
- Security advisories
- Deprecation notices

## Quick Reference Cards

### 🎯 [Slash Commands Quick Reference](quick-ref/slash-commands.md)
```
/help           Show available commands
/context <path> Add files/directories to context
/model <name>   Switch LLM model
/provider <name> Switch LLM provider
/save [name]    Save current conversation
/load <name>    Load saved conversation
/clear          Clear conversation history
/exit           Exit interactive mode
```

### ⌨️ [CLI Quick Reference](quick-ref/cli-commands.md)
```bash
picode interactive          # Start interactive mode
picode analyze [path]       # Analyze project/files
picode generate <desc>      # Generate code
picode review [files]       # Code review
picode config <key> <val>   # Configure settings
picode test-connection      # Test LLM connection
```

### 📝 [Configuration Quick Reference](quick-ref/config-options.md)
```toml
[general]
provider = "openai"         # LLM provider
model = "gpt-4"            # Model name
temperature = 0.7          # Response creativity
max_tokens = 2000          # Response length limit

[interface]
interactive_mode = true    # Enable interactive mode
auto_context = true        # Automatic context detection
syntax_highlighting = true # Code highlighting
```

## Compatibility Reference

### 🏗️ [Platform Support](compatibility.md#platforms)
- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)
- WebAssembly (WASI)

### 🤖 [LLM Provider Support](compatibility.md#providers)
- OpenAI (GPT-3.5, GPT-4, GPT-4 Turbo)
- Anthropic (Claude 3 Sonnet, Haiku, Opus)
- Google (Gemini Pro, Gemini Ultra)
- Local models (Ollama, LocalAI)

### 🗂️ [File Format Support](compatibility.md#formats)
- Source code: Rust, Python, JavaScript, TypeScript, Go, Java, C++
- Configuration: TOML, YAML, JSON, INI
- Documentation: Markdown, reStructuredText
- Data: CSV, JSON, XML

---

**Quick Access**: [CLI Reference](cli.md) • [Config Reference](config.md) • [Error Codes](error-codes.md)