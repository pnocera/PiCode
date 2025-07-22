# Contributing to PiCode

Thank you for your interest in contributing to PiCode! This guide will help you get started with contributing to our terminal-native AI workspace.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Setup](#development-setup)
3. [Project Structure](#project-structure)
4. [Contributing Guidelines](#contributing-guidelines)
5. [Testing](#testing)
6. [Documentation](#documentation)
7. [Pull Request Process](#pull-request-process)
8. [Code Review](#code-review)
9. [Release Process](#release-process)
10. [Community Guidelines](#community-guidelines)

## Getting Started

### Prerequisites

- **Rust**: 1.84 or later
- **Git**: 2.30 or later
- **OpenSSL**: For HTTPS connections
- **WebAssembly toolchain**: `wasm-pack` for WASM builds
- **Node.js**: 18+ for documentation builds
- **Docker**: Optional, for containerized testing

### Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/PiCode.git
   cd PiCode
   ```
3. **Set up the development environment**:
   ```bash
   ./scripts/setup-dev.sh
   ```
4. **Run tests** to ensure everything works:
   ```bash
   cargo test
   ```
5. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Environment Configuration

Create a `.env` file in the project root:
```bash
# LLM Provider for testing
OPENAI_API_KEY=your_key_here
ANTHROPIC_API_KEY=your_key_here

# Development settings
RUST_LOG=debug
PICODE_ENV=development
PICODE_TEST_MODE=true
```

### Rust Toolchain Setup

```bash
# Install required components
rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown

# Install additional tools
cargo install cargo-audit cargo-expand wasm-pack
cargo install --force cargo-make
```

### IDE Configuration

#### VS Code

Recommended extensions:
- `rust-analyzer`: Rust language support
- `CodeLLDB`: Debugging support
- `Better TOML`: TOML syntax highlighting
- `Error Lens`: Inline error display

#### Vim/Neovim

Configuration for `coc.nvim`:
```json
{
  "rust-analyzer.server.path": "rust-analyzer",
  "rust-analyzer.cargo.features": "all"
}
```

### Build System

PiCode uses `cargo-make` for task automation:

```bash
# Available tasks
cargo make --list-all-steps

# Development build
cargo make dev

# Release build  
cargo make release

# Run all tests
cargo make test-all

# Format code
cargo make format

# Lint code
cargo make lint

# Generate documentation
cargo make docs
```

## Project Structure

```
PiCode/
├── src/                     # Main application
│   ├── cli.rs              # CLI interface
│   ├── config.rs           # Configuration management
│   ├── error.rs            # Error handling
│   └── main.rs             # Application entry point
├── picode-core/            # Core workspace logic
│   ├── src/
│   │   ├── command.rs      # Command processing
│   │   ├── event.rs        # Event handling
│   │   ├── pane.rs         # Pane management
│   │   ├── session.rs      # Session management
│   │   └── workspace.rs    # Workspace coordination
│   └── Cargo.toml
├── picode-cli/             # CLI components
│   ├── src/
│   │   ├── args.rs         # Argument parsing
│   │   └── commands.rs     # CLI command implementations
│   └── Cargo.toml
├── picode-llm/             # LLM integration
│   ├── src/
│   │   ├── client.rs       # HTTP client
│   │   ├── openapi.rs      # OpenAPI handling
│   │   └── providers.rs    # Provider implementations
│   └── Cargo.toml
├── picode-hooks/           # Hook system
│   ├── src/
│   │   ├── hooks.rs        # Hook definitions
│   │   └── registry.rs     # Hook registration
│   └── Cargo.toml
├── picode-wasm/            # WebAssembly bindings
│   ├── src/lib.rs
│   └── Cargo.toml
├── tests/                  # Integration tests
├── benches/                # Benchmarks
├── examples/               # Example code
├── doc/                    # Documentation
└── scripts/                # Build and utility scripts
```

### Crate Organization

- **`picode`**: Main binary crate that orchestrates all components
- **`picode-core`**: Core terminal workspace functionality
- **`picode-cli`**: Command-line interface and argument parsing
- **`picode-llm`**: LLM provider integration and OpenAPI handling
- **`picode-hooks`**: Extension system and custom workflow hooks
- **`picode-wasm`**: WebAssembly bindings for browser/MCP deployment

## Contributing Guidelines

### Code Style

We follow Rust standard conventions with some additional guidelines:

#### Formatting
- Use `cargo fmt` for automatic formatting
- 100-character line limit (configurable in `.rustfmt.toml`)
- Use trailing commas in multi-line constructs

#### Naming Conventions
- `snake_case` for variables, functions, and modules
- `PascalCase` for types, structs, and enums
- `SCREAMING_SNAKE_CASE` for constants
- Descriptive names over clever abbreviations

#### Error Handling
- Use `anyhow::Result<T>` for functions that can fail
- Implement custom error types with `thiserror` for libraries
- Avoid `unwrap()` and `expect()` in production code
- Use `?` operator for error propagation

#### Documentation
- Public APIs must have rustdoc comments
- Use `///` for function/struct documentation
- Use `//!` for module-level documentation
- Include examples in documentation when appropriate

### Architecture Principles

#### Modularity
- Each crate should have a single, well-defined responsibility
- Minimize dependencies between crates
- Use dependency injection for testability

#### Error Handling
- Errors should be structured and actionable
- Provide context for error debugging
- Fail fast for unrecoverable errors
- Graceful degradation for non-critical failures

#### Performance
- Minimize allocations in hot paths
- Use `Cow<str>` for string handling when appropriate
- Profile code changes that affect performance
- Document performance characteristics

#### Security
- Validate all external inputs
- Use secure defaults for configuration
- Avoid logging sensitive information
- Follow OWASP guidelines for web components

### Commit Guidelines

We use [Conventional Commits](https://www.conventionalcommits.org/) specification:

#### Commit Types
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring without functional changes
- `perf`: Performance improvements
- `test`: Test additions or modifications
- `chore`: Build process or auxiliary tool changes
- `ci`: CI configuration changes

#### Commit Format
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Examples
```
feat(llm): add support for custom OpenAPI providers

- Implement dynamic client generation from OpenAPI specs
- Add authentication method detection
- Include comprehensive error handling

Closes #123
```

```
fix(cli): resolve argument parsing edge case

The CLI was not properly handling empty string arguments in interactive mode.
Added validation and appropriate error messages.

Fixes #456
```

### Branch Naming

Use descriptive branch names with prefixes:
- `feature/description`: New features
- `fix/description`: Bug fixes
- `refactor/description`: Code refactoring
- `docs/description`: Documentation improvements
- `test/description`: Test improvements

Examples:
- `feature/openapi-provider-support`
- `fix/cli-argument-parsing`
- `refactor/error-handling-restructure`

## Testing

### Testing Philosophy

- **Unit tests**: Test individual functions and components
- **Integration tests**: Test component interactions
- **End-to-end tests**: Test complete user workflows
- **Property-based tests**: Test invariants and edge cases
- **Performance tests**: Ensure performance regressions don't occur

### Test Organization

```
tests/
├── integration/           # Integration tests
│   ├── cli_tests.rs      # CLI integration tests
│   ├── llm_tests.rs      # LLM provider tests
│   └── workspace_tests.rs # Workspace tests
├── fixtures/             # Test data
│   ├── projects/         # Sample projects
│   └── openapi_specs/    # OpenAPI specifications
└── common/               # Shared test utilities
    └── mod.rs
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_cli_basic_commands

# Run integration tests only
cargo test --test integration

# Run tests with coverage
cargo tarpaulin --out Html

# Run performance tests
cargo test --release -- --ignored perf
```

### Writing Tests

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_openapi_spec() {
        let spec_json = r#"{"openapi": "3.0.0", ...}"#;
        let spec = parse_openapi_spec(spec_json).unwrap();
        assert_eq!(spec.openapi, "3.0.0");
    }

    #[tokio::test]
    async fn test_llm_client_request() {
        let client = LLMClient::new("test_provider").await;
        let response = client.complete("Hello").await;
        assert!(response.is_ok());
    }
}
```

#### Integration Tests
```rust
// tests/integration/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_analyze_command() {
    let temp_dir = TempDir::new().unwrap();
    
    Command::cargo_bin("picode")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["analyze", "src/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Analysis complete"));
}
```

#### Property-Based Tests
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_config_serialization_roundtrip(
        config in any::<Config>()
    ) {
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }
}
```

### Mock and Test Utilities

For testing LLM providers, use mock servers:

```rust
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_openai_client() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v1/completions"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "choices": [{"text": "Hello, world!"}]
            })))
        .mount(&mock_server)
        .await;

    let client = OpenAIClient::new(mock_server.uri());
    let result = client.complete("Hello").await.unwrap();
    assert_eq!(result.text, "Hello, world!");
}
```

## Documentation

### Code Documentation

- All public APIs must have rustdoc comments
- Include examples in documentation
- Document error conditions and edge cases
- Use `#[doc(hidden)]` for internal APIs

```rust
/// Analyzes a codebase and returns insights about its structure and quality.
///
/// # Arguments
///
/// * `path` - The root path of the codebase to analyze
/// * `options` - Analysis configuration options
///
/// # Returns
///
/// Returns an `AnalysisResult` containing insights about the codebase,
/// including metrics, suggestions, and identified issues.
///
/// # Errors
///
/// This function will return an error if:
/// - The path doesn't exist or isn't readable
/// - The analysis timeout is exceeded
/// - An I/O error occurs while reading files
///
/// # Examples
///
/// ```
/// use picode_core::{analyze_codebase, AnalysisOptions};
/// 
/// let options = AnalysisOptions::default();
/// let result = analyze_codebase("./src", options)?;
/// println!("Found {} issues", result.issues.len());
/// ```
pub fn analyze_codebase(path: &Path, options: AnalysisOptions) -> Result<AnalysisResult> {
    // Implementation
}
```

### User Documentation

User-facing documentation is written in Markdown and organized in the `doc/` directory:

- **User guides**: Step-by-step instructions for end users
- **Developer guides**: Technical documentation for contributors
- **API reference**: Generated from rustdoc comments

### Building Documentation

```bash
# Generate API documentation
cargo doc --open

# Build user documentation
cargo make docs

# Serve documentation locally
cargo make serve-docs
```

## Pull Request Process

### Before Submitting

1. **Ensure tests pass**: Run `cargo test` and fix any failures
2. **Run linting**: Execute `cargo clippy` and address warnings
3. **Format code**: Run `cargo fmt` to ensure consistent formatting
4. **Update documentation**: Add or update relevant documentation
5. **Write descriptive commit messages**: Follow conventional commit format

### Pull Request Template

When creating a pull request, use this template:

```markdown
## Description
Brief description of the changes and their purpose.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed
- [ ] Performance impact assessed

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes

## Related Issues
Closes #issue_number
```

### Review Process

1. **Automated checks**: CI/CD pipeline runs automatically
2. **Code review**: At least one maintainer reviews the code
3. **Testing verification**: Reviewers verify tests are adequate
4. **Documentation review**: Ensure documentation is updated
5. **Final approval**: Maintainer approves and merges

### Merge Criteria

Pull requests must meet these criteria to be merged:

- ✅ All CI checks pass
- ✅ Code review approved by maintainer
- ✅ No merge conflicts
- ✅ Tests provide adequate coverage
- ✅ Documentation is updated
- ✅ Follows project conventions

## Code Review

### As an Author

When your code is being reviewed:

- **Be responsive**: Address feedback promptly
- **Be open to suggestions**: Consider reviewer feedback seriously
- **Explain your reasoning**: Help reviewers understand your approach
- **Make requested changes**: Update code based on feedback
- **Test thoroughly**: Ensure changes don't break existing functionality

### As a Reviewer

When reviewing code:

- **Be constructive**: Provide helpful, actionable feedback
- **Focus on important issues**: Distinguish between major and minor issues
- **Explain reasoning**: Help authors understand your concerns
- **Acknowledge good code**: Praise well-written code
- **Test the changes**: Verify functionality when possible

### Review Checklist

#### Functionality
- [ ] Code solves the intended problem
- [ ] Edge cases are handled appropriately
- [ ] Error conditions are managed properly
- [ ] Performance implications are reasonable

#### Code Quality
- [ ] Code is readable and well-structured
- [ ] Functions and variables have descriptive names
- [ ] Complex logic is commented
- [ ] Code follows project conventions

#### Testing
- [ ] Adequate test coverage
- [ ] Tests are meaningful and comprehensive
- [ ] Tests will catch regressions
- [ ] Mock objects are used appropriately

#### Security
- [ ] Input validation is present
- [ ] No sensitive information is logged
- [ ] Authentication/authorization is correct
- [ ] Dependencies are up-to-date and secure

## Release Process

### Versioning

PiCode follows [Semantic Versioning](https://semver.org/):

- **Major version** (X.0.0): Breaking changes
- **Minor version** (0.X.0): New features, backward compatible
- **Patch version** (0.0.X): Bug fixes, backward compatible

### Release Checklist

#### Pre-release
- [ ] All tests pass on CI
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version numbers are bumped
- [ ] Release notes are prepared

#### Release
- [ ] Create release tag
- [ ] Build release artifacts
- [ ] Publish to crates.io
- [ ] Create GitHub release
- [ ] Update documentation website

#### Post-release
- [ ] Announce release
- [ ] Update examples and tutorials
- [ ] Monitor for issues
- [ ] Plan next release

### Release Automation

```bash
# Prepare release
cargo make release-prepare --version 0.2.0

# Create release (maintainers only)
cargo make release --version 0.2.0

# Publish to crates.io (maintainers only)
cargo make publish
```

## Community Guidelines

### Code of Conduct

We are committed to providing a friendly, safe, and welcoming environment for all. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

### Communication

- **GitHub Issues**: Bug reports, feature requests, and discussions
- **GitHub Discussions**: General questions and community discussions
- **Discord**: Real-time community chat (link in README)
- **Email**: Direct contact with maintainers

### Getting Help

If you need help:

1. **Check the documentation**: Start with the user guide
2. **Search existing issues**: Your question might already be answered
3. **Ask in discussions**: Use GitHub Discussions for general questions
4. **Create an issue**: For bugs or specific feature requests

### Recognition

Contributors are recognized in several ways:

- **Contributors file**: Listed in CONTRIBUTORS.md
- **Release notes**: Mentioned in significant releases  
- **Documentation**: Credited in relevant documentation
- **Community roles**: Active contributors may be invited to maintainer roles

### Maintainer Responsibilities

Maintainers are responsible for:

- **Code review**: Reviewing and merging pull requests
- **Issue triage**: Labeling and prioritizing issues
- **Release management**: Planning and executing releases
- **Community building**: Fostering a welcoming community
- **Technical direction**: Guiding project architecture and features

## Development Workflow

### Daily Development

1. **Sync with upstream**:
   ```bash
   git checkout main
   git pull upstream main
   ```

2. **Create feature branch**:
   ```bash
   git checkout -b feature/my-feature
   ```

3. **Make changes**, following the guidelines above

4. **Test changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Commit changes**:
   ```bash
   git add .
   git commit -m "feat: add my awesome feature"
   ```

6. **Push and create PR**:
   ```bash
   git push origin feature/my-feature
   # Create pull request on GitHub
   ```

### Debugging

Use these tools for debugging:

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Use the debugger
cargo build
lldb target/debug/picode

# Profile performance
cargo flamegraph --bin picode
```

### Performance Profiling

```bash
# CPU profiling
cargo install flamegraph
cargo flamegraph --bin picode -- analyze large_project/

# Memory profiling  
cargo install heaptrack
heaptrack target/release/picode analyze large_project/

# Benchmark specific functions
cargo bench
```

Thank you for contributing to PiCode! Your contributions help make AI-assisted development accessible to everyone. 🚀