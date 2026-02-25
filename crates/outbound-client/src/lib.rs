use std::time::Instant;

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
    /// Exécute un appel sortant simulé en mesurant la latence via horloge
    /// monotone (`Instant`), garantissant une valeur toujours positive
    /// indépendamment des corrections NTP (invariant I3).
    async fn execute(&self, context: &RequestContext) -> Result<OutboundResult, DomainError> {
        if context.idempotency_key.is_empty() {
            return Err(DomainError::InvalidInput(
                "idempotency_key is required".to_string(),
            ));
        }

        // Mesure monotone : démarre avant l'appel, calcule l'élapsé après.
        // En implémentation réelle (reqwest), le `started_at` encadre
        // la requête HTTP complète (connect + send + recv).
        let started_at = Instant::now();

        // Stub : simule une réponse immédiate. Remplacer par reqwest::Client.
        let latency_ms = started_at.elapsed().as_millis() as u64;

        Ok(OutboundResult {
            status_code: 200,
            latency_ms,
            bytes_sent: 256,
            bytes_received: 512,
            retry_count: 0,
            timeout_count: 0,
            outcome: Outcome::Success,
        })
    }
}
