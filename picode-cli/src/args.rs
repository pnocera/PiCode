//! Command-line argument parsing for PiCode CLI
//!
//! This module provides the argument parsing functionality using clap,
//! defining all CLI commands, subcommands, and options available in PiCode.

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// PiCode CLI - A terminal workspace with AI capabilities
#[derive(Parser, Debug, Clone)]
#[command(
    name = "picode",
    version = env!("CARGO_PKG_VERSION"),
    about = "A terminal workspace with AI capabilities - Claude Code compatible with OpenAPI LLMs",
    long_about = "PiCode provides an intelligent terminal workspace that integrates with various LLM providers through OpenAPI specifications. It offers advanced code assistance, project management, and AI-powered development tools."
)]
pub struct Args {
    /// Global verbosity level
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Working directory
    #[arg(short = 'C', long, global = true)]
    pub directory: Option<PathBuf>,

    /// Enable debug mode
    #[arg(long, global = true)]
    pub debug: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// The subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}

/// Available PiCode commands
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Initialize a new PiCode workspace
    Init {
        /// Target directory for the workspace
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Workspace name
        #[arg(short, long)]
        name: Option<String>,

        /// Initialize with a specific template
        #[arg(short, long)]
        template: Option<String>,

        /// Force initialization even if directory is not empty
        #[arg(short, long)]
        force: bool,
    },

    /// Start the interactive terminal workspace
    Workspace {
        /// Enable AI assistance mode
        #[arg(short, long)]
        ai: bool,

        /// LLM provider to use
        #[arg(short, long)]
        provider: Option<LlmProvider>,

        /// API endpoint URL (for custom providers)
        #[arg(short, long)]
        endpoint: Option<String>,

        /// Session name for workspace isolation
        #[arg(short, long)]
        session: Option<String>,
    },

    /// Execute a command with AI assistance
    Execute {
        /// The command to execute
        command: String,

        /// Additional arguments to pass to the command
        args: Vec<String>,

        /// Enable AI-powered command suggestions
        #[arg(short, long)]
        suggest: bool,

        /// Dry run - show what would be executed without running it
        #[arg(short = 'n', long)]
        dry_run: bool,
    },

    /// Manage project configurations and settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Git integration commands
    Git {
        #[command(subcommand)]
        action: GitAction,
    },

    /// LLM provider management
    Llm {
        #[command(subcommand)]
        action: LlmAction,
    },

    /// Plugin and extension management
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },

    /// Development and debugging utilities
    Dev {
        #[command(subcommand)]
        action: DevAction,
    },
}

/// Configuration management subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Remove a configuration value
    Remove {
        /// Configuration key
        key: String,
    },
    /// Reset configuration to defaults
    Reset {
        /// Confirm the reset operation
        #[arg(short, long)]
        confirm: bool,
    },
}

/// Git integration subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum GitAction {
    /// Initialize git repository with PiCode integration
    Init {
        /// Repository path
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Smart commit with AI-generated messages
    Commit {
        /// Commit message (will be AI-enhanced if not provided)
        #[arg(short, long)]
        message: Option<String>,
        /// Skip AI enhancement
        #[arg(long)]
        no_ai: bool,
        /// Include all changes
        #[arg(short, long)]
        all: bool,
    },
    /// Analyze repository health and suggest improvements
    Analyze {
        /// Generate detailed report
        #[arg(short, long)]
        detailed: bool,
        /// Focus on specific aspect
        #[arg(short, long)]
        focus: Option<GitFocus>,
    },
}

/// Git analysis focus areas
#[derive(ValueEnum, Debug, Clone)]
pub enum GitFocus {
    Security,
    Performance,
    Quality,
    Dependencies,
}

/// LLM provider management subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum LlmAction {
    /// List available LLM providers
    List,
    /// Add a new LLM provider configuration
    Add {
        /// Provider name
        name: String,
        /// Provider type
        #[arg(value_enum)]
        provider_type: LlmProvider,
        /// API endpoint URL
        endpoint: String,
        /// API key (will prompt if not provided)
        #[arg(short, long)]
        api_key: Option<String>,
    },
    /// Remove an LLM provider configuration
    Remove {
        /// Provider name
        name: String,
    },
    /// Test LLM provider connection
    Test {
        /// Provider name
        name: String,
        /// Test prompt
        #[arg(short, long, default_value = "Hello, are you working?")]
        prompt: String,
    },
    /// Set default LLM provider
    SetDefault {
        /// Provider name
        name: String,
    },
}

/// Supported LLM providers
#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
pub enum LlmProvider {
    /// Anthropic Claude
    Anthropic,
    /// OpenAI GPT models
    OpenAI,
    /// Google Gemini
    Google,
    /// Mistral AI
    Mistral,
    /// Local Ollama instance
    Ollama,
    /// Custom OpenAPI-compatible provider
    Custom,
}

/// Plugin management subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum PluginAction {
    /// List installed plugins
    List,
    /// Install a plugin
    Install {
        /// Plugin name or URL
        plugin: String,
        /// Install from local path
        #[arg(short, long)]
        local: bool,
        /// Force reinstallation
        #[arg(short, long)]
        force: bool,
    },
    /// Remove a plugin
    Remove {
        /// Plugin name
        plugin: String,
    },
    /// Update plugins
    Update {
        /// Update specific plugin (updates all if not specified)
        plugin: Option<String>,
    },
    /// Enable a plugin
    Enable {
        /// Plugin name
        plugin: String,
    },
    /// Disable a plugin
    Disable {
        /// Plugin name
        plugin: String,
    },
}

/// Development utilities subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum DevAction {
    /// Generate shell completions
    Completions {
        /// Shell type
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Show system information
    SystemInfo,
    /// Check for updates
    Update {
        /// Check for pre-release versions
        #[arg(long)]
        pre_release: bool,
        /// Automatically install updates
        #[arg(short, long)]
        auto: bool,
    },
    /// Export configuration
    Export {
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Export format
        #[arg(short, long, value_enum, default_value = "yaml")]
        format: ExportFormat,
    },
    /// Import configuration
    Import {
        /// Input file path
        input: PathBuf,
        /// Merge with existing configuration
        #[arg(short, long)]
        merge: bool,
    },
}

/// Shell types for completion generation
#[derive(ValueEnum, Debug, Clone)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

/// Export formats for configuration
#[derive(ValueEnum, Debug, Clone)]
pub enum ExportFormat {
    Yaml,
    Json,
    Toml,
}

/// Parse command-line arguments
pub fn parse() -> Args {
    Args::parse()
}

/// Parse command-line arguments from iterator
pub fn parse_from<I, T>(args: I) -> Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Args::parse_from(args)
}

/// Try to parse command-line arguments, returning a Result
pub fn try_parse() -> Result<Args, clap::Error> {
    Args::try_parse()
}

/// Try to parse command-line arguments from iterator
pub fn try_parse_from<I, T>(args: I) -> Result<Args, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Args::try_parse_from(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let args = Args::try_parse_from(["picode", "init", "my-project"]).unwrap();
        
        match args.command {
            Commands::Init { path, name, .. } => {
                assert_eq!(path, PathBuf::from("my-project"));
                assert_eq!(name, None);
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn test_workspace_command() {
        let args = Args::try_parse_from(["picode", "workspace", "--ai", "--provider", "anthropic"]).unwrap();
        
        match args.command {
            Commands::Workspace { ai, provider, .. } => {
                assert!(ai);
                assert_eq!(provider, Some(LlmProvider::Anthropic));
            }
            _ => panic!("Expected Workspace command"),
        }
    }

    #[test]
    fn test_config_set() {
        let args = Args::try_parse_from(["picode", "config", "set", "key", "value"]).unwrap();
        
        match args.command {
            Commands::Config { action: ConfigAction::Set { key, value } } => {
                assert_eq!(key, "key");
                assert_eq!(value, "value");
            }
            _ => panic!("Expected Config Set command"),
        }
    }

    #[test]
    fn test_global_args() {
        let args = Args::try_parse_from(["picode", "-vv", "--debug", "init"]).unwrap();
        
        assert_eq!(args.verbose, 2);
        assert!(args.debug);
    }
}