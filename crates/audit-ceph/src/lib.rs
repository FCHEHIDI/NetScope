use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use domain::{AuditEvent, AuditSink, DomainError};

#[derive(Debug, Clone, Default)]
pub struct CephAuditSink {
    events: Arc<Mutex<Vec<AuditEvent>>>,
}

impl CephAuditSink {
    pub fn stored_events(&self) -> usize {
        self.events.lock().map(|events| events.len()).unwrap_or_default()
    }
}

#[async_trait]
impl AuditSink for CephAuditSink {
    async fn persist(&self, event: &AuditEvent) -> Result<(), DomainError> {
        let mut guard = self
            .events
            .lock()
            .map_err(|error| DomainError::AuditSinkError(error.to_string()))?;

        guard.push(event.clone());
        Ok(())
    }
}
