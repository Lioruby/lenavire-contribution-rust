use crate::ledger::application::ports::event_streams::EventStream;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default, Clone)]
pub struct InMemoryEventStream {
    pub events: Arc<Mutex<Vec<String>>>,
}

impl InMemoryEventStream {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl EventStream for InMemoryEventStream {
    async fn send(&self, event: &str) {
        let mut events = self.events.lock().await;
        events.push(event.to_string());
    }
}
