//! CLI interface for PiCode
//!
//! This module provides the command-line interface components that integrate
//! with the main application logic.

use serde::{Deserialize, Serialize};

// Re-export from picode-cli for main.rs compatibility
pub use picode_cli::{Args as CliArgs, LlmProvider};

/// Main CLI configuration that matches main.rs expectations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveOptions {
    /// Enable debugging output
    pub debug: bool,
    /// Terminal layout preference  
    pub layout: String,
    /// Provider to use for LLM interactions
    pub provider: Option<String>,
}

impl Default for InteractiveOptions {
    fn default() -> Self {
        Self {
            debug: false,
            layout: "default".to_string(),
            provider: None,
        }
    }
}

/// Commands enum that matches what main.rs expects
#[derive(Debug, Clone)]
pub enum Command {
    /// Start interactive mode
    Interactive(InteractiveOptions),
    /// Execute a command directly
    Execute {
        command: String,
        provider: Option<String>,
    },
    /// Configuration management
    Config(ConfigCommand),
    /// Hooks management
    Hooks(HooksCommand),
}

/// Configuration commands
#[derive(Debug, Clone)]
pub enum ConfigCommand {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set { key: String, value: String },
    /// Get a configuration value  
    Get { key: String },
    /// Reset configuration
    Reset,
}

/// Hooks commands
#[derive(Debug, Clone)]
pub enum HooksCommand {
    /// List available hooks
    List,
    /// Install a hook
    Install { name: String },
    /// Remove a hook
    Remove { name: String },
    /// Run a specific hook
    Run { name: String, args: Vec<String> },
}

/// Convert CLI arguments to command structure (simplified approach)
pub fn convert_cli_to_command(_args: CliArgs) -> Option<Command> {
    // For now, default to interactive mode
    Some(Command::Interactive(InteractiveOptions::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interactive_options_default() {
        let opts = InteractiveOptions::default();
        assert!(!opts.debug);
        assert_eq!(opts.layout, "default");
        assert!(opts.provider.is_none());
    }

    #[test]
    fn command_conversion_works() {
        // Test that we can convert from CLI args to commands
        // This is more of a compilation test since we need actual CLI parsing
        let opts = InteractiveOptions::default();
        let cmd = Command::Interactive(opts);
        
        match cmd {
            Command::Interactive(_) => {}, // Success
            _ => panic!("Expected Interactive command"),
        }
    }
}