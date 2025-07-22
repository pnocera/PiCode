//! Error handling for PiCode

use thiserror::Error;

/// Main error type for PiCode
#[derive(Error, Debug)]
pub enum PiCodeError {
    #[error("Core error: {0}")]
    Core(#[from] picode_core::CoreError),
    
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Configuration error (local): {0}")]
    ConfigLocal(#[from] crate::config::ConfigError),
    
    #[error("CLI error: {0}")]
    Cli(#[from] CliError),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("LLM error: {0}")]
    Llm(String),
    
    #[error("Interactive mode error: {0}")]
    Interactive(String),
    
    #[error("Hook error: {0}")]
    Hook(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),
    
    #[error("YAML serialization error: {0}")]
    YamlSerialization(#[from] serde_yaml::Error),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Permission denied: {0}")]
    Permission(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Cancelled operation: {0}")]
    Cancelled(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Configuration-specific errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing configuration: {0}")]
    Missing(String),
    
    #[error("Invalid configuration value: {0}")]
    Invalid(String),
    
    #[error("Configuration file not found: {0}")]
    FileNotFound(String),
    
    #[error("Configuration file is corrupted: {0}")]
    Corrupted(String),
    
    #[error("Permission denied accessing config: {0}")]
    Permission(String),
    
    #[error("Config serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Config YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// CLI-specific errors
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("Missing required argument: {0}")]
    MissingArgument(String),
    
    #[error("Invalid argument value: {0}")]
    InvalidArgument(String),
    
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Help requested")]
    Help,
    
    #[error("Version requested")]
    Version,
}

/// Result type for PiCode operations
pub type Result<T> = std::result::Result<T, PiCodeError>;

/// Result type for configuration operations
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

/// Result type for CLI operations
pub type CliResult<T> = std::result::Result<T, CliError>;

/// Helper trait for converting errors to PiCodeError
pub trait IntoPiCodeError {
    fn into_picode_error(self) -> PiCodeError;
}

impl IntoPiCodeError for String {
    fn into_picode_error(self) -> PiCodeError {
        PiCodeError::Internal(self)
    }
}

impl IntoPiCodeError for &str {
    fn into_picode_error(self) -> PiCodeError {
        PiCodeError::Internal(self.to_string())
    }
}

/// Helper macro for creating PiCode errors
#[macro_export]
macro_rules! picode_error {
    ($kind:ident, $msg:expr) => {
        $crate::error::PiCodeError::$kind($msg.to_string())
    };
    ($kind:ident, $fmt:expr, $($arg:tt)*) => {
        $crate::error::PiCodeError::$kind(format!($fmt, $($arg)*))
    };
}

/// Helper macro for creating config errors
#[macro_export]
macro_rules! config_error {
    ($kind:ident, $msg:expr) => {
        $crate::error::ConfigError::$kind($msg.to_string())
    };
    ($kind:ident, $fmt:expr, $($arg:tt)*) => {
        $crate::error::ConfigError::$kind(format!($fmt, $($arg)*))
    };
}

/// Helper macro for creating CLI errors
#[macro_export]
macro_rules! cli_error {
    ($kind:ident, $msg:expr) => {
        $crate::error::CliError::$kind($msg.to_string())
    };
    ($kind:ident, $fmt:expr, $($arg:tt)*) => {
        $crate::error::CliError::$kind(format!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picode_error_display() {
        let error = PiCodeError::NotFound("test resource".to_string());
        assert_eq!(error.to_string(), "Resource not found: test resource");
    }

    #[test]
    fn config_error_display() {
        let error = ConfigError::Missing("api_key".to_string());
        assert_eq!(error.to_string(), "Missing configuration: api_key");
    }

    #[test]
    fn cli_error_display() {
        let error = CliError::InvalidCommand("unknown-cmd".to_string());
        assert_eq!(error.to_string(), "Invalid command: unknown-cmd");
    }

    #[test]
    fn error_conversion() {
        let config_error = ConfigError::Missing("test".to_string());
        let picode_error: PiCodeError = config_error.into();
        
        match picode_error {
            PiCodeError::Config(ConfigError::Missing(msg)) => assert_eq!(msg, "test"),
            _ => panic!("Wrong error type conversion"),
        }
    }

    #[test]
    fn into_picode_error_trait() {
        let error1 = "test error".into_picode_error();
        let error2 = "test error".to_string().into_picode_error();
        
        match (error1, error2) {
            (PiCodeError::Internal(msg1), PiCodeError::Internal(msg2)) => {
                assert_eq!(msg1, "test error");
                assert_eq!(msg2, "test error");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn error_macros() {
        let error1 = picode_error!(NotFound, "test resource");
        let error2 = picode_error!(Internal, "formatted error: {}", 42);
        
        assert_eq!(error1.to_string(), "Resource not found: test resource");
        assert_eq!(error2.to_string(), "Internal error: formatted error: 42");
        
        let config_err = config_error!(Invalid, "bad value: {}", "test");
        assert_eq!(config_err.to_string(), "Invalid configuration value: bad value: test");
        
        let cli_err = cli_error!(MissingArgument, "--required-arg");
        assert_eq!(cli_err.to_string(), "Missing required argument: --required-arg");
    }
}