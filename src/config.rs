//! Configuration management for PiCode

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::cli::CliArgs;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// LLM provider configuration
    pub llm: LlmConfig,
    
    /// UI/terminal configuration
    pub ui: UiConfig,
    
    /// Session management
    pub session: SessionConfig,
    
    /// Workspace settings
    pub workspace: WorkspaceConfig,
    
    /// Hooks configuration
    pub hooks: HooksConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: LlmConfig::default(),
            ui: UiConfig::default(),
            session: SessionConfig::default(),
            workspace: WorkspaceConfig::default(),
            hooks: HooksConfig::default(),
        }
    }
}

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Default provider to use
    pub default_provider: String,
    
    /// Default model for the provider
    pub default_model: String,
    
    /// Provider-specific configurations
    pub providers: HashMap<String, ProviderConfig>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            default_provider: "anthropic".to_string(),
            default_model: "claude-3-sonnet-20240229".to_string(),
            providers: HashMap::new(),
        }
    }
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// API endpoint URL
    pub endpoint: String,
    
    /// API key (stored securely)
    pub api_key_env: Option<String>,
    
    /// Default model for this provider
    pub default_model: Option<String>,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Color theme
    pub theme: String,
    
    /// Enable syntax highlighting
    pub syntax_highlighting: bool,
    
    /// Editor settings
    pub editor: EditorConfig,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            syntax_highlighting: true,
            editor: EditorConfig::default(),
        }
    }
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Tab size
    pub tab_size: usize,
    
    /// Use spaces instead of tabs
    pub use_spaces: bool,
    
    /// Enable line numbers
    pub line_numbers: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_size: 4,
            use_spaces: true,
            line_numbers: true,
        }
    }
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Default session name
    pub default_session: String,
    
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    
    /// Maximum session history
    pub max_history: usize,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            default_session: "main".to_string(),
            auto_save_interval: 300, // 5 minutes
            max_history: 100,
        }
    }
}

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Root directory for workspaces
    pub root_dir: Option<PathBuf>,
    
    /// Ignore patterns for file scanning
    pub ignore_patterns: Vec<String>,
    
    /// Maximum file size to process (in bytes)
    pub max_file_size: u64,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            root_dir: None,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                "build".to_string(),
                "dist".to_string(),
                ".DS_Store".to_string(),
            ],
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Hooks configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksConfig {
    /// Hooks directory
    pub hooks_dir: Option<PathBuf>,
    
    /// Enable hooks by default
    pub enabled: bool,
    
    /// Hook timeout in seconds
    pub timeout: u64,
}

impl Default for HooksConfig {
    fn default() -> Self {
        Self {
            hooks_dir: None,
            enabled: true,
            timeout: 30,
        }
    }
}

impl Config {
    /// Load configuration from default location
    pub async fn load_default() -> Result<Config, ConfigError> {
        // For now, return default configuration
        // TODO: Implement actual file loading
        Ok(Config::default())
    }
    
    /// Save configuration to file
    pub async fn save(&self) -> Result<(), ConfigError> {
        // TODO: Implement actual file saving
        println!("Configuration would be saved (not implemented yet)");
        Ok(())
    }
    
    /// Create configuration from CLI arguments
    pub async fn try_from(args: &CliArgs) -> crate::Result<Config> {
        let mut config = Config::load_default().await.map_err(crate::error::PiCodeError::ConfigLocal)?;
        
        // Override with CLI arguments
        if args.verbose > 0 {
            println!("Verbose mode enabled (level: {})", args.verbose);
        }
        
        if args.debug {
            println!("Debug mode enabled");
        }
        
        Ok(config)
    }
    
    /// Get default configuration path
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("picode")
            .join("config.toml")
    }
    
    /// Check if configuration exists
    pub fn exists() -> bool {
        Self::default_config_path().exists()
    }
}

/// Configuration errors
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.llm.default_provider, "anthropic");
        assert_eq!(config.ui.theme, "dark");
        assert!(config.ui.syntax_highlighting);
    }
    
    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(config.llm.default_provider, deserialized.llm.default_provider);
    }
}

/// Handle configuration commands
pub async fn handle_command(cmd: crate::cli::ConfigCommand) -> crate::Result<()> {
    // Basic config command handling - simplified for now
    println!("Config command handling not yet implemented: {:?}", cmd);
    Ok(())
}