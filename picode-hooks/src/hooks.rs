//! Hook system for PiCode
//!
//! This module provides a flexible hook system that allows users to run custom scripts
//! at various points in the PiCode workflow.

use crate::registry::HookRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command as AsyncCommand;
use tracing::{debug, error, info, warn};

/// Hook execution errors
#[derive(Error, Debug)]
pub enum HookError {
    #[error("Hook not found: {0}")]
    NotFound(String),
    
    #[error("Hook script not executable: {0}")]
    NotExecutable(String),
    
    #[error("Hook execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Hook timeout: {0}")]
    Timeout(String),
    
    #[error("Invalid hook configuration: {0}")]
    InvalidConfig(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Hook execution result
pub type HookResult<T> = Result<T, HookError>;

/// Hook execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookContext {
    /// Hook name
    pub name: String,
    
    /// Current working directory
    pub cwd: PathBuf,
    
    /// Environment variables
    pub env: HashMap<String, String>,
    
    /// Hook arguments
    pub args: Vec<String>,
    
    /// Hook metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl HookContext {
    /// Create a new hook context
    pub fn new(name: String) -> Self {
        Self {
            name,
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            env: std::env::vars().collect(),
            args: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Set working directory
    pub fn with_cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = cwd;
        self
    }
    
    /// Add environment variable
    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.env.insert(key, value);
        self
    }
    
    /// Set arguments
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Hook execution output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookOutput {
    /// Exit status code
    pub status_code: i32,
    
    /// Standard output
    pub stdout: String,
    
    /// Standard error
    pub stderr: String,
    
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    
    /// Whether the hook succeeded
    pub success: bool,
}

/// Hook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    /// Hook name
    pub name: String,
    
    /// Hook description
    pub description: Option<String>,
    
    /// Script path
    pub script: PathBuf,
    
    /// Execution timeout in seconds
    pub timeout_seconds: Option<u64>,
    
    /// Whether the hook is enabled
    pub enabled: bool,
    
    /// Hook triggers (events that trigger this hook)
    pub triggers: Vec<String>,
    
    /// Environment variables to set
    pub env: HashMap<String, String>,
    
    /// Working directory override
    pub cwd: Option<PathBuf>,
    
    /// Hook priority (lower number = higher priority)
    pub priority: i32,
}

impl Hook {
    /// Create a new hook
    pub fn new(name: String, script: PathBuf) -> Self {
        Self {
            name,
            description: None,
            script,
            timeout_seconds: Some(30), // Default 30 second timeout
            enabled: true,
            triggers: Vec::new(),
            env: HashMap::new(),
            cwd: None,
            priority: 0,
        }
    }
    
    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    /// Set timeout
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }
    
    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Add trigger
    pub fn with_trigger(mut self, trigger: String) -> Self {
        self.triggers.push(trigger);
        self
    }
    
    /// Set working directory
    pub fn with_cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = Some(cwd);
        self
    }
    
    /// Set priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
}

/// Hook manager - coordinates hook execution
pub struct HookManager {
    /// Hook registry
    registry: HookRegistry,
    
    /// Base hooks directory
    hooks_dir: PathBuf,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new(hooks_dir: PathBuf) -> Self {
        Self {
            registry: HookRegistry::new(),
            hooks_dir,
        }
    }
    
    /// Initialize hook manager
    pub async fn init(&mut self) -> HookResult<()> {
        info!("Initializing hook manager");
        
        // Create hooks directory if it doesn't exist
        if !self.hooks_dir.exists() {
            std::fs::create_dir_all(&self.hooks_dir)
                .map_err(HookError::Io)?;
        }
        
        // Load hooks from directory
        self.load_hooks().await?;
        
        Ok(())
    }
    
    /// Load hooks from the hooks directory
    async fn load_hooks(&mut self) -> HookResult<()> {
        debug!("Loading hooks from: {}", self.hooks_dir.display());
        
        let entries = std::fs::read_dir(&self.hooks_dir)
            .map_err(HookError::Io)?;
        
        for entry in entries {
            let entry = entry.map_err(HookError::Io)?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    let hook = Hook::new(name.to_string(), path.clone());
                    self.registry.register(hook)?;
                    debug!("Loaded hook: {}", name);
                }
            }
        }
        
        info!("Loaded {} hooks", self.registry.count());
        Ok(())
    }
    
    /// Register a new hook
    pub fn register_hook(&mut self, hook: Hook) -> HookResult<()> {
        info!("Registering hook: {}", hook.name);
        self.registry.register(hook)
    }
    
    /// Unregister a hook
    pub fn unregister_hook(&mut self, name: &str) -> HookResult<()> {
        info!("Unregistering hook: {}", name);
        self.registry.unregister(name)
    }
    
    /// Get a hook by name
    pub fn get_hook(&self, name: &str) -> Option<&Hook> {
        self.registry.get(name)
    }
    
    /// List all hooks
    pub fn list_hooks(&self) -> Vec<&Hook> {
        self.registry.list()
    }
    
    /// List hooks for a specific trigger
    pub fn list_hooks_for_trigger(&self, trigger: &str) -> Vec<&Hook> {
        self.registry.list_for_trigger(trigger)
    }
    
    /// Execute a hook by name
    pub async fn execute_hook(&self, name: &str, context: HookContext) -> HookResult<HookOutput> {
        let hook = self.get_hook(name)
            .ok_or_else(|| HookError::NotFound(name.to_string()))?;
        
        self.execute_hook_impl(hook, context).await
    }
    
    /// Execute all hooks for a trigger
    pub async fn execute_trigger(&self, trigger: &str, context: HookContext) -> HookResult<Vec<HookOutput>> {
        let hooks = self.list_hooks_for_trigger(trigger);
        
        if hooks.is_empty() {
            debug!("No hooks found for trigger: {}", trigger);
            return Ok(Vec::new());
        }
        
        info!("Executing {} hooks for trigger: {}", hooks.len(), trigger);
        
        let mut outputs = Vec::new();
        
        // Sort hooks by priority (lower number = higher priority)
        let mut sorted_hooks = hooks;
        sorted_hooks.sort_by_key(|h| h.priority);
        
        for hook in sorted_hooks {
            if hook.enabled {
                let output = self.execute_hook_impl(hook, context.clone()).await?;
                outputs.push(output);
            } else {
                debug!("Skipping disabled hook: {}", hook.name);
            }
        }
        
        Ok(outputs)
    }
    
    /// Internal hook execution implementation
    async fn execute_hook_impl(&self, hook: &Hook, context: HookContext) -> HookResult<HookOutput> {
        info!("Executing hook: {}", hook.name);
        
        if !hook.script.exists() {
            return Err(HookError::NotFound(format!(
                "Script not found: {}", 
                hook.script.display()
            )));
        }
        
        // Check if script is executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = hook.script.metadata().map_err(HookError::Io)?;
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 == 0 {
                return Err(HookError::NotExecutable(hook.script.display().to_string()));
            }
        }
        
        let start_time = std::time::Instant::now();
        
        // Build command
        let mut cmd = AsyncCommand::new(&hook.script);
        
        // Set working directory
        let cwd = hook.cwd.as_ref().unwrap_or(&context.cwd);
        cmd.current_dir(cwd);
        
        // Set environment variables
        cmd.envs(&context.env);
        for (key, value) in &hook.env {
            cmd.env(key, value);
        }
        
        // Add arguments
        cmd.args(&context.args);
        
        // Set up stdio
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        
        // Execute with timeout
        let timeout_duration = hook.timeout_seconds
            .map(std::time::Duration::from_secs)
            .unwrap_or(std::time::Duration::from_secs(30));
        
        let output = match tokio::time::timeout(timeout_duration, cmd.output()).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                error!("Hook execution failed: {}", e);
                return Err(HookError::ExecutionFailed(e.to_string()));
            },
            Err(_) => {
                error!("Hook execution timed out: {}", hook.name);
                return Err(HookError::Timeout(hook.name.clone()));
            },
        };
        
        let duration = start_time.elapsed();
        
        let hook_output = HookOutput {
            status_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms: duration.as_millis() as u64,
            success: output.status.success(),
        };
        
        if hook_output.success {
            info!("Hook completed successfully: {} ({}ms)", hook.name, hook_output.duration_ms);
        } else {
            warn!("Hook failed: {} (exit code: {})", hook.name, hook_output.status_code);
            if !hook_output.stderr.is_empty() {
                warn!("Hook stderr: {}", hook_output.stderr.trim());
            }
        }
        
        Ok(hook_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_hook_context_creation() {
        let context = HookContext::new("test-hook".to_string())
            .with_args(vec!["arg1".to_string(), "arg2".to_string()])
            .with_env("TEST_VAR".to_string(), "test_value".to_string());
        
        assert_eq!(context.name, "test-hook");
        assert_eq!(context.args, vec!["arg1", "arg2"]);
        assert_eq!(context.env.get("TEST_VAR"), Some(&"test_value".to_string()));
    }
    
    #[test]
    fn test_hook_creation() {
        let script_path = PathBuf::from("/tmp/test-hook.sh");
        let hook = Hook::new("test-hook".to_string(), script_path.clone())
            .with_description("Test hook".to_string())
            .with_timeout(60)
            .with_trigger("pre-commit".to_string());
        
        assert_eq!(hook.name, "test-hook");
        assert_eq!(hook.script, script_path);
        assert_eq!(hook.description, Some("Test hook".to_string()));
        assert_eq!(hook.timeout_seconds, Some(60));
        assert!(hook.triggers.contains(&"pre-commit".to_string()));
    }
    
    #[tokio::test]
    async fn test_hook_manager_init() {
        let temp_dir = TempDir::new().unwrap();
        let hooks_dir = temp_dir.path().to_path_buf();
        
        let mut manager = HookManager::new(hooks_dir.clone());
        manager.init().await.unwrap();
        
        assert!(hooks_dir.exists());
    }
}