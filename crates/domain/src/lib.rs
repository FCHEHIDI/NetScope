pub mod context;
pub mod errors;
pub mod events;
pub mod ports;
pub mod resilience;

pub use context::RequestContext;
pub use errors::DomainError;
pub use events::{AuditEvent, OutboundResult, Outcome, RequestMetricPoint};
pub use ports::{AuditSink, EventBus, MetricsRecorder, OutboundGateway};
pub use resilience::{CircuitState, ResilienceSnapshot};
