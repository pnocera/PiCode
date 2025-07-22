//! PiCode Core - Fundamental types and traits for the PiCode ecosystem

// use chrono::{DateTime, Utc}; // Unused import

pub mod session;
pub mod workspace;
pub mod pane;
pub mod command;
pub mod event;
pub mod traits;

pub use session::{Session, SessionId, SessionManager};
pub use workspace::{Workspace, WorkspaceConfig};
pub use pane::{Pane, PaneId, PaneType};
pub use command::{Command, CommandResult, CommandStatus, CommandBuilder};
pub use event::{Event, EventHandler, EventBus};
pub use traits::*;

/// Core result type
pub type Result<T> = std::result::Result<T, CoreError>;

/// Core error types for PiCode
#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Session error: {0}")]
    Session(#[from] session::SessionError),
    
    #[error("Workspace error: {0}")]
    Workspace(#[from] workspace::WorkspaceError),
    
    #[error("Pane error: {0}")]
    Pane(#[from] pane::PaneError),
    
    #[error("Command error: {0}")]
    Command(#[from] command::CommandError),
    
    #[error("Event error: {0}")]
    Event(#[from] event::EventError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_error_display() {
        let error = CoreError::Session(session::SessionError::NotFound("test".to_string()));
        assert!(error.to_string().contains("Session error"));
    }

    #[test]
    fn core_types_creation() {
        let session_id = SessionId::new();
        let pane_id = PaneId::new();
        let command_id = command::CommandId::new();
        let event_id = event::EventId::new();

        // Verify IDs are unique
        assert_ne!(session_id, SessionId::new());
        assert_ne!(pane_id, PaneId::new());
        assert_ne!(command_id, command::CommandId::new());
        assert_ne!(event_id, event::EventId::new());
    }
}