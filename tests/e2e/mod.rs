//! End-to-end tests for PiCode CLI functionality

pub mod cli_e2e;
pub mod interactive_e2e;
pub mod config_e2e;

use super::TestContext;
use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;

/// End-to-end test utilities
pub struct E2ETestRunner {
    pub binary_path: String,
    pub test_context: TestContext,
}

impl E2ETestRunner {
    pub fn new() -> picode::Result<Self> {
        let test_context = TestContext::new()?;
        Ok(Self {
            binary_path: "target/debug/picode".to_string(),
            test_context,
        })
    }

    pub fn command(&self) -> Command {
        Command::cargo_bin("picode").unwrap()
    }

    pub fn setup_test_environment(&self) -> picode::Result<()> {
        self.test_context.create_test_config()?;
        Ok(())
    }
}

/// CLI end-to-end tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_version() {
        let runner = E2ETestRunner::new().expect("Failed to create E2E runner");
        
        runner.command()
            .arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains("picode"));
    }

    #[tokio::test]
    async fn test_cli_help() {
        let runner = E2ETestRunner::new().expect("Failed to create E2E runner");
        
        runner.command()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("PiCode"))
            .stdout(predicate::str::contains("USAGE"));
    }

    #[tokio::test]
    async fn test_config_init() {
        let runner = E2ETestRunner::new().expect("Failed to create E2E runner");
        
        // This test will work once config command is implemented
        runner.command()
            .args(&["config", "init"])
            .assert()
            .success();
    }

    #[tokio::test]
    async fn test_invalid_command() {
        let runner = E2ETestRunner::new().expect("Failed to create E2E runner");
        
        runner.command()
            .arg("invalid-command")
            .assert()
            .failure()
            .stderr(predicate::str::contains("error"));
    }

    #[tokio::test]
    async fn test_interactive_mode_basic() {
        let runner = E2ETestRunner::new().expect("Failed to create E2E runner");
        runner.setup_test_environment().expect("Failed to setup test environment");
        
        // This test will need to be implemented once interactive mode is available
        // For now, we expect it to fail with missing implementation
        runner.command()
            .args(&["interactive"])
            .timeout(std::time::Duration::from_secs(5))
            .assert()
            .failure(); // Expected to fail until implementation is complete
    }

    #[tokio::test] 
    async fn test_execute_command() {
        let runner = E2ETestRunner::new().expect("Failed to create E2E runner");
        runner.setup_test_environment().expect("Failed to setup test environment");
        
        // This test will work once execute command is implemented
        runner.command()
            .args(&["execute", "--command", "echo hello", "--provider", "test"])
            .assert()
            .failure(); // Expected to fail until implementation is complete
    }

    #[tokio::test]
    async fn test_hooks_system() {
        let runner = E2ETestRunner::new().expect("Failed to create E2E runner");
        runner.setup_test_environment().expect("Failed to setup test environment");
        
        // Test hooks integration
        runner.command()
            .args(&["hooks", "list"])
            .assert()
            .failure(); // Expected to fail until hooks command is implemented
    }
}