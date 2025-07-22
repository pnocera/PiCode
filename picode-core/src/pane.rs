//! Pane management for PiCode workspace
//! 
//! Inspired by Zellij's pane system with AI-focused enhancements

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

/// Unique identifier for a pane
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaneId(pub Uuid);

impl PaneId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for PaneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Types of panes available in PiCode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaneType {
    /// Terminal pane for command execution
    Terminal {
        shell: String,
        working_dir: PathBuf,
    },
    /// LLM interaction pane
    LLMChat {
        provider: String,
        model: String,
        system_prompt: Option<String>,
    },
    /// File editor pane
    Editor {
        file_path: PathBuf,
        language: Option<String>,
    },
    /// Output/result display pane
    Output {
        content_type: String,
    },
    /// Plugin-based pane (for future extensibility)
    Plugin {
        plugin_name: String,
        config: HashMap<String, String>,
    },
}

/// Pane configuration and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pane {
    pub id: PaneId,
    pub pane_type: PaneType,
    pub title: String,
    pub is_active: bool,
    pub size: PaneSize,
    pub position: PanePosition,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Pane size information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaneSize {
    pub width: u16,
    pub height: u16,
    pub min_width: u16,
    pub min_height: u16,
}

impl Default for PaneSize {
    fn default() -> Self {
        Self {
            width: 80,
            height: 24,
            min_width: 10,
            min_height: 3,
        }
    }
}

/// Pane position in the workspace
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanePosition {
    pub x: u16,
    pub y: u16,
    pub z_index: u8,
}

impl Default for PanePosition {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            z_index: 0,
        }
    }
}

impl Pane {
    pub fn new_terminal(shell: String, working_dir: PathBuf, title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: PaneId::new(),
            pane_type: PaneType::Terminal { shell, working_dir },
            title,
            is_active: false,
            size: PaneSize::default(),
            position: PanePosition::default(),
            metadata: HashMap::new(),
            created_at: now,
            last_activity: now,
        }
    }
    
    pub fn new_llm_chat(provider: String, model: String, title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: PaneId::new(),
            pane_type: PaneType::LLMChat {
                provider,
                model,
                system_prompt: None,
            },
            title,
            is_active: false,
            size: PaneSize::default(),
            position: PanePosition::default(),
            metadata: HashMap::new(),
            created_at: now,
            last_activity: now,
        }
    }
    
    pub fn new_editor(file_path: PathBuf, title: String) -> Self {
        let now = chrono::Utc::now();
        let language = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_string());
        
        Self {
            id: PaneId::new(),
            pane_type: PaneType::Editor { file_path, language },
            title,
            is_active: false,
            size: PaneSize::default(),
            position: PanePosition::default(),
            metadata: HashMap::new(),
            created_at: now,
            last_activity: now,
        }
    }
    
    pub fn new_output(content_type: String, title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: PaneId::new(),
            pane_type: PaneType::Output { content_type },
            title,
            is_active: false,
            size: PaneSize::default(),
            position: PanePosition::default(),
            metadata: HashMap::new(),
            created_at: now,
            last_activity: now,
        }
    }
    
    pub fn activate(&mut self) {
        self.is_active = true;
        self.touch();
    }
    
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.touch();
    }
    
    pub fn touch(&mut self) {
        self.last_activity = chrono::Utc::now();
    }
    
    pub fn resize(&mut self, width: u16, height: u16) -> Result<(), PaneError> {
        if width < self.size.min_width || height < self.size.min_height {
            return Err(PaneError::InvalidSize {
                requested: (width, height),
                minimum: (self.size.min_width, self.size.min_height),
            });
        }
        
        self.size.width = width;
        self.size.height = height;
        self.touch();
        Ok(())
    }
    
    pub fn move_to(&mut self, x: u16, y: u16) {
        self.position.x = x;
        self.position.y = y;
        self.touch();
    }
    
    pub fn set_z_index(&mut self, z_index: u8) {
        self.position.z_index = z_index;
        self.touch();
    }
    
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.touch();
    }
    
    pub fn get_working_dir(&self) -> Option<PathBuf> {
        match &self.pane_type {
            PaneType::Terminal { working_dir, .. } => Some(working_dir.clone()),
            PaneType::Editor { file_path, .. } => file_path.parent().map(|p| p.to_path_buf()),
            _ => None,
        }
    }
    
    pub fn can_receive_input(&self) -> bool {
        matches!(
            self.pane_type,
            PaneType::Terminal { .. } | PaneType::LLMChat { .. } | PaneType::Editor { .. }
        )
    }
}

/// Pane-related errors
#[derive(Error, Debug)]
pub enum PaneError {
    #[error("Invalid pane size: requested ({requested:?}) is smaller than minimum ({minimum:?})")]
    InvalidSize {
        requested: (u16, u16),
        minimum: (u16, u16),
    },
    
    #[error("Pane not found: {0}")]
    NotFound(String),
    
    #[error("Invalid pane type for operation: {0}")]
    InvalidType(String),
    
    #[error("Pane is not active")]
    NotActive,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_id_generation() {
        let id1 = PaneId::new();
        let id2 = PaneId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn pane_creation_terminal() {
        let shell = "bash".to_string();
        let working_dir = PathBuf::from("/tmp");
        let title = "Terminal".to_string();
        
        let pane = Pane::new_terminal(shell.clone(), working_dir.clone(), title.clone());
        
        assert_eq!(pane.title, title);
        assert!(!pane.is_active);
        assert!(pane.can_receive_input());
        
        match pane.pane_type {
            PaneType::Terminal { shell: s, working_dir: wd } => {
                assert_eq!(s, shell);
                assert_eq!(wd, working_dir);
            }
            _ => panic!("Expected Terminal pane type"),
        }
    }

    #[test]
    fn pane_creation_llm_chat() {
        let provider = "openai".to_string();
        let model = "gpt-4".to_string();
        let title = "Chat".to_string();
        
        let pane = Pane::new_llm_chat(provider.clone(), model.clone(), title.clone());
        
        assert_eq!(pane.title, title);
        assert!(pane.can_receive_input());
        
        match pane.pane_type {
            PaneType::LLMChat { provider: p, model: m, .. } => {
                assert_eq!(p, provider);
                assert_eq!(m, model);
            }
            _ => panic!("Expected LLMChat pane type"),
        }
    }

    #[test]
    fn pane_creation_editor() {
        let file_path = PathBuf::from("test.rs");
        let title = "Editor".to_string();
        
        let pane = Pane::new_editor(file_path.clone(), title.clone());
        
        assert_eq!(pane.title, title);
        assert!(pane.can_receive_input());
        
        match pane.pane_type {
            PaneType::Editor { file_path: fp, language } => {
                assert_eq!(fp, file_path);
                assert_eq!(language, Some("rs".to_string()));
            }
            _ => panic!("Expected Editor pane type"),
        }
    }

    #[test]
    fn pane_resize_validation() {
        let mut pane = Pane::new_terminal(
            "bash".to_string(),
            PathBuf::from("/tmp"),
            "Test".to_string(),
        );
        
        // Valid resize
        assert!(pane.resize(100, 50).is_ok());
        assert_eq!(pane.size.width, 100);
        assert_eq!(pane.size.height, 50);
        
        // Invalid resize (too small)
        assert!(pane.resize(5, 2).is_err());
        
        // Size should remain unchanged after failed resize
        assert_eq!(pane.size.width, 100);
        assert_eq!(pane.size.height, 50);
    }

    #[test]
    fn pane_activation() {
        let mut pane = Pane::new_terminal(
            "bash".to_string(),
            PathBuf::from("/tmp"),
            "Test".to_string(),
        );
        
        assert!(!pane.is_active);
        
        pane.activate();
        assert!(pane.is_active);
        
        pane.deactivate();
        assert!(!pane.is_active);
    }

    #[test]
    fn pane_positioning() {
        let mut pane = Pane::new_terminal(
            "bash".to_string(),
            PathBuf::from("/tmp"),
            "Test".to_string(),
        );
        
        assert_eq!(pane.position.x, 0);
        assert_eq!(pane.position.y, 0);
        assert_eq!(pane.position.z_index, 0);
        
        pane.move_to(10, 20);
        assert_eq!(pane.position.x, 10);
        assert_eq!(pane.position.y, 20);
        
        pane.set_z_index(5);
        assert_eq!(pane.position.z_index, 5);
    }

    #[test]
    fn pane_working_directory() {
        let working_dir = PathBuf::from("/tmp/test");
        let pane = Pane::new_terminal(
            "bash".to_string(),
            working_dir.clone(),
            "Test".to_string(),
        );
        
        assert_eq!(pane.get_working_dir(), Some(&working_dir));
        
        let file_path = PathBuf::from("/home/user/project/main.rs");
        let editor_pane = Pane::new_editor(file_path, "Editor".to_string());
        
        // Editor pane working dir should be the parent directory of the file
        let expected_dir = PathBuf::from("/home/user/project");
        let actual_dir = editor_pane.get_working_dir().unwrap();
        assert_eq!(actual_dir.file_name(), expected_dir.file_name());
    }
}