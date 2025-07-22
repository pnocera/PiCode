# PiCode Slash Commands Reference

This document provides a comprehensive reference for all slash commands available in PiCode's interactive mode.

## Command Categories

- [Analysis Commands](#analysis-commands)
- [Code Assistance](#code-assistance)
- [Testing Commands](#testing-commands)
- [Git Commands](#git-commands)
- [Project Management](#project-management)
- [Configuration Commands](#configuration-commands)
- [System Commands](#system-commands)
- [Workflow Commands](#workflow-commands)

---

## Analysis Commands

Commands for understanding and analyzing your codebase.

### `/analyze [path] [options]`

Performs comprehensive analysis of code files or directories.

**Usage:**
```
/analyze                           # Analyze entire project
/analyze src/                      # Analyze specific directory
/analyze src/main.rs              # Analyze single file
/analyze --focus security         # Focus on specific aspects
/analyze --depth shallow          # Control analysis depth
```

**Options:**
- `--focus <aspect>`: Focus analysis on specific areas
  - `security`: Security vulnerabilities and best practices
  - `performance`: Performance issues and optimizations
  - `maintainability`: Code maintainability metrics
  - `complexity`: Code complexity analysis
  - `dependencies`: Dependency analysis
- `--depth <level>`: Analysis depth (`shallow`, `medium`, `deep`)
- `--format <type>`: Output format (`text`, `json`, `markdown`)
- `--save <file>`: Save results to file

**Examples:**
```
/analyze src/auth.rs --focus security --depth deep
/analyze --format json --save analysis.json
/analyze tests/ --focus coverage
```

### `/summary [scope]`

Generates a high-level summary of the project or specific components.

**Usage:**
```
/summary                    # Full project summary
/summary src/              # Directory summary
/summary --architecture    # Architectural overview
/summary --changes         # Recent changes summary
```

**Options:**
- `--architecture`: Focus on system architecture
- `--changes [since]`: Summarize changes since commit/date
- `--metrics`: Include quantitative metrics
- `--brief`: Condensed summary

### `/metrics [path] [options]`

Display code metrics and statistics.

**Usage:**
```
/metrics                    # Project-wide metrics
/metrics src/handlers/     # Directory metrics
/metrics --complexity      # Focus on complexity metrics
/metrics --history         # Show metric trends
```

**Metrics Included:**
- Lines of code (LOC)
- Cyclomatic complexity
- Coupling and cohesion
- Test coverage
- Technical debt
- Duplication percentage

### `/dependencies [options]`

Analyze project dependencies and their relationships.

**Usage:**
```
/dependencies                    # Show dependency tree
/dependencies --outdated        # Find outdated dependencies
/dependencies --vulnerabilities # Security vulnerability scan
/dependencies --licenses        # License compatibility check
```

### `/search <query> [options]`

Intelligent code search across the project.

**Usage:**
```
/search "authentication logic"           # Semantic search
/search "TODO" --type comments          # Search in comments
/search "unsafe" --lang rust            # Language-specific search
/search "database connection" --recent  # Recent changes only
```

**Search Types:**
- `semantic`: Meaning-based search (default)
- `literal`: Exact string matching
- `regex`: Regular expression search
- `fuzzy`: Approximate matching

---

## Code Assistance

Commands for AI-powered code generation, editing, and improvement.

### `/edit <file> [instructions]`

AI-assisted code editing with specific instructions.

**Usage:**
```
/edit src/main.rs                           # Interactive editing
/edit src/auth.rs --add error-handling     # Add specific functionality
/edit src/api.rs "optimize for performance" # Natural language instruction
/edit --selection 23-45 "refactor this"    # Edit specific lines
```

**Common Instructions:**
- `--add <feature>`: Add specific functionality
- `--fix <issue>`: Fix identified problems
- `--optimize`: Optimize for performance/memory
- `--secure`: Improve security
- `--refactor`: Refactor for better structure
- `--document`: Add documentation
- `--test`: Add test cases

### `/review <file> [focus]`

Comprehensive code review with suggestions.

**Usage:**
```
/review src/database.rs                # General code review
/review src/auth.rs --focus security   # Security-focused review
/review --all --brief                  # Quick review of all files
/review --staged                       # Review staged changes
```

**Review Focus Areas:**
- `security`: Security vulnerabilities and best practices
- `performance`: Performance issues and improvements
- `maintainability`: Code structure and readability
- `bugs`: Potential bugs and logical errors
- `style`: Code style and conventions
- `tests`: Test coverage and quality

### `/explain <target> [detail]`

Explain code functionality and implementation details.

**Usage:**
```
/explain src/algorithm.rs:45          # Explain specific line
/explain src/parser.rs:23-67         # Explain code block
/explain calculate_hash              # Explain function
/explain --simple src/complex.rs     # Simplified explanation
```

**Detail Levels:**
- `--simple`: High-level overview
- `--detailed`: In-depth technical explanation
- `--beginner`: Beginner-friendly explanation
- `--expert`: Expert-level technical details

### `/optimize <file> [target]`

Provide optimization suggestions for code performance.

**Usage:**
```
/optimize src/sorting.rs              # General optimization
/optimize src/api.rs --target memory # Memory optimization
/optimize --benchmark src/calc.rs    # Include benchmarks
/optimize --profile                  # Use profiling data
```

**Optimization Targets:**
- `speed`: Execution speed improvements
- `memory`: Memory usage optimization
- `size`: Binary size reduction
- `compilation`: Faster compilation
- `energy`: Power efficiency

### `/refactor <file> [type]`

Suggest and apply refactoring transformations.

**Usage:**
```
/refactor src/large_file.rs                    # General refactoring
/refactor src/utils.rs --extract-functions    # Extract functions
/refactor src/model.rs --rename-symbols       # Rename symbols
/refactor --move-code src/old.rs src/new.rs   # Move code
```

**Refactoring Types:**
- `extract-functions`: Extract reusable functions
- `extract-modules`: Split into modules
- `rename-symbols`: Improve naming
- `eliminate-duplication`: Remove code duplication
- `simplify-conditionals`: Simplify complex logic

### `/generate <type> [specifications]`

Generate various types of code constructs.

**Usage:**
```
/generate function --name hash_password --params password:String
/generate struct User --fields name,email,age --derive Clone,Debug
/generate module auth --functions login,logout,register
/generate api-endpoint /users/:id --method GET --response User
```

**Generation Types:**
- `function`: Generate function with signature
- `struct`/`class`: Generate data structures
- `module`: Generate module with functions
- `test`: Generate test cases
- `documentation`: Generate docs
- `api-endpoint`: Generate REST endpoints

---

## Testing Commands

Commands for test generation, execution, and analysis.

### `/test [target] [options]`

Run existing tests or test specific components.

**Usage:**
```
/test                        # Run all tests
/test src/auth.rs           # Test specific module
/test --unit                # Run unit tests only
/test --integration         # Run integration tests
/test --bench               # Run benchmarks
```

**Test Types:**
- `--unit`: Unit tests
- `--integration`: Integration tests
- `--doc`: Documentation tests
- `--bench`: Benchmark tests

### `/test-gen <file> [options]`

Generate comprehensive test cases for code.

**Usage:**
```
/test-gen src/calculator.rs              # Generate basic tests
/test-gen src/auth.rs --coverage 90%     # Target coverage level
/test-gen src/api.rs --edge-cases        # Include edge cases
/test-gen --all --quick                  # Quick test generation for all
```

**Options:**
- `--coverage <percent>`: Target coverage percentage
- `--edge-cases`: Include edge case testing
- `--integration`: Generate integration tests
- `--property`: Property-based testing
- `--mock`: Generate with mocking

### `/test-fix [test]`

Fix failing tests with AI assistance.

**Usage:**
```
/test-fix                    # Fix all failing tests
/test-fix test_auth_login   # Fix specific test
/test-fix --interactive     # Interactive fixing
/test-fix --explain         # Explain test failures
```

### `/coverage [options]`

Analyze and display test coverage information.

**Usage:**
```
/coverage                    # Show overall coverage
/coverage src/handlers/     # Coverage for specific directory
/coverage --missing         # Show uncovered lines
/coverage --report html     # Generate HTML report
```

### `/benchmark [target] [options]`

Run performance benchmarks and analysis.

**Usage:**
```
/benchmark                   # Run all benchmarks
/benchmark src/crypto.rs    # Benchmark specific module
/benchmark --compare main   # Compare with another branch
/benchmark --profile        # Include profiling data
```

---

## Git Commands

Enhanced Git operations with AI insights.

### `/status [options]`

Enhanced git status with AI analysis.

**Usage:**
```
/status                      # Enhanced git status
/status --analyze           # Analyze changes
/status --suggest           # Suggest next actions
/status --clean             # Clean working directory suggestions
```

### `/diff [target] [options]`

Intelligent diff analysis with explanations.

**Usage:**
```
/diff                        # Show working directory changes
/diff HEAD~1                # Compare with previous commit
/diff main..feature         # Compare branches
/diff --staged --explain    # Explain staged changes
/diff --impact              # Analyze change impact
```

### `/commit [message] [options]`

Smart commit with AI-generated messages.

**Usage:**
```
/commit                            # Interactive commit
/commit "Add user authentication"  # Manual message
/commit --generate                 # AI-generated message
/commit --conventional            # Conventional commit format
/commit --analyze-impact          # Include impact analysis
```

**Conventional Commit Types:**
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Maintenance tasks

### `/branch <name> [options]`

Enhanced branch operations.

**Usage:**
```
/branch feature/auth              # Create and switch to branch
/branch --list --analyze         # List with analysis
/branch --cleanup --merged       # Clean up merged branches
/branch --compare main feature   # Compare branches
```

### `/merge <branch> [options]`

Intelligent merge operations with conflict resolution.

**Usage:**
```
/merge feature/auth              # Merge branch
/merge --strategy recursive     # Specify merge strategy
/merge --resolve-conflicts      # Auto-resolve conflicts
/merge --preview               # Preview merge changes
```

### `/rebase [branch] [options]`

Interactive rebase with AI assistance.

**Usage:**
```
/rebase main                    # Rebase onto main
/rebase --interactive          # Interactive rebase
/rebase --resolve-conflicts    # Auto-resolve conflicts
/rebase --squash              # Squash commits
```

---

## Project Management

Commands for project-level operations and management.

### `/init [template] [options]`

Initialize new projects with templates.

**Usage:**
```
/init rust-cli                  # Initialize Rust CLI project
/init --template web-api       # Use specific template
/init --provider anthropic     # Configure AI provider
/init --git                    # Initialize Git repository
```

### `/scaffold <type> [name] [options]`

Generate project scaffolding and boilerplate.

**Usage:**
```
/scaffold module auth                    # Create auth module
/scaffold controller UserController     # Create controller
/scaffold test integration/auth_test    # Create test file
/scaffold --crud User                   # Generate CRUD operations
```

### `/todo [action] [options]`

Manage TODO items and project tasks.

**Usage:**
```
/todo                          # List all TODOs
/todo --extract               # Extract from code comments
/todo --prioritize            # AI prioritization
/todo --estimate             # Effort estimation
/todo --assign <dev>         # Assign to developer
```

### `/plan <feature> [options]`

Generate implementation plans for features.

**Usage:**
```
/plan "user authentication system"      # Create feature plan
/plan --detailed                       # Detailed implementation steps
/plan --estimate                       # Include time estimates
/plan --dependencies                   # Show dependencies
```

---

## Configuration Commands

Commands for managing PiCode configuration.

### `/config <key> [value] [options]`

View or modify configuration settings.

**Usage:**
```
/config                              # Show all configuration
/config provider                    # Show specific setting
/config provider openai            # Set configuration value
/config --reset                     # Reset to defaults
/config --export config.toml       # Export configuration
```

### `/provider <name> [options]`

Manage LLM provider settings.

**Usage:**
```
/provider                           # Show current provider
/provider openai                   # Switch to OpenAI
/provider --list                   # List available providers
/provider --test                   # Test connection
/provider --add custom-llm         # Add custom provider
```

### `/layout <name> [options]`

Manage interface layouts.

**Usage:**
```
/layout                             # Show current layout
/layout coding                     # Switch to coding layout
/layout --list                     # List available layouts
/layout --save my-layout           # Save current layout
/layout --reset                    # Reset to default
```

---

## System Commands

System-level commands and utilities.

### `/help [command] [options]`

Display help information.

**Usage:**
```
/help                          # General help
/help analyze                 # Help for specific command
/help --commands              # List all commands
/help --examples              # Show examples
```

### `/history [options]`

Show command history and statistics.

**Usage:**
```
/history                       # Show recent commands
/history --search "test"      # Search command history
/history --stats              # Usage statistics
/history --clear              # Clear history
```

### `/clear [options]`

Clear various types of data and interface.

**Usage:**
```
/clear                         # Clear conversation
/clear --cache                # Clear caches
/clear --history              # Clear command history
/clear --all                  # Clear everything
```

### `/logs [options]`

View system logs and debugging information.

**Usage:**
```
/logs                          # Show recent logs
/logs --level error           # Filter by log level
/logs --tail                  # Follow logs in real-time
/logs --export debug.log      # Export logs to file
```

### `/doctor [options]`

Run system diagnostics.

**Usage:**
```
/doctor                        # Run all diagnostics
/doctor --connection          # Test LLM connection
/doctor --performance         # Performance diagnostics
/doctor --config              # Configuration validation
```

### `/update [options]`

Update PiCode and related components.

**Usage:**
```
/update                        # Check for updates
/update --install             # Install updates
/update --plugins             # Update plugins
/update --check               # Check update availability
```

### `/exit [options]`

Exit PiCode with cleanup.

**Usage:**
```
/exit                          # Exit normally
/exit --force                 # Force exit
/exit --save-session          # Save session state
/exit --no-cleanup            # Skip cleanup
```

---

## Workflow Commands

Commands for advanced workflow management.

### `/workflow <name> [options]`

Execute predefined workflows.

**Usage:**
```
/workflow deploy               # Run deployment workflow
/workflow --list              # List available workflows
/workflow --create ci-cd      # Create new workflow
/workflow --edit deploy       # Edit existing workflow
```

### `/automate <task> [options]`

Set up automation for repetitive tasks.

**Usage:**
```
/automate testing             # Automate testing
/automate --on-save format   # Format files on save
/automate --schedule daily   # Daily automation
/automate --webhook deploy   # Webhook-triggered automation
```

### `/export <type> [options]`

Export various types of project data.

**Usage:**
```
/export documentation          # Export project docs
/export --format markdown    # Specify export format
/export --include-code       # Include code samples
/export --to-file docs.md    # Export to specific file
```

---

## Advanced Usage

### Command Chaining

Chain multiple commands together:
```
/analyze src/ && /test-gen --coverage 80% && /test
```

### Conditional Execution

Execute commands based on conditions:
```
/test || /test-fix --auto
```

### Variable Substitution

Use variables in commands:
```
/analyze ${current_file}
/commit "${commit_type}: ${description}"
```

### Aliases

Create command aliases for frequently used patterns:
```
/config alias quick-deploy "/test && /commit --generate && /push"
/config alias full-check "/analyze && /test && /coverage --missing"
```

### Batch Operations

Execute commands on multiple files:
```
/review src/*.rs               # Review all Rust files
/test-gen src/**/*.py         # Generate tests for all Python files
/optimize --all --target speed # Optimize all files for speed
```

---

This comprehensive reference covers all available slash commands in PiCode. For more detailed information about specific commands, use `/help <command>` within PiCode.