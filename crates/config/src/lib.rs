use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub bind_addr: String,
    /// Délai maximal de traitement d'une requête HTTP côté serveur (ms).
    /// Passé ce délai, le middleware Tower répond 408 Request Timeout.
    pub request_timeout_ms: u64,
    /// Nombre maximal de requêtes traitées simultanément (Tower ConcurrencyLimitLayer).
    pub max_concurrent_requests: usize,
    /// Capacité du canal broadcast du hub WebSocket (drop-oldest si plein).
    pub ws_hub_capacity: usize,
}

/// Métriques : buckets d'histogramme et taille de fenêtre glissante.
///
/// Les buckets **doivent être identiques** entre tous les environnements
/// (dev, staging, prod) pour que p50/p95/p99 soient comparables et que
/// les SLO puissent être vérifiés de façon cohérente (invariant I3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Bornes des buckets de latence en millisecondes.
    pub latency_buckets_ms: Vec<f64>,
    /// Nombre maximal d'échantillons conservés par label dans la fenêtre glissante.
    pub max_latency_samples: usize,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            latency_buckets_ms: vec![10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 2500.0],
            max_latency_samples: 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CephConfig {
    pub endpoint: String,
    pub bucket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub service: ServiceConfig,
    pub kafka: KafkaConfig,
    pub ceph: CephConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid config: {0}")]
    Invalid(String),
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            service: ServiceConfig {
                bind_addr: "127.0.0.1:8080".to_string(),
                request_timeout_ms: 5_000,
                max_concurrent_requests: 500,
                ws_hub_capacity: 256,
            },
            kafka: KafkaConfig {
                brokers: "127.0.0.1:9092".to_string(),
                topic: "netscope.ingress".to_string(),
            },
            ceph: CephConfig {
                endpoint: "http://127.0.0.1:7480".to_string(),
                bucket: "netscope-audit".to_string(),
            },
            metrics: MetricsConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.service.bind_addr.is_empty() {
            return Err(ConfigError::Invalid("service.bind_addr is empty".to_string()));
        }
        if self.service.request_timeout_ms == 0 {
            return Err(ConfigError::Invalid(
                "service.request_timeout_ms must be > 0".to_string(),
            ));
        }
        if self.service.max_concurrent_requests == 0 {
            return Err(ConfigError::Invalid(
                "service.max_concurrent_requests must be > 0".to_string(),
            ));
        }
        if self.kafka.brokers.is_empty() || self.kafka.topic.is_empty() {
            return Err(ConfigError::Invalid("kafka config is incomplete".to_string()));
        }
        if self.ceph.endpoint.is_empty() || self.ceph.bucket.is_empty() {
            return Err(ConfigError::Invalid("ceph config is incomplete".to_string()));
        }
        if self.metrics.latency_buckets_ms.is_empty() {
            return Err(ConfigError::Invalid(
                "metrics.latency_buckets_ms must not be empty".to_string(),
            ));
        }
        if self.metrics.max_latency_samples == 0 {
            return Err(ConfigError::Invalid(
                "metrics.max_latency_samples must be > 0".to_string(),
            ));
        }
        Ok(())
    }
}
