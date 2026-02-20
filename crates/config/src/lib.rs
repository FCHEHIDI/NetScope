use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub bind_addr: String,
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
            },
            kafka: KafkaConfig {
                brokers: "127.0.0.1:9092".to_string(),
                topic: "netscope.ingress".to_string(),
            },
            ceph: CephConfig {
                endpoint: "http://127.0.0.1:7480".to_string(),
                bucket: "netscope-audit".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.service.bind_addr.is_empty() {
            return Err(ConfigError::Invalid("service.bind_addr is empty".to_string()));
        }
        if self.kafka.brokers.is_empty() || self.kafka.topic.is_empty() {
            return Err(ConfigError::Invalid("kafka config is incomplete".to_string()));
        }
        if self.ceph.endpoint.is_empty() || self.ceph.bucket.is_empty() {
            return Err(ConfigError::Invalid("ceph config is incomplete".to_string()));
        }
        Ok(())
    }
}
