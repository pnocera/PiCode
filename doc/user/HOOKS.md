# PiCode Hooks System

PiCode's hook system allows you to customize and extend your development workflow by automatically triggering actions at specific points during operations. This guide covers everything you need to know about using and creating hooks.

## Table of Contents

1. [Hook Types](#hook-types)
2. [Built-in Hooks](#built-in-hooks)
3. [Configuration](#configuration)
4. [Custom Hooks](#custom-hooks)
5. [Hook Examples](#hook-examples)
6. [Advanced Usage](#advanced-usage)
7. [Troubleshooting](#troubleshooting)

## Hook Types

PiCode supports several types of hooks that execute at different points in your workflow:

### Pre-Operation Hooks

Execute before operations begin, allowing validation and preparation:

- **`pre_edit`**: Before file editing operations
- **`pre_commit`**: Before Git commit operations
- **`pre_push`**: Before Git push operations
- **`pre_analyze`**: Before code analysis
- **`pre_test`**: Before running tests
- **`pre_command`**: Before executing any shell command

### Post-Operation Hooks

Execute after operations complete, enabling cleanup and follow-up actions:

- **`post_edit`**: After file editing operations
- **`post_commit`**: After Git commit operations
- **`post_push`**: After Git push operations
- **`post_analyze`**: After code analysis
- **`post_test`**: After running tests
- **`post_command`**: After executing shell commands

### Event Hooks

Triggered by specific events in your workflow:

- **`file_created`**: When new files are created
- **`file_deleted`**: When files are deleted
- **`project_opened`**: When opening a project
- **`session_start`**: When starting PiCode
- **`session_end`**: When exiting PiCode

### Custom Hooks

User-defined hooks for specific use cases:

- **`deploy`**: Custom deployment workflows
- **`backup`**: Backup operations
- **`notify`**: Notification systems
- **`security_scan`**: Security scanning operations

## Built-in Hooks

PiCode comes with several useful built-in hooks:

### Code Formatting Hooks

Automatically format code when files are edited:

```toml
[hooks.formatting]
pre_edit = ["format_check"]
post_edit = ["auto_format"]

[hooks.custom.format_check]
command = "cargo fmt -- --check"
description = "Check if code is properly formatted"
fail_on_error = true

[hooks.custom.auto_format]
command = "cargo fmt"
description = "Automatically format code"
async = true
```

### Testing Hooks

Ensure tests pass before commits:

```toml
[hooks.testing]
pre_commit = ["run_tests"]
post_edit = ["test_related"]

[hooks.custom.run_tests]
command = "cargo test"
description = "Run all tests"
timeout = 300
fail_on_error = true

[hooks.custom.test_related]
command = "cargo test -- ${edited_files}"
description = "Run tests for edited files"
async = true
conditional = true
```

### Linting Hooks

Maintain code quality with automatic linting:

```toml
[hooks.linting]
pre_commit = ["lint_check"]
post_edit = ["lint_fix"]

[hooks.custom.lint_check]
command = "cargo clippy -- -D warnings"
description = "Check for linting issues"
fail_on_error = true

[hooks.custom.lint_fix]
command = "cargo clippy --fix --allow-dirty"
description = "Auto-fix linting issues"
async = true
```

### Security Hooks

Automated security scanning:

```toml
[hooks.security]
pre_commit = ["security_audit"]
pre_push = ["dependency_check"]

[hooks.custom.security_audit]
command = "cargo audit"
description = "Check for security vulnerabilities"
fail_on_error = false
notify_on_issues = true

[hooks.custom.dependency_check]
command = "cargo outdated"
description = "Check for outdated dependencies"
async = true
report_results = true
```

## Configuration

### Configuration File

Hooks are configured in your `picode.toml` configuration file:

```toml
# ~/.config/picode/config.toml or ./picode.toml

[hooks]
# Enable/disable hooks globally
enabled = true

# Timeout for hook execution (seconds)
default_timeout = 60

# Continue on hook failures
fail_fast = false

# Log hook execution
verbose_logging = true

# Hook-specific configuration
pre_edit = ["backup", "format_check"]
post_edit = ["auto_format", "lint_fix"]
pre_commit = ["run_tests", "security_audit"]
post_commit = ["notify", "backup"]

# Custom hook definitions
[hooks.custom.backup]
command = "cp ${file} ${file}.bak"
description = "Create backup of edited file"
enabled = true
async = false

[hooks.custom.notify]
command = "echo 'Commit completed: ${commit_message}' | notify-send"
description = "Send desktop notification"
enabled = true
async = true
platform = "linux"  # Only run on Linux
```

### Environment Variables

Control hooks through environment variables:

```bash
# Disable all hooks
PICODE_HOOKS_ENABLED=false picode

# Disable specific hook types
PICODE_PRE_COMMIT_HOOKS=false picode

# Override hook timeout
PICODE_HOOK_TIMEOUT=120 picode

# Enable verbose hook logging
PICODE_HOOK_VERBOSE=true picode
```

### Command-Line Options

Control hooks via command-line arguments:

```bash
# Disable hooks for this session
picode --no-hooks

# Run only specific hooks
picode --hooks pre_edit,post_edit

# Set hook timeout
picode --hook-timeout 300

# Enable hook debugging
picode --hook-debug
```

## Custom Hooks

### Shell Script Hooks

Create custom shell script hooks:

```bash
#!/bin/bash
# ~/.config/picode/hooks/custom_deploy.sh

set -e

echo "Deploying application..."

# Build release version
cargo build --release

# Run tests
cargo test --release

# Create deployment package
tar -czf deploy.tar.gz target/release/picode

# Upload to server (example)
scp deploy.tar.gz user@server:/path/to/deploy/

echo "Deployment completed successfully!"
```

Register the hook:

```toml
[hooks.custom.deploy]
command = "~/.config/picode/hooks/custom_deploy.sh"
description = "Deploy application to server"
timeout = 600
async = false
```

### Python Script Hooks

Use Python for more complex hook logic:

```python
#!/usr/bin/env python3
# ~/.config/picode/hooks/analyze_complexity.py

import os
import sys
import subprocess
import json

def analyze_file_complexity(file_path):
    """Analyze code complexity using radon"""
    try:
        result = subprocess.run(
            ["radon", "cc", "-j", file_path],
            capture_output=True,
            text=True,
            check=True
        )
        
        complexity_data = json.loads(result.stdout)
        return complexity_data
    except subprocess.CalledProcessError:
        return None

def main():
    file_path = sys.argv[1] if len(sys.argv) > 1 else None
    if not file_path or not os.path.exists(file_path):
        print("Invalid file path")
        sys.exit(1)
    
    complexity = analyze_file_complexity(file_path)
    if complexity:
        # Check if complexity is too high
        for item in complexity.get(file_path, []):
            if item.get('complexity', 0) > 10:
                print(f"High complexity detected in {file_path}: {item['complexity']}")
                sys.exit(1)
    
    print(f"Complexity check passed for {file_path}")
    sys.exit(0)

if __name__ == "__main__":
    main()
```

Register the Python hook:

```toml
[hooks.custom.complexity_check]
command = "python3 ~/.config/picode/hooks/analyze_complexity.py ${file}"
description = "Check code complexity"
timeout = 30
fail_on_error = true
```

### Rust Binary Hooks

Create hooks as compiled Rust binaries for maximum performance:

```rust
// ~/.config/picode/hooks/src/main.rs

use std::env;
use std::process::{Command, exit};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }
    
    let file_path = &args[1];
    
    match validate_code_style(file_path) {
        Ok(()) => {
            println!("Code style validation passed");
            exit(0);
        }
        Err(e) => {
            eprintln!("Code style validation failed: {}", e);
            exit(1);
        }
    }
}

fn validate_code_style(file_path: &str) -> Result<(), String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Check for TODO comments
    if content.contains("TODO") {
        return Err("TODO comments found".to_string());
    }
    
    // Check for debugging prints
    if content.contains("println!") || content.contains("dbg!") {
        return Err("Debug prints found".to_string());
    }
    
    // Check line length
    for (line_num, line) in content.lines().enumerate() {
        if line.len() > 100 {
            return Err(format!("Line {} exceeds 100 characters", line_num + 1));
        }
    }
    
    Ok(())
}
```

Compile and register:

```bash
cd ~/.config/picode/hooks
cargo build --release

# Register in config
```

```toml
[hooks.custom.style_check]
command = "~/.config/picode/hooks/target/release/style_validator ${file}"
description = "Validate code style"
timeout = 10
fail_on_error = true
```

## Hook Examples

### Git Workflow Integration

Comprehensive Git workflow with hooks:

```toml
[hooks.git_workflow]
pre_commit = ["format_check", "lint_check", "test_check"]
post_commit = ["update_changelog", "tag_version"]
pre_push = ["integration_tests", "security_scan"]
post_push = ["deploy_staging", "notify_team"]

[hooks.custom.update_changelog]
command = "conventional-changelog -p angular -i CHANGELOG.md -s"
description = "Update changelog from commits"
async = true

[hooks.custom.tag_version]
command = "git tag v$(cargo metadata --format-version 1 | jq -r '.packages[0].version')"
description = "Tag new version"
conditional = true

[hooks.custom.deploy_staging]
command = "./scripts/deploy-staging.sh"
description = "Deploy to staging environment"
async = true
timeout = 300
```

### Continuous Integration Hooks

Local CI/CD simulation:

```toml
[hooks.ci_simulation]
pre_push = ["build_matrix", "test_matrix", "package_artifacts"]

[hooks.custom.build_matrix]
command = "./scripts/build-matrix.sh"
description = "Build for all target platforms"
timeout = 600
parallel = true

[hooks.custom.test_matrix]
command = "./scripts/test-matrix.sh"  
description = "Test across multiple configurations"
timeout = 900
depends_on = ["build_matrix"]

[hooks.custom.package_artifacts]
command = "./scripts/package.sh"
description = "Package build artifacts"
depends_on = ["build_matrix", "test_matrix"]
```

### Documentation Generation

Automatic documentation updates:

```toml
[hooks.documentation]
post_edit = ["update_docs"]
pre_commit = ["docs_check"]

[hooks.custom.update_docs]
command = "cargo doc --no-deps && mdbook build doc/"
description = "Update documentation"
async = true
file_patterns = ["src/**/*.rs", "doc/**/*.md"]

[hooks.custom.docs_check]
command = "cargo doc --no-deps 2>&1 | grep -q warning && exit 1 || exit 0"
description = "Check for documentation warnings"
fail_on_error = true
```

### Performance Monitoring

Track performance regressions:

```toml
[hooks.performance]
post_test = ["benchmark_check"]
pre_push = ["performance_baseline"]

[hooks.custom.benchmark_check]
command = "cargo bench --bench main_bench | tee benchmark_results.txt"
description = "Run performance benchmarks"
store_results = true

[hooks.custom.performance_baseline]
command = "./scripts/compare-benchmarks.sh"
description = "Compare with performance baseline"
fail_on_error = false
report_results = true
```

## Advanced Usage

### Conditional Hooks

Execute hooks based on conditions:

```toml
[hooks.custom.rust_specific]
command = "cargo clippy"
description = "Run Rust linting"
condition = "file_extension == 'rs'"
enabled = true

[hooks.custom.production_deploy]
command = "./scripts/deploy-prod.sh"
description = "Deploy to production"
condition = "branch == 'main' && has_tag"
async = true

[hooks.custom.large_file_check]
command = "./scripts/check-file-size.sh ${file}"
description = "Check for large files"
condition = "file_size > 1MB"
fail_on_error = true
```

### Hook Dependencies

Define execution order with dependencies:

```toml
[hooks.custom.compile_check]
command = "cargo check"
description = "Check compilation"
priority = 100

[hooks.custom.test_run]  
command = "cargo test"
description = "Run tests"
depends_on = ["compile_check"]
priority = 200

[hooks.custom.integration_test]
command = "cargo test --test integration"
description = "Run integration tests"
depends_on = ["test_run"]
priority = 300
```

### Parallel Hook Execution

Execute hooks in parallel for performance:

```toml
[hooks.parallel_group]
pre_commit = ["lint_check", "format_check", "type_check"]

[hooks.custom.lint_check]
command = "cargo clippy"
parallel_group = "pre_commit_checks"

[hooks.custom.format_check]
command = "cargo fmt -- --check"
parallel_group = "pre_commit_checks"

[hooks.custom.type_check]
command = "cargo check"
parallel_group = "pre_commit_checks"
```

### Hook Context Variables

Access context information in hooks:

```toml
[hooks.custom.context_aware]
command = "echo 'Editing ${file} in ${project} on ${branch}'"
description = "Context-aware hook"
```

Available variables:
- `${file}`: Current file path
- `${project}`: Project root directory
- `${branch}`: Current Git branch
- `${commit}`: Latest commit hash
- `${author}`: Git author name
- `${timestamp}`: Current timestamp
- `${operation}`: Current operation (edit, commit, etc.)

### Hook Result Processing

Process hook results and chain operations:

```toml
[hooks.custom.test_with_coverage]
command = "cargo test --coverage"
description = "Run tests with coverage"
capture_output = true
process_results = "coverage_processor"

[hooks.processors.coverage_processor]
type = "script"
script = "./scripts/process-coverage.sh"
input = "hook_output"
output = "coverage_report"
```

## Troubleshooting

### Common Issues

#### Hooks Not Running

Check if hooks are enabled:

```bash
picode config get hooks.enabled
# Should return: true
```

Verify hook configuration:

```bash
picode hooks list
picode hooks status
```

#### Hook Execution Failures

Enable verbose logging:

```bash
PICODE_HOOK_VERBOSE=true picode
```

Check hook logs:

```bash
picode logs --filter hooks
```

Debug specific hook:

```bash
picode hooks debug pre_commit
```

#### Performance Issues

Identify slow hooks:

```bash
picode hooks benchmark
```

Optimize hook execution:

```toml
[hooks]
# Enable parallel execution
parallel_execution = true

# Increase timeout for slow hooks
default_timeout = 120

# Cache hook results
cache_results = true
cache_duration = "1h"
```

### Debugging Hooks

#### Enable Debug Mode

```bash
# Run with hook debugging
picode --hook-debug

# Or set environment variable
PICODE_HOOK_DEBUG=true picode
```

#### Hook Execution Tracing

```toml
[hooks]
trace_execution = true
log_level = "debug"
save_execution_log = true
log_file = "~/.picode/hooks.log"
```

#### Manual Hook Testing

Test individual hooks:

```bash
# Test a specific hook
picode hooks test pre_commit

# Test hook with specific file
picode hooks test post_edit --file src/main.rs

# Test all hooks in dry-run mode
picode hooks test --all --dry-run
```

### Best Practices

#### Hook Development

1. **Keep hooks fast**: Hooks should complete quickly to avoid slowing down workflows
2. **Make hooks idempotent**: Hooks should be safe to run multiple times
3. **Handle errors gracefully**: Provide clear error messages and recovery options
4. **Use appropriate timeouts**: Set realistic timeouts for hook operations
5. **Test thoroughly**: Test hooks in various scenarios before deployment

#### Configuration Management

1. **Use version control**: Keep hook configurations in version control
2. **Document hooks**: Add descriptions to all custom hooks
3. **Environment-specific configs**: Use different configurations for different environments
4. **Regular maintenance**: Review and update hooks regularly

#### Security Considerations

1. **Validate inputs**: Always validate file paths and other inputs
2. **Limit permissions**: Run hooks with minimal required permissions
3. **Avoid secrets in hooks**: Don't include API keys or passwords in hook commands
4. **Audit hook changes**: Review hook modifications carefully

The PiCode hooks system provides powerful workflow automation capabilities while maintaining simplicity and flexibility. Use hooks to streamline your development process and ensure consistent code quality across your projects.