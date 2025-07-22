//! CLI argument parsing and command structures

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "picode")]
#[command(about = "A terminal workspace with AI capabilities")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct CliArgs {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Session name
    #[arg(short, long)]
    pub session: Option<String>,

    /// Workspace directory
    #[arg(short, long)]
    pub workspace: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start interactive mode
    Interactive(InteractiveOptions),
    
    /// Execute a command with LLM assistance
    Execute {
        /// Command to execute
        command: String,
        /// LLM provider to use
        #[arg(short, long)]
        provider: Option<String>,
    },
    
    /// Configuration management
    Config(ConfigCommand),
    
    /// Hooks management
    Hooks(HooksCommand),
}

#[derive(Parser, Debug)]
pub struct InteractiveOptions {
    /// LLM provider to use
    #[arg(short, long)]
    pub provider: Option<String>,

    /// Model to use
    #[arg(short, long)]
    pub model: Option<String>,

    /// System prompt
    #[arg(long)]
    pub system_prompt: Option<String>,

    /// Disable auto-save
    #[arg(long)]
    pub no_auto_save: bool,
}

impl Default for InteractiveOptions {
    fn default() -> Self {
        Self {
            provider: None,
            model: None,
            system_prompt: None,
            no_auto_save: false,
        }
    }
}

#[derive(clap::Args, Debug)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    
    /// Initialize configuration
    Init,
}

#[derive(clap::Args, Debug)]
pub struct HooksCommand {
    #[command(subcommand)]
    pub action: HooksAction,
}

#[derive(Subcommand, Debug)]
pub enum HooksAction {
    /// List available hooks
    List,
    
    /// Add a new hook
    Add {
        /// Hook name
        name: String,
        /// Hook script path
        script: PathBuf,
    },
    
    /// Remove a hook
    Remove {
        /// Hook name
        name: String,
    },
    
    /// Run a hook manually
    Run {
        /// Hook name
        name: String,
        /// Hook arguments
        args: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_args_parsing() {
        let args = CliArgs::parse_from(&["picode", "--verbose"]);
        assert!(args.verbose);
        assert!(!args.no_color);
    }

    #[test]
    fn interactive_command() {
        let args = CliArgs::parse_from(&[
            "picode", 
            "interactive", 
            "--provider", "openai",
            "--model", "gpt-4"
        ]);
        
        match args.command {
            Some(Command::Interactive(opts)) => {
                assert_eq!(opts.provider, Some("openai".to_string()));
                assert_eq!(opts.model, Some("gpt-4".to_string()));
            }
            _ => panic!("Expected Interactive command"),
        }
    }

    #[test]
    fn execute_command() {
        let args = CliArgs::parse_from(&[
            "picode", 
            "execute", 
            "cargo test",
            "--provider", "anthropic"
        ]);
        
        match args.command {
            Some(Command::Execute { command, provider }) => {
                assert_eq!(command, "cargo test");
                assert_eq!(provider, Some("anthropic".to_string()));
            }
            _ => panic!("Expected Execute command"),
        }
    }

    #[test]
    fn config_command() {
        let args = CliArgs::parse_from(&["picode", "config", "set", "api_key", "test123"]);
        
        match args.command {
            Some(Command::Config(ConfigCommand { action: ConfigAction::Set { key, value } })) => {
                assert_eq!(key, "api_key");
                assert_eq!(value, "test123");
            }
            _ => panic!("Expected Config Set command"),
        }
    }
}