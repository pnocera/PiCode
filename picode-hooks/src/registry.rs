//! Hook registry for managing installed hooks

use crate::{Hook, HookResult, HooksError};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info};

/// Registry for managing hooks
#[derive(Debug)]
pub struct HookRegistry {
    /// Map of hook name to hook
    hooks: HashMap<String, Hook>,
}

impl HookRegistry {
    /// Create a new hook registry
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    /// Register a hook
    pub fn register(&mut self, hook: Hook) -> HookResult<()> {
        debug!("Registering hook: {}", hook.name);
        self.hooks.insert(hook.name.clone(), hook);
        Ok(())
    }

    /// Unregister a hook
    pub fn unregister(&mut self, name: &str) -> HookResult<()> {
        debug!("Unregistering hook: {}", name);
        match self.hooks.remove(name) {
            Some(_) => Ok(()),
            None => Err(HooksError::HookNotFound(name.to_string())),
        }
    }

    /// Get a hook by name
    pub fn get(&self, name: &str) -> Option<&Hook> {
        self.hooks.get(name)
    }

    /// Get a mutable reference to a hook by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Hook> {
        self.hooks.get_mut(name)
    }

    /// Check if a hook exists
    pub fn has_hook(&self, name: &str) -> bool {
        self.hooks.contains_key(name)
    }

    /// List all hook names
    pub fn list_hooks(&self) -> Vec<String> {
        self.hooks.keys().cloned().collect()
    }

    /// Get all hooks
    pub fn list(&self) -> Vec<&Hook> {
        self.hooks.values().collect()
    }

    /// Get the number of registered hooks
    pub fn count(&self) -> usize {
        self.hooks.len()
    }

    /// Clear all hooks
    pub fn clear(&mut self) {
        self.hooks.clear();
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.hooks.is_empty()
    }

    /// Load hooks from a directory (simplified implementation)
    pub fn load_from_dir(hooks_dir: PathBuf) -> HookResult<Self> {
        let mut registry = Self::new();

        if !hooks_dir.exists() {
            info!("Hooks directory does not exist: {}", hooks_dir.display());
            return Ok(registry);
        }

        let entries = std::fs::read_dir(&hooks_dir)
            .map_err(HooksError::IoError)?;

        for entry in entries {
            let entry = entry.map_err(HooksError::IoError)?;
            let path = entry.path();

            if path.is_file() {
                // Check for executable files (on Unix) or .bat/.cmd files (on Windows)
                let is_executable = {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        path.metadata()
                            .map(|m| m.permissions().mode() & 0o111 != 0)
                            .unwrap_or(false)
                    }
                    #[cfg(windows)]
                    {
                        path.extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| matches!(ext.to_lowercase().as_str(), "bat" | "cmd" | "exe"))
                            .unwrap_or(false)
                    }
                };

                if is_executable {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        let hook = Hook::new(name.to_string(), path.clone());
                        registry.register(hook)?;
                        debug!("Loaded hook from file: {}", name);
                    }
                }
            }
        }

        info!("Loaded {} hooks from {}", registry.count(), hooks_dir.display());
        Ok(registry)
    }
}

impl Default for HookRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_registry_basic_operations() {
        let mut registry = HookRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.count(), 0);

        let hook = Hook::new("test_hook".to_string(), PathBuf::from("/path/to/script"));
        registry.register(hook).unwrap();

        assert!(!registry.is_empty());
        assert_eq!(registry.count(), 1);
        assert!(registry.has_hook("test_hook"));
        assert!(registry.get("test_hook").is_some());

        registry.unregister("test_hook").unwrap();
        assert!(registry.is_empty());
        assert!(!registry.has_hook("test_hook"));
    }

    #[test]
    fn test_registry_unregister_nonexistent() {
        let mut registry = HookRegistry::new();
        let result = registry.unregister("nonexistent");
        assert!(result.is_err());
        
        match result {
            Err(HooksError::HookNotFound(name)) => assert_eq!(name, "nonexistent"),
            _ => panic!("Expected HookNotFound error"),
        }
    }

    #[test]
    fn test_registry_list_hooks() {
        let mut registry = HookRegistry::new();
        
        registry.register(Hook::new("hook1".to_string(), PathBuf::from("/path/1"))).unwrap();
        registry.register(Hook::new("hook2".to_string(), PathBuf::from("/path/2"))).unwrap();

        let hooks = registry.list_hooks();
        assert_eq!(hooks.len(), 2);
        assert!(hooks.contains(&"hook1".to_string()));
        assert!(hooks.contains(&"hook2".to_string()));
    }

    #[test]
    fn test_load_from_nonexistent_dir() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_dir = temp_dir.path().join("nonexistent");
        
        let registry = HookRegistry::load_from_dir(nonexistent_dir).unwrap();
        assert!(registry.is_empty());
    }
}