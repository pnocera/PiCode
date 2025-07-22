//! PiCode - A terminal workspace with AI capabilities
//! 
//! Claude Code compatible with OpenAPI LLMs, built on Rust and inspired by Zellij architecture.

// Re-export main modules for easy access
pub mod cli;
pub mod config;
pub mod error;
pub mod logging;

// Interactive and execution modules
pub mod interactive;
pub mod execute;

// Re-export workspace crates
pub use picode_core as core;
pub use picode_cli as cli_utils;
pub use picode_llm as llm;
pub use picode_hooks as hooks;

#[cfg(feature = "wasm")]
pub use picode_wasm as wasm;

/// PiCode version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration values
pub mod defaults {
    pub const DEFAULT_LLM_PROVIDER: &str = "openai";
    pub const DEFAULT_MODEL: &str = "gpt-4";
    pub const DEFAULT_SESSION_NAME: &str = "picode-session";
    pub const CONFIG_DIR: &str = ".picode";
    pub const HOOKS_DIR: &str = "hooks";
}

/// Common result type for PiCode operations
pub type Result<T> = std::result::Result<T, error::PiCodeError>;

/// Initialize PiCode with default configuration
pub async fn init() -> Result<config::Config> {
    logging::configure_logger();
    config::Config::load_default().await.map_err(error::PiCodeError::ConfigLocal)
}

/// Check if PiCode is properly configured
pub fn is_configured() -> bool {
    config::Config::exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_valid() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.chars().next().unwrap().is_ascii_digit());
    }

    #[test]
    fn defaults_are_valid() {
        assert!(!defaults::DEFAULT_LLM_PROVIDER.is_empty());
        assert!(!defaults::DEFAULT_MODEL.is_empty());
        assert!(!defaults::DEFAULT_SESSION_NAME.is_empty());
    }

    #[tokio::test]
    async fn init_returns_config() {
        // This test ensures initialization doesn't panic
        let result = init().await;
        // We expect it might fail in test environment, but shouldn't panic
        match result {
            Ok(_config) => {
                // Success case - config loaded properly
            },
            Err(_) => {
                // Expected failure in test environment without proper config
            }
        }
    }
}
