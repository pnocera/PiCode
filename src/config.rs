//! Configuration management for PiCode

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::cli::{CliArgs, ConfigAction};
use crate::error::{ConfigError, ConfigResult};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// LLM providers configuration
    pub llm: LlmConfig,
    
    /// Terminal and UI settings
    pub ui: UiConfig,
    
    /// Session management settings
    pub session: SessionConfig,
    
    /// Workspace settings
    pub workspace: WorkspaceConfig,
    
    /// Hooks configuration
    pub hooks: HooksConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Default LLM provider
    pub default_provider: String,
    
    /// Provider configurations
    pub providers: HashMap<String, ProviderConfig>,
    
    /// Default model per provider
    pub default_models: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// API endpoint URL
    pub endpoint: String,
    
    /// API key (stored securely)
    pub api_key: Option<String>,
    
    /// Additional headers
    pub headers: HashMap<String, String>,
    
    /// Timeout in seconds
    pub timeout: u64,
    
    /// Max tokens per request
    pub max_tokens: Option<u32>,
    
    /// Custom configuration
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Color scheme
    pub theme: String,
    
    /// Show line numbers
    pub show_line_numbers: bool,
    
    /// Enable syntax highlighting
    pub syntax_highlighting: bool,
    
    /// Tab size
    pub tab_size: usize,
    
    /// Word wrap
    pub word_wrap: bool,
    
    /// Font family (for GUI mode)
    pub font_family: Option<String>,
    
    /// Font size (for GUI mode)
    pub font_size: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Default session name
    pub default_name: String,
    
    /// Auto-save sessions
    pub auto_save: bool,
    
    /// Session persistence directory
    pub sessions_dir: PathBuf,
    
    /// Maximum number of sessions to keep
    pub max_sessions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Default workspace directory
    pub default_directory: Option<PathBuf>,
    
    /// Auto-scan workspace on startup
    pub auto_scan: bool,
    
    /// Git integration enabled
    pub git_enabled: bool,
    
    /// File watching enabled
    pub file_watching: bool,
    
    /// Ignore patterns
    pub ignore_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksConfig {
    /// Hooks directory
    pub hooks_dir: PathBuf,
    
    /// Enable hooks
    pub enabled: bool,
    
    /// Hook timeout in seconds
    pub timeout: u64,
    
    /// Custom hook configurations
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let picode_dir = home_dir.join(".picode");
        
        let mut providers = HashMap::new();
        
        // OpenAI provider
        providers.insert("openai".to_string(), ProviderConfig {
            endpoint: "https://api.openai.com/v1".to_string(),
            api_key: None,
            headers: HashMap::new(),
            timeout: 30,
            max_tokens: Some(4000),
            custom: HashMap::new(),
        });
        
        // Anthropic provider
        providers.insert("anthropic".to_string(), ProviderConfig {
            endpoint: "https://api.anthropic.com/v1".to_string(),
            api_key: None,
            headers: HashMap::new(),
            timeout: 30,
            max_tokens: Some(4000),
            custom: HashMap::new(),
        });
        
        let mut default_models = HashMap::new();
        default_models.insert("openai".to_string(), "gpt-4".to_string());
        default_models.insert("anthropic".to_string(), "claude-3-sonnet-20240229".to_string());
        
        Self {
            llm: LlmConfig {
                default_provider: "openai".to_string(),
                providers,
                default_models,
            },
            ui: UiConfig {
                theme: "default".to_string(),
                show_line_numbers: true,
                syntax_highlighting: true,
                tab_size: 4,
                word_wrap: false,
                font_family: None,
                font_size: None,
            },
            session: SessionConfig {
                default_name: "default".to_string(),
                auto_save: true,
                sessions_dir: picode_dir.join("sessions"),
                max_sessions: 50,
            },
            workspace: WorkspaceConfig {
                default_directory: None,
                auto_scan: true,
                git_enabled: true,
                file_watching: true,
                ignore_patterns: vec![
                    "target/".to_string(),
                    "node_modules/".to_string(),
                    ".git/".to_string(),
                    "*.tmp".to_string(),
                ],
            },
            hooks: HooksConfig {
                hooks_dir: picode_dir.join("hooks"),
                enabled: true,
                timeout: 30,
                custom: HashMap::new(),
            },
        }
    }
}

impl Config {
    /// Load configuration from default location
    pub async fn load_default() -> ConfigResult<Self> {
        let config_path = Self::default_config_path();
        if config_path.exists() {
            Self::load_from_path(config_path).await
        } else {
            Ok(Self::default())
        }
    }
    
    /// Load configuration from file
    pub async fn load_from_path(path: PathBuf) -> ConfigResult<Self> {
        let content = tokio::fs::read_to_string(&path).await?;
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&content).map_err(ConfigError::from)
        } else {
            // Assume YAML
            serde_yaml::from_str(&content).map_err(ConfigError::from)
        }
    }
    
    /// Save configuration to default location
    pub async fn save(&self) -> ConfigResult<()> {
        let config_path = Self::default_config_path();
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let content = serde_yaml::to_string(self)?;
        tokio::fs::write(config_path, content).await?;
        
        Ok(())
    }
    
    /// Save configuration to specific path
    pub async fn save_to_path(&self, path: PathBuf) -> ConfigResult<()> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let content = if path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::to_string_pretty(self)?
        } else {
            serde_yaml::to_string(self)?
        };
        
        tokio::fs::write(path, content).await?;
        Ok(())
    }
    
    /// Check if configuration file exists
    pub fn exists() -> bool {
        Self::default_config_path().exists()
    }
    
    /// Get default configuration file path
    pub fn default_config_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".picode").join("config.yaml")
    }
    
    /// Create configuration from CLI arguments
    pub async fn try_from(args: &CliArgs) -> ConfigResult<Self> {
        if let Some(config_path) = &args.config {
            Self::load_from_path(config_path.clone()).await
        } else {
            Self::load_default().await
        }
    }
    
    /// Get LLM provider configuration
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.llm.providers.get(name)
    }
    
    /// Get default model for provider
    pub fn get_default_model(&self, provider: &str) -> Option<&String> {
        self.llm.default_models.get(provider)
    }
    
    /// Set configuration value by key path
    pub fn set_value(&mut self, key_path: &str, value: &str) -> ConfigResult<()> {
        let keys: Vec<&str> = key_path.split('.').collect();
        
        match keys.as_slice() {
            ["llm", "default_provider"] => {
                self.llm.default_provider = value.to_string();
            }
            ["ui", "theme"] => {
                self.ui.theme = value.to_string();
            }
            ["ui", "tab_size"] => {
                self.ui.tab_size = value.parse()
                    .map_err(|_| ConfigError::Invalid(format!("Invalid tab_size: {}", value)))?;
            }
            ["session", "default_name"] => {
                self.session.default_name = value.to_string();
            }
            ["workspace", "git_enabled"] => {
                self.workspace.git_enabled = value.parse()
                    .map_err(|_| ConfigError::Invalid(format!("Invalid boolean: {}", value)))?;
            }
            _ => {
                return Err(ConfigError::Invalid(format!("Unknown config key: {}", key_path)));
            }
        }
        
        Ok(())
    }
    
    /// Get configuration value by key path
    pub fn get_value(&self, key_path: &str) -> Option<String> {
        let keys: Vec<&str> = key_path.split('.').collect();
        
        match keys.as_slice() {
            ["llm", "default_provider"] => Some(self.llm.default_provider.clone()),
            ["ui", "theme"] => Some(self.ui.theme.clone()),
            ["ui", "tab_size"] => Some(self.ui.tab_size.to_string()),
            ["session", "default_name"] => Some(self.session.default_name.clone()),
            ["workspace", "git_enabled"] => Some(self.workspace.git_enabled.to_string()),
            _ => None,
        }
    }
}

/// Handle configuration commands
pub async fn handle_command(cmd: crate::cli::ConfigCommand) -> crate::Result<()> {
    use crate::cli::ConfigCommand;
    
    match cmd.action {
        ConfigAction::Show => {
            let config = Config::load_default().await?;
            let yaml = serde_yaml::to_string(&config)?;
            println!("{}", yaml);
        }
        
        ConfigAction::Set { key, value } => {
            let mut config = Config::load_default().await?;
            config.set_value(&key, &value)?;
            config.save().await?;
            println!("Configuration updated: {} = {}", key, value);
        }
        
        ConfigAction::Get { key } => {
            let config = Config::load_default().await?;
            match config.get_value(&key) {
                Some(value) => println!("{}", value),
                None => eprintln!("Configuration key not found: {}", key),
            }
        }
        
        ConfigAction::Init => {
            let config = Config::default();
            config.save().await?;
            println!("Configuration initialized at: {:?}", Config::default_config_path());
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn config_default() {
        let config = Config::default();
        assert_eq!(config.llm.default_provider, "openai");
        assert!(config.llm.providers.contains_key("openai"));
        assert!(config.llm.providers.contains_key("anthropic"));
        assert_eq!(config.ui.tab_size, 4);
        assert!(config.workspace.git_enabled);
    }

    #[test]
    fn config_value_access() {
        let mut config = Config::default();
        
        // Test setting values
        config.set_value("llm.default_provider", "anthropic").unwrap();
        assert_eq!(config.llm.default_provider, "anthropic");
        
        config.set_value("ui.tab_size", "8").unwrap();
        assert_eq!(config.ui.tab_size, 8);
        
        // Test getting values
        assert_eq!(config.get_value("llm.default_provider"), Some("anthropic".to_string()));
        assert_eq!(config.get_value("ui.tab_size"), Some("8".to_string()));
        assert_eq!(config.get_value("nonexistent.key"), None);
    }

    #[test]
    fn config_invalid_values() {
        let mut config = Config::default();
        
        let result = config.set_value("ui.tab_size", "invalid");
        assert!(result.is_err());
        
        let result = config.set_value("workspace.git_enabled", "not_a_boolean");
        assert!(result.is_err());
        
        let result = config.set_value("unknown.key", "value");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn config_serialization() {
        let config = Config::default();
        
        // Test JSON serialization
        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config.llm.default_provider, deserialized.llm.default_provider);
        
        // Test YAML serialization
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config.ui.theme, deserialized.ui.theme);
    }

    #[tokio::test]
    async fn config_file_operations() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.yaml");
        
        let original_config = Config::default();
        original_config.save_to_path(config_path.clone()).await.unwrap();
        
        let loaded_config = Config::load_from_path(config_path).await.unwrap();
        assert_eq!(original_config.llm.default_provider, loaded_config.llm.default_provider);
        assert_eq!(original_config.ui.tab_size, loaded_config.ui.tab_size);
    }

    #[test]
    fn provider_config() {
        let config = Config::default();
        
        let openai_provider = config.get_provider("openai").unwrap();
        assert_eq!(openai_provider.endpoint, "https://api.openai.com/v1");
        assert_eq!(openai_provider.timeout, 30);
        
        let anthropic_provider = config.get_provider("anthropic").unwrap();
        assert_eq!(anthropic_provider.endpoint, "https://api.anthropic.com/v1");
        
        assert!(config.get_provider("nonexistent").is_none());
    }

    #[test]
    fn default_models() {
        let config = Config::default();
        
        assert_eq!(config.get_default_model("openai"), Some(&"gpt-4".to_string()));
        assert_eq!(config.get_default_model("anthropic"), Some(&"claude-3-sonnet-20240229".to_string()));
        assert_eq!(config.get_default_model("nonexistent"), None);
    }
}