use async_trait::async_trait;

use domain::{DomainError, OutboundGateway, OutboundResult, Outcome, RequestContext};

#[derive(Debug, Clone)]
pub struct ResiliencePolicy {
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub global_timeout_ms: u64,
    pub max_attempts: u32,
}

#[derive(Debug, Clone)]
pub struct StubOutboundClient {
    pub policy: ResiliencePolicy,
}

impl Default for StubOutboundClient {
    fn default() -> Self {
        Self {
            policy: ResiliencePolicy {
                connect_timeout_ms: 200,
                read_timeout_ms: 1000,
                global_timeout_ms: 1500,
                max_attempts: 2,
            },
        }
    }
}

#[async_trait]
impl OutboundGateway for StubOutboundClient {
    async fn execute(&self, context: &RequestContext) -> Result<OutboundResult, DomainError> {
        if context.idempotency_key.is_empty() {
            return Err(DomainError::InvalidInput(
                "idempotency_key is required".to_string(),
            ));
        }

        Ok(OutboundResult {
            status_code: 200,
            latency_ms: 42,
            bytes_sent: 256,
            bytes_received: 512,
            retry_count: 0,
            timeout_count: 0,
            outcome: Outcome::Success,
        })
    }
}
