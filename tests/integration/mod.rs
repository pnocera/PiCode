//! Integration tests for PiCode

pub mod llm_integration;
pub mod workspace_integration;
pub mod hooks_integration;

use super::{TestContext, MockLLMProvider};
use picode::Result;

/// Test full integration flows
#[cfg(test)]
mod tests {
    use super::*;
    use picode::{core::*, llm::*};

    #[tokio::test]
    async fn test_llm_provider_integration() {
        let ctx = TestContext::new().expect("Failed to create test context");
        ctx.create_test_config().expect("Failed to create test config");

        // This test will be enabled once LLM providers are implemented
        // let provider = LlmProvider::new("test", "test-key");
        // let response = provider.generate("Hello, world!").await;
        // assert_llm_response_valid!(response);
    }

    #[tokio::test]
    async fn test_workspace_session_integration() {
        let ctx = TestContext::new().expect("Failed to create test context");
        
        let session_id = SessionId::new();
        let session = Session::new(session_id.clone(), ctx.session_id.clone());
        
        let workspace_config = WorkspaceConfig {
            root_path: ctx.temp_dir.path().to_path_buf(),
            name: "integration-test".to_string(),
            layout: "default".to_string(),
        };
        
        let workspace = Workspace::new(workspace_config).expect("Failed to create workspace");
        
        // Test session-workspace integration
        assert_eq!(session.id(), &session_id);
        assert_eq!(workspace.name(), "integration-test");
    }

    #[tokio::test]
    async fn test_command_pane_integration() {
        let pane_id = PaneId::new();
        let pane = Pane::new(pane_id.clone(), PaneType::Terminal);
        
        let command = CommandBuilder::new("echo")
            .arg("integration-test")
            .build();
        
        // Test command execution within pane context
        assert_eq!(pane.id(), &pane_id);
        assert_eq!(command.program(), "echo");
    }

    #[tokio::test]
    async fn test_event_bus_integration() {
        let event_id = event::EventId::new();
        let event = Event::new(event_id.clone(), "integration-test".to_string());
        
        // Test event system integration
        assert_eq!(event.id(), &event_id);
    }
}