//! Hook registry for managing hook storage and retrieval
//!
//! This module provides a centralized registry for managing hooks, including
//! registration, lookup, and trigger-based filtering.

use crate::hooks::{Hook, HookError, HookResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, warn};

/// Hook registry for managing hooks
#[derive(Debug, Default)]
pub struct HookRegistry {
    /// Hooks indexed by name
    hooks: HashMap<String, Hook>,
    
    /// Trigger to hook name mappings
    triggers: HashMap<String, Vec<String>>,
}

impl HookRegistry {
    /// Create a new empty hook registry
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            triggers: HashMap::new(),
        }
    }
    
    /// Register a hook in the registry
    pub fn register(&mut self, hook: Hook) -> HookResult<()> {
        let name = hook.name.clone();
        
        // Check if hook already exists
        if self.hooks.contains_key(&name) {
            debug!("Replacing existing hook: {}", name);
        }
        
        // Update trigger mappings
        for trigger in &hook.triggers {
            self.triggers
                .entry(trigger.clone())
                .or_insert_with(Vec::new)
                .push(name.clone());
        }
        
        // Store the hook
        self.hooks.insert(name.clone(), hook);
        
        debug!("Registered hook: {}", name);
        Ok(())
    }
    
    /// Unregister a hook from the registry
    pub fn unregister(&mut self, name: &str) -> HookResult<()> {
        if let Some(hook) = self.hooks.remove(name) {
            // Remove from trigger mappings
            for trigger in &hook.triggers {
                if let Some(hook_names) = self.triggers.get_mut(trigger) {
                    hook_names.retain(|h| h != name);
                    if hook_names.is_empty() {
                        self.triggers.remove(trigger);
                    }
                }
            }
            
            debug!("Unregistered hook: {}", name);
            Ok(())
        } else {
            Err(HookError::NotFound(name.to_string()))
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
    
    /// List all hooks
    pub fn list(&self) -> Vec<&Hook> {
        self.hooks.values().collect()
    }
    
    /// List hook names
    pub fn list_names(&self) -> Vec<&String> {
        self.hooks.keys().collect()
    }
    
    /// List hooks for a specific trigger
    pub fn list_for_trigger(&self, trigger: &str) -> Vec<&Hook> {
        if let Some(hook_names) = self.triggers.get(trigger) {
            hook_names
                .iter()
                .filter_map(|name| self.hooks.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// List all triggers
    pub fn list_triggers(&self) -> Vec<&String> {
        self.triggers.keys().collect()
    }
    
    /// Check if a hook exists
    pub fn exists(&self, name: &str) -> bool {
        self.hooks.contains_key(name)
    }
    
    /// Get the number of registered hooks
    pub fn count(&self) -> usize {
        self.hooks.len()
    }
    
    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.hooks.is_empty()
    }
    
    /// Clear all hooks from the registry
    pub fn clear(&mut self) {
        self.hooks.clear();
        self.triggers.clear();
        debug!("Cleared hook registry");
    }
    
    /// Enable a hook
    pub fn enable_hook(&mut self, name: &str) -> HookResult<()> {
        if let Some(hook) = self.hooks.get_mut(name) {
            hook.enabled = true;
            debug!("Enabled hook: {}", name);
            Ok(())
        } else {
            Err(HookError::NotFound(name.to_string()))
        }
    }
    
    /// Disable a hook
    pub fn disable_hook(&mut self, name: &str) -> HookResult<()> {
        if let Some(hook) = self.hooks.get_mut(name) {
            hook.enabled = false;
            debug!("Disabled hook: {}", name);
            Ok(())
        } else {
            Err(HookError::NotFound(name.to_string()))
        }
    }
    
    /// Get enabled hooks for a trigger
    pub fn list_enabled_for_trigger(&self, trigger: &str) -> Vec<&Hook> {
        self.list_for_trigger(trigger)
            .into_iter()
            .filter(|hook| hook.enabled)
            .collect()
    }
    
    /// Get all enabled hooks
    pub fn list_enabled(&self) -> Vec<&Hook> {
        self.hooks
            .values()
            .filter(|hook| hook.enabled)
            .collect()
    }
    
    /// Get all disabled hooks
    pub fn list_disabled(&self) -> Vec<&Hook> {
        self.hooks
            .values()
            .filter(|hook| !hook.enabled)
            .collect()
    }
    
    /// Update hook configuration
    pub fn update_hook(&mut self, name: &str, mut hook: Hook) -> HookResult<()> {
        // Ensure the name matches
        hook.name = name.to_string();
        
        // Remove the old hook and re-register
        if self.exists(name) {
            self.unregister(name)?;
        }
        
        self.register(hook)?;
        Ok(())
    }
    
    /// Export registry to serializable format
    pub fn export(&self) -> HookRegistryExport {
        HookRegistryExport {
            hooks: self.hooks.values().cloned().collect(),
        }
    }
    
    /// Import hooks from serialized format
    pub fn import(&mut self, export: HookRegistryExport) -> HookResult<()> {
        for hook in export.hooks {
            if let Err(e) = self.register(hook) {
                warn!("Failed to import hook: {}", e);
            }
        }
        Ok(())
    }
    
    /// Get registry statistics
    pub fn stats(&self) -> HookRegistryStats {
        let total_hooks = self.count();
        let enabled_hooks = self.list_enabled().len();
        let disabled_hooks = self.list_disabled().len();
        let total_triggers = self.triggers.len();
        
        HookRegistryStats {
            total_hooks,
            enabled_hooks,
            disabled_hooks,
            total_triggers,
            triggers: self.triggers.clone(),
        }
    }
}

/// Serializable hook registry export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookRegistryExport {
    /// List of hooks
    pub hooks: Vec<Hook>,
}

/// Hook registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookRegistryStats {
    /// Total number of hooks
    pub total_hooks: usize,
    
    /// Number of enabled hooks
    pub enabled_hooks: usize,
    
    /// Number of disabled hooks
    pub disabled_hooks: usize,
    
    /// Total number of triggers
    pub total_triggers: usize,
    
    /// Trigger to hook mappings
    pub triggers: HashMap<String, Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    fn create_test_hook(name: &str, triggers: Vec<&str>) -> Hook {
        let mut hook = Hook::new(name.to_string(), PathBuf::from("/tmp/test.sh"));
        for trigger in triggers {
            hook = hook.with_trigger(trigger.to_string());
        }
        hook
    }
    
    #[test]
    fn test_registry_basic_operations() {
        let mut registry = HookRegistry::new();
        
        assert!(registry.is_empty());
        assert_eq!(registry.count(), 0);
        
        let hook = create_test_hook("test-hook", vec!["pre-commit"]);
        registry.register(hook).unwrap();
        
        assert!(!registry.is_empty());
        assert_eq!(registry.count(), 1);
        assert!(registry.exists("test-hook"));
        assert!(registry.get("test-hook").is_some());
    }
    
    #[test]
    fn test_registry_trigger_mappings() {
        let mut registry = HookRegistry::new();
        
        let hook1 = create_test_hook("hook1", vec!["pre-commit", "post-commit"]);
        let hook2 = create_test_hook("hook2", vec!["pre-commit"]);
        
        registry.register(hook1).unwrap();
        registry.register(hook2).unwrap();
        
        let pre_commit_hooks = registry.list_for_trigger("pre-commit");
        assert_eq!(pre_commit_hooks.len(), 2);
        
        let post_commit_hooks = registry.list_for_trigger("post-commit");
        assert_eq!(post_commit_hooks.len(), 1);
        
        let nonexistent_hooks = registry.list_for_trigger("nonexistent");
        assert_eq!(nonexistent_hooks.len(), 0);
    }
    
    #[test]
    fn test_registry_unregister() {
        let mut registry = HookRegistry::new();
        
        let hook = create_test_hook("test-hook", vec!["pre-commit"]);
        registry.register(hook).unwrap();
        
        assert!(registry.exists("test-hook"));
        assert_eq!(registry.list_for_trigger("pre-commit").len(), 1);
        
        registry.unregister("test-hook").unwrap();
        
        assert!(!registry.exists("test-hook"));
        assert_eq!(registry.list_for_trigger("pre-commit").len(), 0);
        
        // Unregistering non-existent hook should fail
        assert!(registry.unregister("nonexistent").is_err());
    }
    
    #[test]
    fn test_registry_enable_disable() {
        let mut registry = HookRegistry::new();
        
        let hook = create_test_hook("test-hook", vec!["pre-commit"]);
        registry.register(hook).unwrap();
        
        // Hook should be enabled by default
        let hook = registry.get("test-hook").unwrap();
        assert!(hook.enabled);
        
        registry.disable_hook("test-hook").unwrap();
        let hook = registry.get("test-hook").unwrap();
        assert!(!hook.enabled);
        
        registry.enable_hook("test-hook").unwrap();
        let hook = registry.get("test-hook").unwrap();
        assert!(hook.enabled);
    }
    
    #[test]
    fn test_registry_stats() {
        let mut registry = HookRegistry::new();
        
        let hook1 = create_test_hook("hook1", vec!["pre-commit"]);
        let hook2 = create_test_hook("hook2", vec!["post-commit"]);
        
        registry.register(hook1).unwrap();
        registry.register(hook2).unwrap();
        registry.disable_hook("hook2").unwrap();
        
        let stats = registry.stats();
        
        assert_eq!(stats.total_hooks, 2);
        assert_eq!(stats.enabled_hooks, 1);
        assert_eq!(stats.disabled_hooks, 1);
        assert_eq!(stats.total_triggers, 2);
    }
    
    #[test]
    fn test_registry_export_import() {
        let mut registry = HookRegistry::new();
        
        let hook1 = create_test_hook("hook1", vec!["pre-commit"]);
        let hook2 = create_test_hook("hook2", vec!["post-commit"]);
        
        registry.register(hook1).unwrap();
        registry.register(hook2).unwrap();
        
        let export = registry.export();
        assert_eq!(export.hooks.len(), 2);
        
        let mut new_registry = HookRegistry::new();
        new_registry.import(export).unwrap();
        
        assert_eq!(new_registry.count(), 2);
        assert!(new_registry.exists("hook1"));
        assert!(new_registry.exists("hook2"));
    }
}