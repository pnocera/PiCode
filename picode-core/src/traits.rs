//! Common traits for PiCode core components

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{CoreError, Result};

/// Trait for components that can be serialized and persisted
pub trait Persistent {
    /// Save the component to persistent storage
    async fn save(&self) -> Result<()>;
    
    /// Load the component from persistent storage
    async fn load(&mut self) -> Result<()>;
    
    /// Check if the component exists in persistent storage
    async fn exists(&self) -> bool;
    
    /// Delete the component from persistent storage
    async fn delete(&self) -> Result<()>;
}

/// Trait for components that can be configured
pub trait Configurable {
    type Config: Serialize + for<'de> Deserialize<'de>;
    
    /// Apply configuration to the component
    fn configure(&mut self, config: Self::Config) -> Result<()>;
    
    /// Get current configuration
    fn get_config(&self) -> Self::Config;
    
    /// Validate configuration
    fn validate_config(config: &Self::Config) -> Result<()>;
}

/// Trait for components that can be activated/deactivated
pub trait Activatable {
    /// Activate the component
    fn activate(&mut self);
    
    /// Deactivate the component
    fn deactivate(&mut self);
    
    /// Check if the component is active
    fn is_active(&self) -> bool;
}

/// Trait for components that have metadata
pub trait Metadata {
    /// Set metadata value
    fn set_metadata(&mut self, key: String, value: String);
    
    /// Get metadata value
    fn get_metadata(&self, key: &str) -> Option<&String>;
    
    /// Remove metadata value
    fn remove_metadata(&mut self, key: &str) -> Option<String>;
    
    /// Get all metadata
    fn get_all_metadata(&self) -> &HashMap<String, String>;
    
    /// Clear all metadata
    fn clear_metadata(&mut self);
}

/// Trait for components that can be resized
pub trait Resizable {
    /// Resize the component
    fn resize(&mut self, width: u16, height: u16) -> Result<()>;
    
    /// Get current size
    fn size(&self) -> (u16, u16);
    
    /// Get minimum size
    fn min_size(&self) -> (u16, u16);
    
    /// Check if a size is valid
    fn is_valid_size(&self, width: u16, height: u16) -> bool {
        let (min_width, min_height) = self.min_size();
        width >= min_width && height >= min_height
    }
}

/// Trait for components that can be positioned
pub trait Positionable {
    /// Move the component to a new position
    fn move_to(&mut self, x: u16, y: u16);
    
    /// Get current position
    fn position(&self) -> (u16, u16);
    
    /// Set Z-index for layering
    fn set_z_index(&mut self, z: u8);
    
    /// Get Z-index
    fn z_index(&self) -> u8;
}

/// Trait for components that can receive input
#[async_trait]
pub trait InputReceiver {
    /// Handle input data
    async fn handle_input(&mut self, input: &str) -> Result<()>;
    
    /// Check if the component can receive input
    fn can_receive_input(&self) -> bool;
    
    /// Get input prompt (if applicable)
    fn input_prompt(&self) -> Option<String> {
        None
    }
}

/// Trait for components that can produce output
pub trait OutputProducer {
    /// Get current output
    fn get_output(&self) -> Option<String>;
    
    /// Check if there's new output since last check
    fn has_new_output(&self) -> bool;
    
    /// Mark output as read
    fn mark_output_read(&mut self);
    
    /// Clear output buffer
    fn clear_output(&mut self);
}

/// Trait for components that can be rendered
pub trait Renderable {
    /// Render the component to a string representation
    fn render(&self) -> String;
    
    /// Render with specific dimensions
    fn render_with_size(&self, width: u16, height: u16) -> String;
    
    /// Check if the component needs re-rendering
    fn needs_render(&self) -> bool;
    
    /// Mark as rendered
    fn mark_rendered(&mut self);
}

/// Trait for components with lifecycle management
#[async_trait]
pub trait Lifecycle {
    /// Initialize the component
    async fn initialize(&mut self) -> Result<()>;
    
    /// Start the component
    async fn start(&mut self) -> Result<()>;
    
    /// Stop the component
    async fn stop(&mut self) -> Result<()>;
    
    /// Shutdown and cleanup the component
    async fn shutdown(&mut self) -> Result<()>;
    
    /// Get current lifecycle state
    fn state(&self) -> LifecycleState;
}

/// Lifecycle states for components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LifecycleState {
    Uninitialized,
    Initialized,
    Starting,
    Running,
    Stopping,
    Stopped,
    Error(String),
}

/// Trait for components that can be validated
pub trait Validatable {
    /// Validate the component's current state
    fn validate(&self) -> Result<()>;
    
    /// Get validation errors (if any)
    fn validation_errors(&self) -> Vec<String>;
    
    /// Check if the component is valid
    fn is_valid(&self) -> bool {
        self.validation_errors().is_empty()
    }
}

/// Trait for components that support plugins or extensions
pub trait Extensible {
    type Extension;
    
    /// Add an extension
    fn add_extension(&mut self, name: String, extension: Self::Extension) -> Result<()>;
    
    /// Remove an extension
    fn remove_extension(&mut self, name: &str) -> Option<Self::Extension>;
    
    /// Get an extension
    fn get_extension(&self, name: &str) -> Option<&Self::Extension>;
    
    /// List all extensions
    fn list_extensions(&self) -> Vec<&String>;
}

/// Trait for components that can be cloned or duplicated
pub trait Duplicatable {
    /// Create a duplicate of the component
    fn duplicate(&self) -> Result<Self>
    where
        Self: Sized;
    
    /// Create a duplicate with a new ID
    fn duplicate_with_new_id(&self) -> Result<Self>
    where
        Self: Sized;
}

/// Trait for components that can be compared for changes
pub trait Trackable {
    /// Check if the component has been modified
    fn is_modified(&self) -> bool;
    
    /// Mark the component as modified
    fn mark_modified(&mut self);
    
    /// Mark the component as clean (not modified)
    fn mark_clean(&mut self);
    
    /// Get the last modification time
    fn last_modified(&self) -> chrono::DateTime<chrono::Utc>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_state_equality() {
        assert_eq!(LifecycleState::Uninitialized, LifecycleState::Uninitialized);
        assert_ne!(LifecycleState::Running, LifecycleState::Stopped);
        
        let error1 = LifecycleState::Error("test".to_string());
        let error2 = LifecycleState::Error("test".to_string());
        let error3 = LifecycleState::Error("different".to_string());
        
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    // Mock implementation for testing traits
    #[derive(Default)]
    struct MockComponent {
        active: bool,
        metadata: HashMap<String, String>,
        width: u16,
        height: u16,
        x: u16,
        y: u16,
        z: u8,
        modified: bool,
        last_modified: chrono::DateTime<chrono::Utc>,
    }

    impl Activatable for MockComponent {
        fn activate(&mut self) {
            self.active = true;
        }

        fn deactivate(&mut self) {
            self.active = false;
        }

        fn is_active(&self) -> bool {
            self.active
        }
    }

    impl Metadata for MockComponent {
        fn set_metadata(&mut self, key: String, value: String) {
            self.metadata.insert(key, value);
        }

        fn get_metadata(&self, key: &str) -> Option<&String> {
            self.metadata.get(key)
        }

        fn remove_metadata(&mut self, key: &str) -> Option<String> {
            self.metadata.remove(key)
        }

        fn get_all_metadata(&self) -> &HashMap<String, String> {
            &self.metadata
        }

        fn clear_metadata(&mut self) {
            self.metadata.clear();
        }
    }

    impl Resizable for MockComponent {
        fn resize(&mut self, width: u16, height: u16) -> Result<()> {
            if !self.is_valid_size(width, height) {
                return Err(CoreError::Pane(super::super::pane::PaneError::InvalidSize {
                    requested: (width, height),
                    minimum: self.min_size(),
                }));
            }
            self.width = width;
            self.height = height;
            Ok(())
        }

        fn size(&self) -> (u16, u16) {
            (self.width, self.height)
        }

        fn min_size(&self) -> (u16, u16) {
            (10, 3)
        }
    }

    impl Positionable for MockComponent {
        fn move_to(&mut self, x: u16, y: u16) {
            self.x = x;
            self.y = y;
        }

        fn position(&self) -> (u16, u16) {
            (self.x, self.y)
        }

        fn set_z_index(&mut self, z: u8) {
            self.z = z;
        }

        fn z_index(&self) -> u8 {
            self.z
        }
    }

    impl Trackable for MockComponent {
        fn is_modified(&self) -> bool {
            self.modified
        }

        fn mark_modified(&mut self) {
            self.modified = true;
            self.last_modified = chrono::Utc::now();
        }

        fn mark_clean(&mut self) {
            self.modified = false;
        }

        fn last_modified(&self) -> chrono::DateTime<chrono::Utc> {
            self.last_modified
        }
    }

    #[test]
    fn activatable_trait() {
        let mut component = MockComponent::default();
        assert!(!component.is_active());
        
        component.activate();
        assert!(component.is_active());
        
        component.deactivate();
        assert!(!component.is_active());
    }

    #[test]
    fn metadata_trait() {
        let mut component = MockComponent::default();
        
        component.set_metadata("key1".to_string(), "value1".to_string());
        component.set_metadata("key2".to_string(), "value2".to_string());
        
        assert_eq!(component.get_metadata("key1"), Some(&"value1".to_string()));
        assert_eq!(component.get_metadata("key2"), Some(&"value2".to_string()));
        assert_eq!(component.get_metadata("key3"), None);
        
        assert_eq!(component.get_all_metadata().len(), 2);
        
        let removed = component.remove_metadata("key1");
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(component.get_metadata("key1"), None);
        
        component.clear_metadata();
        assert_eq!(component.get_all_metadata().len(), 0);
    }

    #[test]
    fn resizable_trait() {
        let mut component = MockComponent::default();
        
        // Valid resize
        assert!(component.resize(80, 24).is_ok());
        assert_eq!(component.size(), (80, 24));
        
        // Invalid resize (too small)
        assert!(component.resize(5, 2).is_err());
        assert_eq!(component.size(), (80, 24)); // Should remain unchanged
        
        // Check min size validation
        assert!(component.is_valid_size(10, 3));
        assert!(!component.is_valid_size(9, 3));
        assert!(!component.is_valid_size(10, 2));
    }

    #[test]
    fn positionable_trait() {
        let mut component = MockComponent::default();
        
        assert_eq!(component.position(), (0, 0));
        assert_eq!(component.z_index(), 0);
        
        component.move_to(10, 20);
        assert_eq!(component.position(), (10, 20));
        
        component.set_z_index(5);
        assert_eq!(component.z_index(), 5);
    }

    #[test]
    fn trackable_trait() {
        let mut component = MockComponent::default();
        
        assert!(!component.is_modified());
        
        component.mark_modified();
        assert!(component.is_modified());
        
        let modified_time = component.last_modified();
        assert!(modified_time > chrono::Utc::now() - chrono::Duration::seconds(1));
        
        component.mark_clean();
        assert!(!component.is_modified());
        
        // Last modified time should remain the same
        assert_eq!(component.last_modified(), modified_time);
    }
}