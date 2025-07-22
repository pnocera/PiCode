//! Unit tests for PiCode components

pub mod core_tests;
pub mod config_tests;
pub mod llm_tests;
pub mod cli_tests;
pub mod hooks_tests;

use super::TestContext;
use picode::Result;

/// Test core functionality units
#[cfg(test)]
mod tests {
    use super::*;
    use picode::core::*;

    #[tokio::test]
    async fn test_session_creation() {
        let ctx = TestContext::new().expect("Failed to create test context");
        ctx.create_test_config().expect("Failed to create test config");

        let session_id = SessionId::new();
        let session = Session::new(session_id.clone(), "test-session".to_string());
        
        assert_eq!(session.id(), &session_id);
        assert_eq!(session.name(), "test-session");
    }

    #[tokio::test]
    async fn test_workspace_initialization() {
        let ctx = TestContext::new().expect("Failed to create test context");
        
        let workspace_config = WorkspaceConfig {
            root_path: ctx.temp_dir.path().to_path_buf(),
            name: "test-workspace".to_string(),
            layout: "default".to_string(),
        };
        
        let workspace = Workspace::new(workspace_config).expect("Failed to create workspace");
        assert_eq!(workspace.name(), "test-workspace");
    }

    #[tokio::test]
    async fn test_pane_management() {
        let pane_id = PaneId::new();
        let pane = Pane::new(pane_id.clone(), PaneType::Terminal);
        
        assert_eq!(pane.id(), &pane_id);
        assert_eq!(pane.pane_type(), &PaneType::Terminal);
    }

    #[tokio::test]
    async fn test_command_execution_flow() {
        let command = CommandBuilder::new("echo")
            .arg("test")
            .build();
        
        assert_eq!(command.program(), "echo");
        assert_eq!(command.args(), &["test"]);
    }

    #[test]
    fn test_event_system() {
        let event_id = event::EventId::new();
        let event = Event::new(event_id.clone(), "test-event".to_string());
        
        assert_eq!(event.id(), &event_id);
        assert_eq!(event.event_type(), "test-event");
    }
}