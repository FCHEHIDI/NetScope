use async_trait::async_trait;

use crate::context::RequestContext;
use crate::errors::DomainError;
use crate::events::{AuditEvent, OutboundResult, RequestMetricPoint};

#[async_trait]
pub trait OutboundGateway: Send + Sync {
    async fn execute(&self, context: &RequestContext) -> Result<OutboundResult, DomainError>;
}

#[async_trait]
pub trait AuditSink: Send + Sync {
    async fn persist(&self, event: &AuditEvent) -> Result<(), DomainError>;
}

pub trait MetricsRecorder: Send + Sync {
    fn record(&self, point: &RequestMetricPoint);
}

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish_json(&self, topic: &str, payload: &str) -> Result<(), DomainError>;
}
