use std::sync::Arc;

use config::AppConfig;
use metrics_engine::InMemoryMetricsEngine;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub metrics: Arc<InMemoryMetricsEngine>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let metrics = Arc::new(InMemoryMetricsEngine::new(
            config.metrics.max_latency_samples,
        ));
        Self { config, metrics }
    }
}
