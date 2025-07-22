//! Command execution and management for PiCode

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use thiserror::Error;
use tokio::process::Command as TokioCommand;
use uuid::Uuid;

/// Unique identifier for a command
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(pub Uuid);

impl CommandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for CommandId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Command to be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: CommandId,
    pub program: String,
    pub args: Vec<String>,
    pub working_dir: Option<std::path::PathBuf>,
    pub env: HashMap<String, String>,
    pub stdin_data: Option<String>,
    pub timeout: Option<std::time::Duration>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Command {
    pub fn new(program: String) -> Self {
        Self {
            id: CommandId::new(),
            program,
            args: Vec::new(),
            working_dir: None,
            env: HashMap::new(),
            stdin_data: None,
            timeout: None,
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    
    pub fn with_working_dir(mut self, dir: std::path::PathBuf) -> Self {
        self.working_dir = Some(dir);
        self
    }
    
    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.env.insert(key, value);
        self
    }
    
    pub fn with_stdin(mut self, data: String) -> Self {
        self.stdin_data = Some(data);
        self
    }
    
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    pub async fn execute(&self) -> Result<CommandResult, CommandError> {
        let mut cmd = TokioCommand::new(&self.program);
        cmd.args(&self.args);
        
        if let Some(working_dir) = &self.working_dir {
            cmd.current_dir(working_dir);
        }
        
        for (key, value) in &self.env {
            cmd.env(key, value);
        }
        
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        if self.stdin_data.is_some() {
            cmd.stdin(Stdio::piped());
        }
        
        let start_time = std::time::Instant::now();
        
        let child = cmd.spawn()
            .map_err(|e| CommandError::ExecutionFailed(e.to_string()))?;
        
        let output = if let Some(timeout) = self.timeout {
            tokio::time::timeout(timeout, child.wait_with_output())
                .await
                .map_err(|_| CommandError::Timeout)?
                .map_err(|e| CommandError::ExecutionFailed(e.to_string()))?
        } else {
            child.wait_with_output()
                .await
                .map_err(|e| CommandError::ExecutionFailed(e.to_string()))?
        };
        
        let duration = start_time.elapsed();
        
        let status = if output.status.success() {
            CommandStatus::Success
        } else {
            CommandStatus::Failed(output.status.code().unwrap_or(-1))
        };
        
        Ok(CommandResult {
            command_id: self.id.clone(),
            status,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration,
            started_at: chrono::Utc::now() - chrono::Duration::milliseconds(duration.as_millis() as i64),
            finished_at: chrono::Utc::now(),
        })
    }
}

/// Result of command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_id: CommandId,
    pub status: CommandStatus,
    pub stdout: String,
    pub stderr: String,
    pub duration: std::time::Duration,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: chrono::DateTime<chrono::Utc>,
}

/// Command execution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommandStatus {
    Success,
    Failed(i32),
    Timeout,
    Interrupted,
}

impl CommandStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, CommandStatus::Success)
    }
    
    pub fn exit_code(&self) -> Option<i32> {
        match self {
            CommandStatus::Success => Some(0),
            CommandStatus::Failed(code) => Some(*code),
            _ => None,
        }
    }
}

/// Command-related errors
#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Command timed out")]
    Timeout,
    
    #[error("Command was interrupted")]
    Interrupted,
    
    #[error("Invalid command: {0}")]
    Invalid(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Command builder for common operations
pub struct CommandBuilder;

impl CommandBuilder {
    /// Create a shell command
    pub fn shell(command: &str) -> Command {
        #[cfg(unix)]
        {
            Command::new("sh".to_string())
                .with_args(vec!["-c".to_string(), command.to_string()])
        }
        
        #[cfg(windows)]
        {
            Command::new("cmd".to_string())
                .with_args(vec!["/C".to_string(), command.to_string()])
        }
    }
    
    /// Create a git command
    pub fn git(args: Vec<String>) -> Command {
        Command::new("git".to_string()).with_args(args)
    }
    
    /// Create a cargo command
    pub fn cargo(args: Vec<String>) -> Command {
        Command::new("cargo".to_string()).with_args(args)
    }
    
    /// Create an npm command
    pub fn npm(args: Vec<String>) -> Command {
        Command::new("npm".to_string()).with_args(args)
    }
    
    /// Create a python command
    pub fn python(script: String, args: Vec<String>) -> Command {
        let mut all_args = vec![script];
        all_args.extend(args);
        Command::new("python".to_string()).with_args(all_args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_id_generation() {
        let id1 = CommandId::new();
        let id2 = CommandId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn command_creation() {
        let cmd = Command::new("echo".to_string())
            .with_args(vec!["hello".to_string()])
            .with_env("TEST_VAR".to_string(), "test_value".to_string())
            .with_timeout(std::time::Duration::from_secs(30));
        
        assert_eq!(cmd.program, "echo");
        assert_eq!(cmd.args, vec!["hello"]);
        assert_eq!(cmd.env.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(cmd.timeout, Some(std::time::Duration::from_secs(30)));
    }

    #[tokio::test]
    async fn command_execution_success() {
        let cmd = Command::new("echo".to_string())
            .with_args(vec!["test".to_string()]);
        
        let result = cmd.execute().await.unwrap();
        assert!(result.status.is_success());
        assert!(result.stdout.contains("test"));
        assert!(result.stderr.is_empty());
    }

    #[tokio::test]
    async fn command_execution_failure() {
        let cmd = Command::new("nonexistent_command_12345".to_string());
        let result = cmd.execute().await;
        assert!(result.is_err());
    }

    #[test]
    fn command_status_checks() {
        assert!(CommandStatus::Success.is_success());
        assert!(!CommandStatus::Failed(1).is_success());
        assert!(!CommandStatus::Timeout.is_success());
        
        assert_eq!(CommandStatus::Success.exit_code(), Some(0));
        assert_eq!(CommandStatus::Failed(42).exit_code(), Some(42));
        assert_eq!(CommandStatus::Timeout.exit_code(), None);
    }

    #[test]
    fn command_builder_shell() {
        let cmd = CommandBuilder::shell("echo hello");
        
        #[cfg(unix)]
        {
            assert_eq!(cmd.program, "sh");
            assert_eq!(cmd.args, vec!["-c", "echo hello"]);
        }
        
        #[cfg(windows)]
        {
            assert_eq!(cmd.program, "cmd");
            assert_eq!(cmd.args, vec!["/C", "echo hello"]);
        }
    }

    #[test]
    fn command_builder_git() {
        let cmd = CommandBuilder::git(vec!["status".to_string()]);
        assert_eq!(cmd.program, "git");
        assert_eq!(cmd.args, vec!["status"]);
    }

    #[test]
    fn command_builder_cargo() {
        let cmd = CommandBuilder::cargo(vec!["test".to_string()]);
        assert_eq!(cmd.program, "cargo");
        assert_eq!(cmd.args, vec!["test"]);
    }

    #[tokio::test]
    async fn command_with_working_dir() {
        let temp_dir = std::env::temp_dir();
        let cmd = Command::new("pwd".to_string())
            .with_working_dir(temp_dir.clone());
        
        // Skip this test on Windows as pwd doesn't exist
        #[cfg(unix)]
        {
            let result = cmd.execute().await.unwrap();
            assert!(result.status.is_success());
            assert!(result.stdout.trim().contains(&temp_dir.to_string_lossy()));
        }
    }

    #[tokio::test]
    async fn command_with_env() {
        let cmd = Command::new("env".to_string())
            .with_env("PICODE_TEST".to_string(), "test_value".to_string());
        
        // Skip this test on Windows as env command is different
        #[cfg(unix)]
        {
            let result = cmd.execute().await.unwrap();
            assert!(result.status.is_success());
            assert!(result.stdout.contains("PICODE_TEST=test_value"));
        }
    }
}