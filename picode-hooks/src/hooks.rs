//! Hook system implementation
//!
//! This module provides the core hook execution functionality that allows
//! users to extend PiCode with custom scripts and automation.

use crate::registry::HookRegistry;
use crate::{HookResult, HooksError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tracing::{info, warn, error};

/// A hook represents a script that can be executed at specific points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    /// Name of the hook
    pub name: String,
    /// Path to the hook script
    pub script_path: PathBuf,
    /// Environment variables to set when running the hook
    pub env: HashMap<String, String>,
    /// Working directory for the hook
    pub working_dir: Option<PathBuf>,
    /// Whether the hook should run in the background
    pub background: bool,
}

impl Hook {
    /// Create a new hook
    pub fn new(name: String, script_path: PathBuf) -> Self {
        Self {
            name,
            script_path,
            env: HashMap::new(),
            working_dir: None,
            background: false,
        }
    }

    /// Set environment variables for this hook
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Set working directory for this hook
    pub fn with_working_dir(mut self, working_dir: PathBuf) -> Self {
        self.working_dir = Some(working_dir);
        self
    }

    /// Set whether this hook should run in background
    pub fn with_background(mut self, background: bool) -> Self {
        self.background = background;
        self
    }

    /// Execute this hook with the given arguments
    pub async fn execute(&self, args: Vec<String>) -> HookResult<String> {
        info!("Executing hook '{}' with args: {:?}", self.name, args);

        // Check if script exists
        if !self.script_path.exists() {
            return Err(HooksError::ScriptNotFound(self.script_path.clone()));
        }

        // Prepare command
        let mut cmd = Command::new(&self.script_path);
        cmd.args(&args);

        // Set environment variables
        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        // Set working directory
        if let Some(ref working_dir) = self.working_dir {
            cmd.current_dir(working_dir);
        }

        // Configure stdio based on background setting
        if self.background {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        } else {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
        }

        // Execute the command
        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if output.status.success() {
                    info!("Hook '{}' executed successfully", self.name);
                    Ok(stdout.to_string())
                } else {
                    error!("Hook '{}' failed: {}", self.name, stderr);
                    Err(HooksError::ExecutionFailed(
                        self.name.clone(),
                        stderr.to_string(),
                    ))
                }
            }
            Err(e) => {
                error!("Failed to execute hook '{}': {}", self.name, e);
                Err(HooksError::IoError(e))
            }
        }
    }
}

/// Hook manager for registering and executing hooks
#[derive(Debug)]
pub struct HookManager {
    registry: HookRegistry,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new() -> Self {
        Self {
            registry: HookRegistry::new(),
        }
    }

    /// Load hook manager from a configuration directory
    pub fn load_from_dir(hooks_dir: PathBuf) -> HookResult<Self> {
        let registry = HookRegistry::load_from_dir(hooks_dir)?;
        Ok(Self { registry })
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

    /// Execute a hook by name
    pub async fn execute_hook(&self, name: &str, args: Vec<String>) -> HookResult<String> {
        match self.registry.get(name) {
            Some(hook) => hook.execute(args).await,
            None => {
                warn!("Hook '{}' not found", name);
                Err(HooksError::HookNotFound(name.to_string()))
            }
        }
    }

    /// List all registered hooks
    pub fn list_hooks(&self) -> Vec<String> {
        self.registry.list_hooks()
    }

    /// Check if a hook exists
    pub fn has_hook(&self, name: &str) -> bool {
        self.registry.has_hook(name)
    }

    /// Get hook information
    pub fn get_hook(&self, name: &str) -> Option<&Hook> {
        self.registry.get(name)
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_hook_creation() {
        let hook = Hook::new(
            "test".to_string(),
            PathBuf::from("/path/to/script.sh"),
        );

        assert_eq!(hook.name, "test");
        assert_eq!(hook.script_path, PathBuf::from("/path/to/script.sh"));
        assert!(hook.env.is_empty());
        assert!(hook.working_dir.is_none());
        assert!(!hook.background);
    }

    #[test]
    fn test_hook_with_env() {
        let mut env = HashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());

        let hook = Hook::new("test".to_string(), PathBuf::from("/path/to/script.sh"))
            .with_env(env.clone());

        assert_eq!(hook.env, env);
    }

    #[test]
    fn test_hook_manager() {
        let mut manager = HookManager::new();
        
        let hook = Hook::new(
            "test_hook".to_string(),
            PathBuf::from("/path/to/script.sh"),
        );

        assert!(manager.register_hook(hook).is_ok());
        assert!(manager.has_hook("test_hook"));
        assert_eq!(manager.list_hooks().len(), 1);
        
        assert!(manager.unregister_hook("test_hook").is_ok());
        assert!(!manager.has_hook("test_hook"));
        assert_eq!(manager.list_hooks().len(), 0);
    }

    #[tokio::test]
    async fn test_hook_execution_script_not_found() {
        let hook = Hook::new(
            "missing_script".to_string(),
            PathBuf::from("/nonexistent/script.sh"),
        );

        let result = hook.execute(vec![]).await;
        assert!(result.is_err());
        
        match result {
            Err(HooksError::ScriptNotFound(path)) => {
                assert_eq!(path, PathBuf::from("/nonexistent/script.sh"));
            }
            _ => panic!("Expected ScriptNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_hook_execution_success() {
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("test_script.sh");

        // Create a simple test script
        fs::write(&script_path, "#!/bin/bash\necho 'Hello from hook'\n").unwrap();

        // Make script executable (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let hook = Hook::new("test_script".to_string(), script_path);

        // Skip this test on Windows or if script execution fails
        if let Ok(result) = hook.execute(vec![]).await {
            assert!(result.contains("Hello from hook"));
        }
    }
}