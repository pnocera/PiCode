# PiCode Documentation Navigation Guide

> 🧭 **Find What You Need** - Complete navigation reference for PiCode documentation

## Quick Access Paths

### New Users
1. [Installation](user/quickstart/installation.md) → Get PiCode running
2. [First Steps](user/quickstart/first-steps.md) → Basic usage
3. [Interactive Mode](user/guides/interactive-mode.md) → Core functionality
4. [LLM Setup](user/guides/llm-setup.md) → Connect your LLM provider

### Developers
1. [Architecture Overview](developer/architecture/core.md) → System design
2. [Development Setup](developer/contributing/setup.md) → Get contributing
3. [API Reference](developer/api/core.md) → Interface documentation
4. [Testing Guide](developer/contributing/testing.md) → Quality assurance

### AI Systems
1. [System Overview](ai/context/system-overview.md) → Core capabilities
2. [API Specification](ai/specifications/api-spec.yaml) → Machine-readable interface
3. [Integration Patterns](ai/analysis/integration-patterns.md) → Common approaches
4. [Prompt Templates](ai/prompts/system-prompts.md) → AI interaction patterns

## Complete Documentation Map

```
doc/
├── README.md                    # 📋 Documentation architecture overview
├── STYLE_GUIDE.md              # 📝 Writing and formatting standards
├── NAVIGATION.md               # 🧭 This navigation guide
│
├── user/                       # 👤 User-focused documentation
│   ├── index.md               # User documentation overview
│   ├── quickstart/
│   │   ├── index.md           # Getting started overview
│   │   ├── installation.md    # Installation instructions
│   │   ├── first-steps.md     # Basic usage tutorial
│   │   └── configuration.md   # Initial setup guide
│   ├── guides/
│   │   ├── index.md           # User guides overview
│   │   ├── interactive-mode.md # Interactive development
│   │   ├── llm-setup.md       # LLM provider configuration
│   │   ├── project-setup.md   # Project integration
│   │   ├── slash-commands.md  # Command reference
│   │   └── workflows.md       # Common usage patterns
│   ├── reference/
│   │   ├── index.md           # Reference documentation
│   │   ├── cli.md             # Command-line interface
│   │   ├── config.md          # Configuration options
│   │   ├── hooks.md           # Hook system reference
│   │   └── environment.md     # Environment variables
│   └── examples/
│       ├── index.md           # Examples overview
│       ├── workflows.md       # Common workflows
│       ├── configs.md         # Configuration examples
│       └── integrations.md    # Integration examples
│
├── developer/                 # 🛠️ Developer documentation
│   ├── index.md              # Developer overview
│   ├── architecture/
│   │   ├── index.md          # Architecture overview
│   │   ├── core.md           # Core system design
│   │   ├── plugins.md        # Plugin architecture
│   │   ├── llm-integration.md # LLM provider integration
│   │   └── security.md       # Security model
│   ├── components/
│   │   ├── index.md          # Components overview
│   │   ├── cli.md            # CLI module
│   │   ├── core.md           # Core engine
│   │   ├── providers.md      # Provider registry
│   │   └── hooks.md          # Hook registry
│   ├── api/
│   │   ├── index.md          # API documentation
│   │   ├── core.md           # Core API
│   │   ├── plugins.md        # Plugin API
│   │   ├── providers.md      # Provider interface
│   │   └── hooks.md          # Hook interface
│   └── contributing/
│       ├── index.md          # Contribution overview
│       ├── setup.md          # Development setup
│       ├── standards.md      # Code standards
│       ├── testing.md        # Testing guidelines
│       └── documentation.md  # Documentation contribution
│
└── ai/                       # 🤖 AI-focused documentation
    ├── index.md              # AI documentation overview
    ├── context/
    │   ├── index.md          # System context
    │   ├── system-overview.md # High-level capabilities
    │   ├── capabilities.md   # Feature matrix
    │   ├── limitations.md    # Known constraints
    │   └── use-cases.md      # Common patterns
    ├── specifications/
    │   ├── index.md          # Specifications overview
    │   ├── api-spec.yaml     # OpenAPI specification
    │   ├── plugin-spec.yaml  # Plugin interface spec
    │   ├── provider-spec.yaml # Provider interface spec
    │   └── data-formats.yaml # Data structure definitions
    ├── analysis/
    │   ├── index.md          # Analysis overview
    │   ├── architecture-analysis.md # System analysis
    │   ├── performance-metrics.md   # Performance data
    │   ├── security-analysis.md     # Security model
    │   └── integration-patterns.md  # Integration approaches
    └── prompts/
        ├── index.md          # Prompt engineering
        ├── system-prompts.md # Core system prompts
        ├── task-templates.md # Task-specific templates
        ├── context-examples.md # Context formatting
        └── response-formats.md # Response structures
```

## Cross-Reference Matrix

### By User Journey

| Journey Stage | User Docs | Developer Docs | AI Docs |
|---------------|-----------|----------------|---------|
| **Discovery** | [Overview](user/index.md) | [Architecture](developer/architecture/core.md) | [System Overview](ai/context/system-overview.md) |
| **Setup** | [Installation](user/quickstart/installation.md) | [Dev Setup](developer/contributing/setup.md) | [API Spec](ai/specifications/api-spec.yaml) |
| **First Use** | [First Steps](user/quickstart/first-steps.md) | [Components](developer/components/index.md) | [Use Cases](ai/context/use-cases.md) |
| **Integration** | [Project Setup](user/guides/project-setup.md) | [API Reference](developer/api/core.md) | [Integration Patterns](ai/analysis/integration-patterns.md) |
| **Mastery** | [Workflows](user/examples/workflows.md) | [Contributing](developer/contributing/index.md) | [Advanced Prompts](ai/prompts/task-templates.md) |

### By Feature Area

| Feature | User Guide | Developer Reference | AI Context |
|---------|------------|-------------------|------------|
| **CLI** | [CLI Usage](user/reference/cli.md) | [CLI Module](developer/components/cli.md) | [CLI Capabilities](ai/context/capabilities.md#cli) |
| **Interactive Mode** | [Interactive Guide](user/guides/interactive-mode.md) | [Core Engine](developer/components/core.md) | [Interactive Patterns](ai/prompts/context-examples.md) |
| **LLM Integration** | [LLM Setup](user/guides/llm-setup.md) | [LLM Architecture](developer/architecture/llm-integration.md) | [Provider Spec](ai/specifications/provider-spec.yaml) |
| **Plugins** | [Plugin Usage](user/guides/workflows.md#plugins) | [Plugin API](developer/api/plugins.md) | [Plugin Spec](ai/specifications/plugin-spec.yaml) |
| **Hooks** | [Hooks Reference](user/reference/hooks.md) | [Hook Registry](developer/components/hooks.md) | [Hook Capabilities](ai/context/capabilities.md#hooks) |

### By Technical Topic

| Topic | User Level | Developer Level | AI Level |
|-------|------------|-----------------|----------|
| **Configuration** | [Config Guide](user/quickstart/configuration.md) | [Architecture](developer/architecture/core.md#configuration) | [Data Formats](ai/specifications/data-formats.yaml) |
| **Security** | [Environment Variables](user/reference/environment.md) | [Security Model](developer/architecture/security.md) | [Security Analysis](ai/analysis/security-analysis.md) |
| **Performance** | [Troubleshooting](user/guides/workflows.md#troubleshooting) | [Components](developer/components/index.md#performance) | [Performance Metrics](ai/analysis/performance-metrics.md) |
| **Extensibility** | [Examples](user/examples/integrations.md) | [Plugin System](developer/architecture/plugins.md) | [Integration Patterns](ai/analysis/integration-patterns.md) |

## Search and Discovery

### Search Strategies

#### By Keyword
- **"install"** → [Installation Guide](user/quickstart/installation.md)
- **"config"** → [Configuration](user/quickstart/configuration.md), [Config Reference](user/reference/config.md)
- **"api"** → [API Documentation](developer/api/index.md), [API Spec](ai/specifications/api-spec.yaml)
- **"plugin"** → [Plugin Architecture](developer/architecture/plugins.md), [Plugin API](developer/api/plugins.md)
- **"error"** → [Troubleshooting](user/guides/workflows.md#troubleshooting), [Error Handling](developer/architecture/core.md#error-handling)

#### By Question Type
- **"How do I...?"** → [User Guides](user/guides/index.md)
- **"What is...?"** → [System Overview](ai/context/system-overview.md), [Architecture](developer/architecture/core.md)
- **"Why does...?"** → [Analysis Documents](ai/analysis/index.md)
- **"When should I...?"** → [Use Cases](ai/context/use-cases.md), [Examples](user/examples/index.md)

#### By Role
- **"As a new user..."** → [Quickstart](user/quickstart/index.md)
- **"As a developer..."** → [Contributing](developer/contributing/index.md)
- **"As an integrator..."** → [API Reference](developer/api/index.md)
- **"As an AI system..."** → [AI Documentation](ai/index.md)

## Navigation Utilities

### Quick Links
- 🏠 [Documentation Home](README.md)
- 🚀 [Quick Start](user/quickstart/installation.md)
- 📖 [User Guide](user/index.md)
- 🛠️ [Developer Docs](developer/index.md)
- 🤖 [AI Reference](ai/index.md)

### External Links
- 📦 [PiCode Repository](https://github.com/pnocera/PiCode)
- 🐛 [Issue Tracker](https://github.com/pnocera/PiCode/issues)
- 💬 [Discussions](https://github.com/pnocera/PiCode/discussions)
- 📧 [Support](mailto:support@picode.dev)

### Documentation Tools
- 📝 [Style Guide](STYLE_GUIDE.md)
- 🔍 [Site Search](https://picode.dev/docs/search)
- 📊 [Documentation Stats](https://picode.dev/docs/stats)
- 🔄 [Update Notifications](https://picode.dev/docs/updates)

---

*Can't find what you're looking for? [Open an issue](https://github.com/pnocera/PiCode/issues/new?template=documentation.md) or [join the discussion](https://github.com/pnocera/PiCode/discussions/categories/documentation).*