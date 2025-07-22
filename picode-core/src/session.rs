//! Session management for PiCode
//! 
//! Inspired by Zellij's session architecture with AI-focused enhancements

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Unique identifier for a session
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_name(name: &str) -> Self {
        Self(Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes()))
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Session configuration and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub name: String,
    pub workspace_path: PathBuf,
    pub llm_provider: String,
    pub model: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub panes: Vec<super::PaneId>,
    pub active_pane: Option<super::PaneId>,
    pub metadata: HashMap<String, String>,
}

impl Session {
    pub fn new(name: String, workspace_path: PathBuf) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: SessionId::new(),
            name,
            workspace_path,
            llm_provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            created_at: now,
            last_active: now,
            panes: Vec::new(),
            active_pane: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_llm_config(mut self, provider: String, model: String) -> Self {
        self.llm_provider = provider;
        self.model = model;
        self
    }
    
    pub fn add_pane(&mut self, pane_id: super::PaneId) {
        self.panes.push(pane_id.clone());
        if self.active_pane.is_none() {
            self.active_pane = Some(pane_id.clone());
        }
        self.touch();
    }
    
    pub fn remove_pane(&mut self, pane_id: &super::PaneId) {
        self.panes.retain(|p| p != pane_id);
        if self.active_pane.as_ref() == Some(pane_id) {
            self.active_pane = self.panes.first().cloned();
        }
        self.touch();
    }
    
    pub fn touch(&mut self) {
        self.last_active = chrono::Utc::now();
    }
    
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.touch();
    }
}

/// Session management errors
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Session not found: {0}")]
    NotFound(String),
    
    #[error("Session already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Session persistence error: {0}")]
    Persistence(#[from] std::io::Error),
    
    #[error("Session serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid session state: {0}")]
    InvalidState(String),
}

/// Session manager for handling multiple sessions
#[derive(Debug)]
pub struct SessionManager {
    sessions: RwLock<HashMap<SessionId, Session>>,
    session_dir: PathBuf,
}

impl SessionManager {
    pub fn new(session_dir: PathBuf) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            session_dir,
        }
    }
    
    pub async fn create_session(&self, name: String, workspace_path: PathBuf) -> Result<SessionId, SessionError> {
        let session = Session::new(name.clone(), workspace_path);
        let session_id = session.id.clone();
        
        let mut sessions = self.sessions.write().await;
        
        // Check if session with same name already exists
        if sessions.values().any(|s| s.name == name) {
            return Err(SessionError::AlreadyExists(name));
        }
        
        sessions.insert(session_id.clone(), session);
        
        // Persist session to disk
        self.save_session(&session_id).await?;
        
        Ok(session_id)
    }
    
    pub async fn get_session(&self, session_id: &SessionId) -> Result<Session, SessionError> {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| SessionError::NotFound(session_id.to_string()))
    }
    
    pub async fn get_session_by_name(&self, name: &str) -> Result<Session, SessionError> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .find(|s| s.name == name)
            .cloned()
            .ok_or_else(|| SessionError::NotFound(name.to_string()))
    }
    
    pub async fn update_session<F>(&self, session_id: &SessionId, f: F) -> Result<(), SessionError>
    where
        F: FnOnce(&mut Session),
    {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| SessionError::NotFound(session_id.to_string()))?;
        
        f(session);
        
        // Persist changes
        drop(sessions);
        self.save_session(session_id).await
    }
    
    pub async fn delete_session(&self, session_id: &SessionId) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        sessions
            .remove(session_id)
            .ok_or_else(|| SessionError::NotFound(session_id.to_string()))?;
        
        // Remove from disk
        let session_file = self.session_file_path(session_id);
        if session_file.exists() {
            tokio::fs::remove_file(session_file).await?;
        }
        
        Ok(())
    }
    
    pub async fn list_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }
    
    async fn save_session(&self, session_id: &SessionId) -> Result<(), SessionError> {
        let sessions = self.sessions.read().await;
        let session = sessions
            .get(session_id)
            .ok_or_else(|| SessionError::NotFound(session_id.to_string()))?;
        
        // Ensure session directory exists
        tokio::fs::create_dir_all(&self.session_dir).await?;
        
        // Serialize and save session
        let session_json = serde_json::to_string_pretty(session)?;
        let session_file = self.session_file_path(session_id);
        tokio::fs::write(session_file, session_json).await?;
        
        Ok(())
    }
    
    pub async fn load_sessions(&self) -> Result<(), SessionError> {
        if !self.session_dir.exists() {
            return Ok(());
        }
        
        let mut dir = tokio::fs::read_dir(&self.session_dir).await?;
        let mut sessions = self.sessions.write().await;
        
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                let content = tokio::fs::read_to_string(&path).await?;
                let session: Session = serde_json::from_str(&content)?;
                sessions.insert(session.id.clone(), session);
            }
        }
        
        Ok(())
    }
    
    fn session_file_path(&self, session_id: &SessionId) -> PathBuf {
        self.session_dir.join(format!("{}.json", session_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn session_id_generation() {
        let id1 = SessionId::new();
        let id2 = SessionId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn session_id_from_name_deterministic() {
        let id1 = SessionId::from_name("test-session");
        let id2 = SessionId::from_name("test-session");
        assert_eq!(id1, id2);
    }

    #[test]
    fn session_creation() {
        let workspace = PathBuf::from("/tmp/test");
        let session = Session::new("test".to_string(), workspace.clone());
        
        assert_eq!(session.name, "test");
        assert_eq!(session.workspace_path, workspace);
        assert!(session.panes.is_empty());
        assert_eq!(session.active_pane, None);
    }

    #[test]
    fn session_pane_management() {
        use super::super::PaneId;
        
        let workspace = PathBuf::from("/tmp/test");
        let mut session = Session::new("test".to_string(), workspace);
        let pane_id = PaneId::new();
        
        session.add_pane(pane_id.clone());
        assert_eq!(session.panes.len(), 1);
        assert_eq!(session.active_pane, Some(pane_id.clone()));
        
        session.remove_pane(&pane_id);
        assert!(session.panes.is_empty());
        assert_eq!(session.active_pane, None);
    }

    #[tokio::test]
    async fn session_manager_operations() {
        let temp_dir = tempdir().unwrap();
        let manager = SessionManager::new(temp_dir.path().to_path_buf());
        
        let workspace = PathBuf::from("/tmp/test");
        let session_id = manager
            .create_session("test-session".to_string(), workspace)
            .await
            .unwrap();
        
        let session = manager.get_session(&session_id).await.unwrap();
        assert_eq!(session.name, "test-session");
        
        let sessions = manager.list_sessions().await;
        assert_eq!(sessions.len(), 1);
        
        manager.delete_session(&session_id).await.unwrap();
        let sessions = manager.list_sessions().await;
        assert!(sessions.is_empty());
    }

    #[tokio::test]
    async fn session_persistence() {
        let temp_dir = tempdir().unwrap();
        let session_dir = temp_dir.path().to_path_buf();
        
        let session_id = {
            let manager = SessionManager::new(session_dir.clone());
            let workspace = PathBuf::from("/tmp/test");
            manager
                .create_session("persistent-session".to_string(), workspace)
                .await
                .unwrap()
        };
        
        // Create new manager and load sessions
        let manager = SessionManager::new(session_dir);
        manager.load_sessions().await.unwrap();
        
        let session = manager.get_session(&session_id).await.unwrap();
        assert_eq!(session.name, "persistent-session");
    }
}