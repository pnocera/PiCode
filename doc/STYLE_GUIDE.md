# PiCode Documentation Style Guide

> 📝 **Consistent Documentation Standards** - Style guidelines for all PiCode documentation

## Overview

This style guide ensures consistency, readability, and maintainability across all PiCode documentation. Following these guidelines helps create a cohesive experience for users, developers, and AI systems.

## Markdown Standards

### Header Hierarchy

```markdown
# Document Title (H1)
## Major Section (H2)
### Subsection (H3)
#### Detail Section (H4)
##### Minor Detail (H5)
###### Rarely Used (H6)
```

**Rules:**
- Use only one H1 per document (document title)
- Maintain logical hierarchy (don't skip levels)
- Use sentence case for headers
- Include meaningful anchor-friendly text

### Cross-Reference Links

#### Internal Links (within PiCode docs)
```markdown
<!-- Relative paths preferred -->
[CLI Reference](../reference/cli.md)
[Installation Guide](../quickstart/installation.md)

<!-- With anchors -->
[Configuration Section](config.md#llm-providers)
[Architecture Overview](../developer/architecture/core.md#plugin-system)
```

#### External Links
```markdown
<!-- Descriptive link text -->
[Rust Programming Language](https://www.rust-lang.org/)
[Zellij Terminal Multiplexer](https://zellij.dev/)

<!-- With access date for dynamic content -->
[OpenAPI Specification](https://spec.openapis.org/oas/v3.1.0/) (accessed 2024-01-15)
```

### Code Blocks

#### Language Specification
```markdown
<!-- Always specify language -->
```rust
fn main() {
    println!("Hello, PiCode!");
}
```

<!-- Command line examples -->
```bash
picode --help
picode interactive --provider openai
```

<!-- Configuration files -->
```toml
[providers.openai]
api_key = "${OPENAI_API_KEY}"
model = "gpt-4"
```
```

#### Inline Code
```markdown
Use `backticks` for inline code, commands like `cargo build`, and file names like `Cargo.toml`.
```

### Lists and Structure

#### Ordered Lists
```markdown
1. **Bold for emphasis** - Description of step
2. **Next step** - Detailed explanation
   - Sub-item with additional context
   - Another sub-item
3. **Final step** - Conclusion
```

#### Unordered Lists
```markdown
- **Feature Name**: Brief description
- **Another Feature**: Explanation with [link](reference.md)
- **Complex Feature**:
  - Sub-feature details
  - Implementation notes
  - Usage examples
```

### Tables

```markdown
| Feature | Status | Description |
|---------|--------|-------------|
| CLI Interface | ✅ Complete | Full command-line interface |
| Interactive Mode | 🔄 In Progress | Conversational development |
| Plugin System | ⏳ Planned | Extension architecture |
```

**Table Guidelines:**
- Use emoji for status indicators
- Keep column headers concise
- Align content logically
- Include units for numeric data

### Callouts and Alerts

```markdown
> 🚀 **Quick Start**: For immediate setup, see [Installation](quickstart/installation.md)

> ⚠️ **Warning**: This feature requires Rust 1.70 or later

> 💡 **Tip**: Use `--verbose` flag for detailed debugging output

> 🐛 **Bug**: Known issue with WASM compilation on Windows ([#123](issues/123))

> 📚 **See Also**: Related documentation in [Developer Guide](developer/index.md)
```

## Content Organization

### Document Structure

#### Standard Document Template
```markdown
# Document Title

> 🎯 **Brief Description** - One-line summary of document purpose

## Overview
Brief introduction to the topic

## Quick Navigation
- [Section 1](#section-1)
- [Section 2](#section-2)

## Main Content
...

## Examples
Practical usage examples

## Troubleshooting
Common issues and solutions

## See Also
- [Related Doc 1](link1.md)
- [Related Doc 2](link2.md)

---
*Last updated: YYYY-MM-DD*
```

### Navigation Standards

#### Breadcrumb Navigation
```markdown
[Home](../index.md) > [User Guide](index.md) > Configuration
```

#### Index Page Structure
```markdown
# Category Index

Brief category description

## Quick Navigation
### 🚀 [Subcategory 1](subcategory1/index.md)
Brief description
- [Item 1](subcategory1/item1.md)
- [Item 2](subcategory1/item2.md)

### 📖 [Subcategory 2](subcategory2/index.md)
Brief description
- [Item A](subcategory2/itemA.md)
- [Item B](subcategory2/itemB.md)
```

## Content Guidelines

### Writing Style

#### Voice and Tone
- **Active voice preferred**: "PiCode provides..." not "Features are provided..."
- **Present tense**: "The system handles..." not "The system will handle..."
- **Direct and concise**: Avoid unnecessary words
- **Professional but approachable**: Technical accuracy with clear explanations

#### Technical Terms
- **Define on first use**: "LLM (Large Language Model)"
- **Consistent terminology**: Use project glossary
- **Acronym expansion**: Always expand acronyms initially

#### Examples and Code
- **Complete examples**: Runnable code when possible
- **Realistic scenarios**: Use practical, relevant examples
- **Error handling**: Show both success and failure cases
- **Platform awareness**: Note OS-specific requirements

### Audience-Specific Guidelines

#### User Documentation
- **Task-oriented**: Focus on "how to accomplish X"
- **Step-by-step instructions**: Clear, numbered procedures
- **Prerequisites clearly stated**: What users need before starting
- **Expected outcomes**: What success looks like

#### Developer Documentation
- **API-focused**: Interface definitions and usage
- **Architecture context**: How components fit together
- **Implementation details**: Technical depth appropriate for contributors
- **Testing guidance**: How to validate changes

#### AI Documentation
- **Structured data**: Use YAML frontmatter for metadata
- **Machine-readable**: Consistent formatting for parsing
- **Comprehensive context**: Include constraints and capabilities
- **Version-aware**: Track changes and compatibility

## Validation and Quality

### Automated Checks
- **Link validation**: All internal and external links verified
- **Spell check**: Automated grammar and spelling validation
- **Code block testing**: Ensure code examples are valid
- **Cross-reference verification**: Confirm all references exist

### Manual Review
- **Technical accuracy**: Subject matter expert review
- **Clarity assessment**: Readability and comprehension
- **Completeness check**: All necessary information included
- **Consistency audit**: Style guide compliance

### Maintenance Process

#### Regular Updates
- **Quarterly review**: Full documentation audit
- **Feature updates**: Documentation updated with new features
- **Link maintenance**: Verify external links remain valid
- **User feedback**: Incorporate reader suggestions

#### Version Control
- **Semantic versioning**: Major.Minor.Patch for doc versions
- **Change logs**: Track significant documentation changes
- **Branch strategy**: Feature branches for major updates
- **Review process**: Pull request review for all changes

## File Organization

### Naming Conventions
- **File names**: `kebab-case.md` (lowercase with hyphens)
- **Directory names**: `lowercase` or `kebab-case`
- **Image files**: `descriptive-name.png/svg`
- **Configuration**: `UPPERCASE.toml/yaml/json`

### Directory Structure
```
category/
├── index.md           # Category overview
├── getting-started.md # Basic introduction
├── advanced/          # Complex topics
│   ├── index.md
│   └── topic.md
└── examples/          # Practical examples
    ├── index.md
    └── example-name.md
```

## Integration with Development

### Source Code Integration
- **Rust doc comments**: Extract API docs from code
- **Configuration examples**: Sync with actual config files
- **CLI help**: Generate reference from command definitions
- **Error messages**: Document error codes and solutions

### Build Process
- **Automated generation**: Update docs from source changes
- **Validation pipeline**: Ensure docs build without errors
- **Deployment**: Automated publication of documentation
- **Feedback loop**: Easy reporting of documentation issues

---

*This style guide is a living document. Suggest improvements via [GitHub Issues](https://github.com/pnocera/PiCode/issues) or [Pull Requests](../developer/contributing/documentation.md).*