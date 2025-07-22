//! PiCode Integration Test Suite
//! 
//! Comprehensive testing framework for PiCode functionality

pub mod unit;
pub mod integration;
pub mod e2e;
pub mod performance;
pub mod security;
pub mod test_runner;

use std::path::PathBuf;
use tempfile::TempDir;
use picode::Result;

/// Test utilities and common functions
pub struct TestContext {
    pub temp_dir: TempDir,
    pub config_path: PathBuf,
    pub session_id: String,
}

impl TestContext {
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new().map_err(|e| picode::error::PiCodeError::Io(e))?;
        let config_path = temp_dir.path().join(".picode");
        let session_id = format!("test-session-{}", uuid::Uuid::new_v4());

        Ok(Self {
            temp_dir,
            config_path,
            session_id,
        })
    }

    pub fn create_test_config(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config_path)
            .map_err(|e| picode::error::PiCodeError::Io(e))?;
        
        let config_content = r#"
[llm]
provider = "test"
model = "test-model"
api_key = "test-key"

[workspace]
default_layout = "default"
plugins_dir = "plugins"
        "#;
        
        std::fs::write(
            self.config_path.join("config.toml"),
            config_content
        ).map_err(|e| picode::error::PiCodeError::Io(e))?;
        
        Ok(())
    }
}

/// Mock LLM provider for testing
pub struct MockLLMProvider {
    pub responses: Vec<String>,
    pub call_count: std::sync::atomic::AtomicUsize,
}

impl MockLLMProvider {
    pub fn new(responses: Vec<String>) -> Self {
        Self {
            responses,
            call_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Test assertion macros
#[macro_export]
macro_rules! assert_compilation_success {
    ($result:expr) => {
        match $result {
            Ok(_) => {},
            Err(e) => panic!("Compilation failed: {:?}", e),
        }
    };
}

#[macro_export]
macro_rules! assert_llm_response_valid {
    ($response:expr) => {
        assert!(!$response.is_empty(), "LLM response should not be empty");
        assert!($response.len() > 10, "LLM response should be substantial");
    };
}

#[macro_export]
macro_rules! assert_hook_execution {
    ($hook_result:expr) => {
        match $hook_result {
            Ok(status) => assert!(status.success(), "Hook should execute successfully"),
            Err(e) => panic!("Hook execution failed: {:?}", e),
        }
    };
}