//! Event system for PiCode
//! 
//! Handles events, messaging, and coordination between components

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Unique identifier for events
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Core event types in PiCode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    // Session events
    SessionCreated {
        session_id: super::SessionId,
        name: String,
        workspace_path: std::path::PathBuf,
    },
    SessionActivated {
        session_id: super::SessionId,
    },
    SessionClosed {
        session_id: super::SessionId,
    },
    
    // Pane events
    PaneCreated {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        pane_type: super::pane::PaneType,
    },
    PaneActivated {
        session_id: super::SessionId,
        pane_id: super::PaneId,
    },
    PaneResized {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        width: u16,
        height: u16,
    },
    PaneClosed {
        session_id: super::SessionId,
        pane_id: super::PaneId,
    },
    
    // Command events
    CommandStarted {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        command_id: super::command::CommandId,
        command: String,
    },
    CommandCompleted {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        command_id: super::command::CommandId,
        status: super::command::CommandStatus,
        duration: std::time::Duration,
    },
    
    // LLM events
    LLMRequestStarted {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        provider: String,
        model: String,
        prompt: String,
    },
    LLMResponseReceived {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        provider: String,
        model: String,
        response: String,
        tokens_used: Option<u32>,
    },
    LLMError {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        provider: String,
        error: String,
    },
    
    // File system events
    FileOpened {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        file_path: std::path::PathBuf,
    },
    FileModified {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        file_path: std::path::PathBuf,
    },
    FileSaved {
        session_id: super::SessionId,
        pane_id: super::PaneId,
        file_path: std::path::PathBuf,
    },
    
    // Workspace events
    WorkspaceScanned {
        session_id: super::SessionId,
        file_count: usize,
        total_size: u64,
    },
    
    // Hook events
    HookTriggered {
        session_id: super::SessionId,
        hook_name: String,
        event_type: String,
        context: HashMap<String, String>,
    },
    
    // System events
    SystemShutdown,
    SystemError {
        error: String,
        context: HashMap<String, String>,
    },
    
    // Custom events for extensibility
    Custom {
        name: String,
        data: HashMap<String, serde_json::Value>,
    },
}

impl Event {
    pub fn id(&self) -> EventId {
        EventId::new()
    }
    
    pub fn event_type(&self) -> &'static str {
        match self {
            Event::SessionCreated { .. } => "session_created",
            Event::SessionActivated { .. } => "session_activated",
            Event::SessionClosed { .. } => "session_closed",
            Event::PaneCreated { .. } => "pane_created",
            Event::PaneActivated { .. } => "pane_activated",
            Event::PaneResized { .. } => "pane_resized",
            Event::PaneClosed { .. } => "pane_closed",
            Event::CommandStarted { .. } => "command_started",
            Event::CommandCompleted { .. } => "command_completed",
            Event::LLMRequestStarted { .. } => "llm_request_started",
            Event::LLMResponseReceived { .. } => "llm_response_received",
            Event::LLMError { .. } => "llm_error",
            Event::FileOpened { .. } => "file_opened",
            Event::FileModified { .. } => "file_modified",
            Event::FileSaved { .. } => "file_saved",
            Event::WorkspaceScanned { .. } => "workspace_scanned",
            Event::HookTriggered { .. } => "hook_triggered",
            Event::SystemShutdown => "system_shutdown",
            Event::SystemError { .. } => "system_error",
            Event::Custom { .. } => "custom",
        }
    }
    
    pub fn session_id(&self) -> Option<&super::SessionId> {
        match self {
            Event::SessionCreated { session_id, .. }
            | Event::SessionActivated { session_id }
            | Event::SessionClosed { session_id }
            | Event::PaneCreated { session_id, .. }
            | Event::PaneActivated { session_id, .. }
            | Event::PaneResized { session_id, .. }
            | Event::PaneClosed { session_id, .. }
            | Event::CommandStarted { session_id, .. }
            | Event::CommandCompleted { session_id, .. }
            | Event::LLMRequestStarted { session_id, .. }
            | Event::LLMResponseReceived { session_id, .. }
            | Event::LLMError { session_id, .. }
            | Event::FileOpened { session_id, .. }
            | Event::FileModified { session_id, .. }
            | Event::FileSaved { session_id, .. }
            | Event::WorkspaceScanned { session_id, .. }
            | Event::HookTriggered { session_id, .. } => Some(session_id),
            _ => None,
        }
    }
}

/// Event envelope with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: EventId,
    pub event: Event,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
    pub metadata: HashMap<String, String>,
}

impl EventEnvelope {
    pub fn new(event: Event, source: String) -> Self {
        Self {
            id: EventId::new(),
            event,
            timestamp: chrono::Utc::now(),
            source,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Event handler trait
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    async fn handle(&self, event: &EventEnvelope) -> Result<(), EventError>;
    
    /// Get the event types this handler is interested in
    fn event_types(&self) -> Vec<&'static str>;
    
    /// Get handler name for debugging
    fn name(&self) -> &str;
}

/// Event bus for coordinating events across the system
pub struct EventBus {
    sender: broadcast::Sender<EventEnvelope>,
    handlers: Arc<RwLock<HashMap<String, Box<dyn EventHandler>>>>,
    event_history: Arc<RwLock<Vec<EventEnvelope>>>,
    max_history_size: usize,
}

impl EventBus {
    pub fn new(channel_capacity: usize, max_history_size: usize) -> Self {
        let (sender, _) = broadcast::channel(channel_capacity);
        
        Self {
            sender,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            event_history: Arc::new(RwLock::new(Vec::new())),
            max_history_size,
        }
    }
    
    /// Register an event handler
    pub async fn register_handler(&self, handler: Box<dyn EventHandler>) {
        let name = handler.name().to_string();
        let mut handlers = self.handlers.write().await;
        handlers.insert(name, handler);
    }
    
    /// Unregister an event handler
    pub async fn unregister_handler(&self, name: &str) {
        let mut handlers = self.handlers.write().await;
        handlers.remove(name);
    }
    
    /// Publish an event
    pub async fn publish(&self, event: Event, source: String) -> Result<(), EventError> {
        let envelope = EventEnvelope::new(event, source);
        
        // Add to history
        let mut history = self.event_history.write().await;
        history.push(envelope.clone());
        
        // Maintain history size limit
        if history.len() > self.max_history_size {
            history.remove(0);
        }
        drop(history);
        
        // Send to broadcast channel
        self.sender.send(envelope.clone())
            .map_err(|_| EventError::PublishFailed("No receivers".to_string()))?;
        
        // Handle with registered handlers
        let handlers = self.handlers.read().await;
        for handler in handlers.values() {
            if handler.event_types().contains(&envelope.event.event_type()) {
                if let Err(e) = handler.handle(&envelope).await {
                    tracing::warn!("Handler {} failed to process event: {}", handler.name(), e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }
    
    /// Get event history
    pub async fn get_history(&self) -> Vec<EventEnvelope> {
        let history = self.event_history.read().await;
        history.clone()
    }
    
    /// Get events filtered by session
    pub async fn get_session_events(&self, session_id: &super::SessionId) -> Vec<EventEnvelope> {
        let history = self.event_history.read().await;
        history
            .iter()
            .filter(|e| e.event.session_id() == Some(session_id))
            .cloned()
            .collect()
    }
    
    /// Clear event history
    pub async fn clear_history(&self) {
        let mut history = self.event_history.write().await;
        history.clear();
    }
    
    /// Get handler count
    pub async fn handler_count(&self) -> usize {
        let handlers = self.handlers.read().await;
        handlers.len()
    }
}

/// Event-related errors
#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to publish event: {0}")]
    PublishFailed(String),
    
    #[error("Handler error: {0}")]
    Handler(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Channel error: {0}")]
    Channel(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::time::{sleep, Duration};

    // Test event handler
    struct TestHandler {
        name: String,
        event_types: Vec<&'static str>,
        call_count: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl EventHandler for TestHandler {
        async fn handle(&self, _event: &EventEnvelope) -> Result<(), EventError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        fn event_types(&self) -> Vec<&'static str> {
            self.event_types.clone()
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn event_id_generation() {
        let id1 = EventId::new();
        let id2 = EventId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn event_types() {
        let event = Event::SessionCreated {
            session_id: super::super::SessionId::new(),
            name: "test".to_string(),
            workspace_path: std::path::PathBuf::from("/tmp"),
        };
        
        assert_eq!(event.event_type(), "session_created");
        assert!(event.session_id().is_some());
    }

    #[test]
    fn event_envelope_creation() {
        let event = Event::SystemShutdown;
        let envelope = EventEnvelope::new(event, "test".to_string())
            .with_metadata("key".to_string(), "value".to_string());
        
        assert_eq!(envelope.source, "test");
        assert_eq!(envelope.metadata.get("key"), Some(&"value".to_string()));
    }

    #[tokio::test]
    async fn event_bus_creation() {
        let bus = EventBus::new(100, 1000);
        assert_eq!(bus.handler_count().await, 0);
    }

    #[tokio::test]
    async fn event_bus_handler_registration() {
        let bus = EventBus::new(100, 1000);
        let call_count = Arc::new(AtomicUsize::new(0));
        
        let handler = Box::new(TestHandler {
            name: "test_handler".to_string(),
            event_types: vec!["session_created"],
            call_count: call_count.clone(),
        });
        
        bus.register_handler(handler).await;
        assert_eq!(bus.handler_count().await, 1);
        
        bus.unregister_handler("test_handler").await;
        assert_eq!(bus.handler_count().await, 0);
    }

    #[tokio::test]
    async fn event_bus_publish_and_handle() {
        let bus = EventBus::new(100, 1000);
        let call_count = Arc::new(AtomicUsize::new(0));
        
        let handler = Box::new(TestHandler {
            name: "test_handler".to_string(),
            event_types: vec!["session_created"],
            call_count: call_count.clone(),
        });
        
        bus.register_handler(handler).await;
        
        let event = Event::SessionCreated {
            session_id: super::super::SessionId::new(),
            name: "test".to_string(),
            workspace_path: std::path::PathBuf::from("/tmp"),
        };
        
        bus.publish(event, "test".to_string()).await.unwrap();
        
        // Give handler time to process
        sleep(Duration::from_millis(10)).await;
        
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn event_bus_subscription() {
        let bus = EventBus::new(100, 1000);
        let mut receiver = bus.subscribe();
        
        let event = Event::SystemShutdown;
        
        // Publish event in background
        let bus_clone = bus.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(10)).await;
            bus_clone.publish(event, "test".to_string()).await.unwrap();
        });
        
        // Receive event
        let received = receiver.recv().await.unwrap();
        assert_eq!(received.source, "test");
        assert_eq!(received.event.event_type(), "system_shutdown");
    }

    #[tokio::test]
    async fn event_bus_history() {
        let bus = EventBus::new(100, 10);
        
        let event1 = Event::SystemShutdown;
        let event2 = Event::SessionCreated {
            session_id: super::super::SessionId::new(),
            name: "test".to_string(),
            workspace_path: std::path::PathBuf::from("/tmp"),
        };
        
        bus.publish(event1, "source1".to_string()).await.unwrap();
        bus.publish(event2, "source2".to_string()).await.unwrap();
        
        let history = bus.get_history().await;
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].source, "source1");
        assert_eq!(history[1].source, "source2");
    }

    #[tokio::test]
    async fn event_bus_session_filtering() {
        let bus = EventBus::new(100, 1000);
        let session_id = super::super::SessionId::new();
        let other_session_id = super::super::SessionId::new();
        
        let event1 = Event::SessionCreated {
            session_id: session_id.clone(),
            name: "test1".to_string(),
            workspace_path: std::path::PathBuf::from("/tmp"),
        };
        
        let event2 = Event::SessionCreated {
            session_id: other_session_id,
            name: "test2".to_string(),
            workspace_path: std::path::PathBuf::from("/tmp"),
        };
        
        let event3 = Event::SystemShutdown; // No session ID
        
        bus.publish(event1, "source1".to_string()).await.unwrap();
        bus.publish(event2, "source2".to_string()).await.unwrap();
        bus.publish(event3, "source3".to_string()).await.unwrap();
        
        let session_events = bus.get_session_events(&session_id).await;
        assert_eq!(session_events.len(), 1);
        assert_eq!(session_events[0].source, "source1");
    }

    #[tokio::test]
    async fn event_bus_history_size_limit() {
        let bus = EventBus::new(100, 2); // Only keep 2 events
        
        for i in 0..5 {
            let event = Event::Custom {
                name: format!("event_{}", i),
                data: HashMap::new(),
            };
            bus.publish(event, format!("source_{}", i)).await.unwrap();
        }
        
        let history = bus.get_history().await;
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].source, "source_3");
        assert_eq!(history[1].source, "source_4");
    }
}