# Project: PiCode - Replicating Claude Code Features with Rust and OpenAPI

## Project Overview:

PiCode aims to develop a new application in Rust that replicates a core subset of Claude Code's functionalities, with the key distinction of being compatible with any OpenAPI-enabled Large Language Model (LLM) provider. This project will leverage the existing Zellij terminal multiplexer as a foundational component for building a robust and interactive command-line interface (CLI). The application will also be compiled to WebAssembly (Wasm) to enable its use as an MCP (Multi-Agent Collaboration Protocol) server tool for other LLMs or for direct execution in terminals and browsers.

**Project Repository:** https://github.com/pnocera/PiCode

**Root Project Path:** `/home/pierre/Apps/pi-code/`

## Core Features to Implement (Inspired by Claude Code):

The following features, as defined in the Claude Code documentation (https://docs.anthropic.com/en/docs/claude-code/overview), are to be implemented in PiCode:

*   **CLI Reference:** Implement a comprehensive and user-friendly command-line interface, mirroring the functionality and user experience described in the Claude Code CLI Reference (https://docs.anthropic.com/en/docs/claude-code/cli-reference). This includes command structure, arguments, options, and help messages. Claude Code is known for working directly in the terminal and taking action by editing files, running commands, and creating commits. It also integrates with tools like GitHub and GitLab.
*   **Interactive Mode:** Develop an interactive mode that allows users to engage with the LLM in a conversational manner for code-related tasks, similar to Claude Code's Interactive Mode (https://docs.anthropic.com/en/docs/claude-code/interactive-mode). This mode should facilitate tasks such as code editing, refactoring, bug fixing, code understanding, automated testing, and Git integration. Claude Code excels at understanding entire codebases and project structures.
*   **Slash Commands:** Implement support for "slash commands" as described in the Claude Code Slash Commands documentation (https://docs.anthropic.com/en/docs/claude-code/slash-commands). These commands provide a quick and efficient way to trigger specific actions or functionalities within the interactive mode.
*   **Hooks Reference:** Integrate a robust system for "hooks," allowing for custom actions or scripts to be triggered at various points within the application's workflow, as detailed in the Claude Code Hooks Reference (https://docs.anthropic.com/en/docs/claude-code/hooks).
*   **Configurable LLM Options (OpenAPI Compatibility):** The application must be highly configurable to work with any OpenAPI-compatible LLM provider. This involves:
    *   Dynamic configuration of LLM API endpoints, authentication mechanisms, and model selection.
    *   Support for OpenAPI 3.0.x and 3.1.x specifications, handling complex request bodies and parameter types.
    *   The ability to convert OpenAPI specifications into LLM-compatible tool/function definitions.
    *   Consider popular OpenAI-compatible LLM providers such as OpenAI, Anthropic, Cohere, Google AI Studio, HuggingFace Inference Providers, and others.

## Technical Stack:

*   **Rust:** The primary programming language for the entire application.
*   **Zellij:** Utilize Zellij (source code available at `/home/pierre/Apps/pi-code/0-github-repos/zellij`) as the foundational framework for building the CLI. Zellij is a terminal multiplexer written in Rust, known for its user-friendly UI, layout system, and plugin support via WebAssembly. It offers features like detachable/persistent sessions and multi-language plugins.
*   **WebAssembly (Wasm):** The CLI will be compiled to Wasm to enable its functionality as an MCP server for other LLMs to use as tools, or to be run directly in a terminal or browser. Rust has strong support for compiling to WebAssembly, with tools like `wasm-bindgen` and `wasm-pack` facilitating high-level interactions between Wasm modules and JavaScript. Wasmtime CLI also supports invoking Wasm component exports directly from the command line.

## Project Plan:

The project will be executed in a phased approach, with human review checkpoints to ensure quality and alignment. Each phase will utilize a swarm of 5 AI agents working in parallel to maximize efficiency.

### Phase 1: Research (5 AI Agents in parallel)

**Objective:** Conduct a deep analysis of Claude Code's features and thoroughly understand Zellij's architecture and capabilities.

**Tasks:**

1.  **Claude Code Feature Analysis:**
    *   Thoroughly read and analyze the official Claude Code documentation, specifically focusing on the provided links:
        *   https://docs.anthropic.com/en/docs/claude-code/overview
        *   https://docs.anthropic.com/en/docs/claude-code/cli-reference
        *   https://docs.anthropic.com/en/docs/claude-code/interactive-mode
        *   https://docs.anthropic.com/en/docs/claude-code/slash-commands
        *   https://docs.anthropic.com/en/docs/claude-code/hooks
    *   Identify and document all explicit and implicit features, functionalities, and user interactions.
    *   Pay close attention to how Claude Code handles code editing, refactoring, debugging, testing, and Git integration.
    *   Investigate Claude Code's "agentic search" capabilities for understanding project structure and dependencies.
    *   Research how Claude Code interacts with the terminal and its permission model for file modifications and shell commands.
    *   Utilize web search tools, context7 MCP tools, and GitHub search to gather additional insights, examples, and community discussions related to Claude Code's implementation and usage.
2.  **Zellij Architecture and Usage Analysis:**
    *   Inspect the Zellij documentation located at `/home/pierre/Apps/pi-code/0-github-repos/zellij/docs`.
    *   Examine the Zellij examples in `/home/pierre/Apps/pi-code/0-github-repos/zellij/examples` to understand its API, plugin system, and how it can be extended for CLI development.
    *   Focus on Zellij's capabilities for managing panes, tabs, layouts, and its WebAssembly plugin support.
    *   Research how Zellij handles user input, rendering, and interaction within the terminal environment.
    *   Investigate how Zellij's design principles (e.g., Unix philosophy, composability) can be applied to PiCode.
3.  **OpenAPI and LLM Integration Research:**
    *   Research best practices and existing Rust crates for interacting with OpenAPI-compatible LLM providers.
    *   Investigate how to dynamically configure LLM endpoints, API keys, and model names.
    *   Explore methods for converting OpenAPI specifications into LLM-compatible function definitions.
    *   Identify potential challenges and solutions for handling diverse LLM responses and integrating them into a CLI application.
    *   Research how Rust's WASM capabilities can be leveraged to create an MCP server for LLMs, allowing them to use PiCode as a tool.

**Output:** Comprehensive research results in Markdown format, organized within the `/home/pierre/Apps/pi-code/2-analysis` folder. Each agent should contribute to a shared, well-structured document, with clear headings and citations for all sources.

### Phase 2: Human Review

**Objective:** Allow a human expert to review and validate the research findings from Phase 1, providing feedback and guidance before proceeding.

**Action:** Human review of the `/home/pierre/Apps/pi-code/2-analysis` folder.

### Phase 3: Conception - No Coding Yet (5 AI Agents in parallel)

**Objective:** Design the software architecture and detailed component specifications for PiCode, based on the research from Phase 1.

**Tasks:**

1.  **High-Level Software Architecture Proposal:**
    *   Propose a modular and scalable architecture for PiCode, outlining the main components and their interactions.
    *   Clearly define the separation of concerns between the CLI, LLM integration, and core logic.
    *   Illustrate the architecture with diagrams (e.g., block diagrams, component diagrams) as appropriate.
2.  **Detailed Component Conception:**
    *   For each identified component, provide a detailed conception, including:
        *   **Purpose and Responsibilities:** What does this component do?
        *   **Input/Output:** What data does it receive and produce?
        *   **Key Data Structures:** Define the primary data structures used within the component.
        *   **Core Logic/Algorithms:** Outline the main logic flows.
        *   **API/Interface Definitions:** Specify how other components will interact with it (e.g., Rust traits, function signatures).
        *   **Error Handling Strategy:** How will errors be managed?
        *   **Integration Points:** How does it connect with Zellij and the OpenAPI LLM providers?
        *   **Wasm Compilation Considerations:** How will this component contribute to the overall WASM compilation and MCP server functionality?
3.  **Zellij Integration Design:**
    *   Detail how Zellij will be used as the foundation for the CLI. This includes:
        *   Designing the interactive mode's layout and user experience within Zellij panes and tabs.
        *   Planning how Zellij's plugin system will be leveraged for extending PiCode's functionality.
        *   Specifying how PiCode will interact with Zellij's underlying terminal control.
4.  **OpenAPI LLM Integration Design:**
    *   Design the LLM integration layer, focusing on:
        *   How OpenAPI specifications will be parsed and used to generate LLM tool definitions.
        *   The mechanism for dynamically configuring and switching between different OpenAPI-compatible LLM providers.
        *   Strategies for handling LLM responses, including parsing, validation, and error handling.
        *   Consideration for various authentication methods (e.g., API keys, OAuth).
5.  **Wasm Architecture Design:**
    *   Outline the architecture for compiling PiCode to WebAssembly.
    *   Specify how the Wasm module will expose its functionalities for use as an MCP tool by other LLMs.
    *   Detail how the Wasm version will handle interactive mode in a browser or terminal environment.

**Output:** Detailed conception documents in Markdown format, organized within the `/home/pierre/Apps/pi-code/2-analysis` folder. Diagrams should be included as image files if necessary, referenced within the Markdown.

### Phase 4: Human Review

**Objective:** Allow a human expert to review and validate the architectural and component designs from Phase 3, providing feedback and ensuring feasibility before implementation.

**Action:** Human review of the `/home/pierre/Apps/pi-code/2-analysis` folder.

### Phase 5: Implementation (5 AI Agents in parallel)

**Objective:** Develop the full implementation of PiCode in Rust, following a Test-Driven Development (TDD) approach.

**Tasks:**

1.  **TDD Cycle Implementation:**
    *   For each component and feature identified in Phase 3, follow the TDD cycle:
        *   Write a test function with the `#[test]` attribute.
        *   Run tests and observe failure.
        *   Write the minimum amount of Rust code necessary to make the test pass.
        *   Refactor the code as needed, ensuring maintainability and adherence to Rust best practices (e.g., ownership, lifetimes, traits).
        *   Repeat the cycle until all tests pass and the feature is fully implemented.
    *   Utilize `cargo test` for running unit and integration tests.
2.  **CLI Implementation:**
    *   Implement the CLI commands and subcommands as designed in Phase 3, leveraging Zellij's capabilities.
    *   Ensure robust argument parsing, option handling, and clear help messages.
3.  **Interactive Mode Implementation:**
    *   Develop the interactive mode, handling user input, displaying LLM responses, and managing the conversational flow.
    *   Integrate slash command recognition and execution.
4.  **Hooks Implementation:**
    *   Implement the hooks system, allowing for the registration and execution of custom scripts or functions at predefined points.
5.  **OpenAPI LLM Integration Implementation:**
    *   Implement the logic for connecting to and interacting with OpenAPI-compatible LLM providers.
    *   Develop the functionality for converting OpenAPI specs to LLM tool definitions and invoking LLM functions.
    *   Handle API key management and other authentication details securely.
6.  **Wasm Compilation and MCP Server Development:**
    *   Configure the Rust project for WebAssembly compilation.
    *   Develop the necessary `wasm-bindgen` interfaces to expose PiCode's functionalities as an MCP server for other LLMs.
    *   Ensure the Wasm output can be run in a terminal (e.g., via `wasmtime`) or a browser.

**Output:** Fully implemented and tested Rust codebase within the `https://github.com/pnocera/PiCode` repository. All tests (`cargo test`) must pass.

### Phase 6: Documentation (5 AI Agents in parallel)

**Objective:** Create comprehensive human-facing and AI-reference documentation for PiCode.

**Tasks:**

1.  **Human-Facing Documentation (Quickstart):**
    *   Create a `Quickstart.md` in the `/home/pierre/Apps/pi-code/userfacing` folder.
    *   This documentation should be concise, easy to understand for new users, and cover:
        *   Installation instructions.
        *   Basic usage examples for CLI and interactive mode.
        *   How to configure an LLM provider.
        *   A simple "hello world" example demonstrating core functionality.
2.  **Deep, Referenced Documentation (AI Reference):**
    *   Create detailed documentation in the `/home/pierre/Apps/pi-code/aireference` folder.
    *   This documentation should be comprehensive and include:
        *   **Architecture Overview:** A detailed explanation of PiCode's software architecture, referencing the design documents from Phase 3.
        *   **Component-Level Documentation:** In-depth descriptions of each component, their APIs, and internal workings.
        *   **CLI Reference:** A complete reference for all CLI commands, arguments, and options, with examples.
        *   **Interactive Mode Guide:** A detailed guide on using the interactive mode, including all available slash commands and their usage.
        *   **Hooks Reference:** Comprehensive documentation of the hooks system, including how to define, register, and use custom hooks.
        *   **LLM Integration Guide:** Detailed instructions on configuring and integrating with various OpenAPI-compatible LLM providers.
        *   **Wasm Deployment Guide:** Instructions for compiling PiCode to WebAssembly and deploying it as an MCP server or for browser/terminal execution.
        *   **Development Guide:** Information for developers who want to contribute to PiCode, including setup, testing, and coding conventions.
        *   **Troubleshooting Guide:** Common issues and their solutions.
        *   **Glossary:** Definitions of key terms.
        *   **Citations and References:** All external resources, including Claude Code documentation, Zellij documentation, and relevant Rust/Wasm/OpenAPI resources, should be properly cited.

**Output:** Well-structured Markdown documentation files in the specified folders, ready for public consumption and internal reference.