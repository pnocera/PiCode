//! PiCode Hooks - Configurable hook system
//!
//! This crate provides a flexible hook system that allows users to extend PiCode
//! with custom scripts and automation at various execution points.

pub mod hooks;
pub mod registry;

pub use hooks::*;
pub use registry::*;

use std::path::PathBuf;
use thiserror::Error;

/// Result type for hook operations
pub type HookResult<T> = Result<T, HooksError>;

/// Errors that can occur in the hook system
#[derive(Error, Debug)]
pub enum HooksError {
    #[error("Hook '{0}' not found")]
    HookNotFound(String),

    #[error("Script not found at path: {0}")]
    ScriptNotFound(PathBuf),

    #[error("Hook '{0}' execution failed: {1}")]
    ExecutionFailed(String, String),

    #[error("Registry error: {0}")]
    RegistryError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

}

/// Command types that hooks can handle (matches CLI structure)
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

/// Main function to handle hook commands (required by main.rs)
pub async fn handle_command(command: HooksCommand) -> Result<(), crate::HooksError> {
    let mut manager = HookManager::new();

    match command {
        HooksCommand::List => {
            println!("🪝 Available Hooks:");
            let hooks = manager.list_hooks();
            if hooks.is_empty() {
                println!("  No hooks registered");
            } else {
                for hook_name in hooks {
                    println!("  • {}", hook_name);
                }
            }
            Ok(())
        }
        HooksCommand::Install { name } => {
            println!("📦 Installing hook: {}", name);
            // TODO: Implement hook installation from repository or local script
            println!("Hook installation not yet implemented");
            Ok(())
        }
        HooksCommand::Remove { name } => {
            println!("🗑️  Removing hook: {}", name);
            match manager.unregister_hook(&name) {
                Ok(_) => {
                    println!("✅ Hook '{}' removed successfully", name);
                    Ok(())
                }
                Err(e) => {
                    println!("❌ Failed to remove hook '{}': {}", name, e);
                    Err(e)
                }
            }
        }
        HooksCommand::Run { name, args } => {
            println!("⚡ Running hook '{}' with args: {:?}", name, args);
            match manager.execute_hook(&name, args).await {
                Ok(output) => {
                    if !output.trim().is_empty() {
                        println!("Hook output:\n{}", output);
                    }
                    println!("✅ Hook '{}' completed successfully", name);
                    Ok(())
                }
                Err(e) => {
                    println!("❌ Hook '{}' failed: {}", name, e);
                    Err(e)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hooks_error_display() {
        let err = HooksError::HookNotFound("test_hook".to_string());
        assert_eq!(err.to_string(), "Hook 'test_hook' not found");
    }

    #[tokio::test]
    async fn test_handle_list_command() {
        let result = handle_command(HooksCommand::List).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_remove_nonexistent() {
        let result = handle_command(HooksCommand::Remove { 
            name: "nonexistent".to_string() 
        }).await;
        assert!(result.is_err());
    }
}