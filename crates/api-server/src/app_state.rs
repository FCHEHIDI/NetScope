use std::sync::Arc;

use config::AppConfig;
use metrics_engine::InMemoryMetricsEngine;

use crate::services::ws_hub::WsHub;

/// État global de l'application, clonable (Arc interne) et injecté par Axum.
///
/// Contient uniquement des handles sur des services applicatifs thread-safe.
/// Aucun singleton implicite global — toute dépendance est injectée ici.
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub metrics: Arc<InMemoryMetricsEngine>,
    pub ws_hub: Arc<WsHub>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let metrics = Arc::new(InMemoryMetricsEngine::new(
            config.metrics.max_latency_samples,
        ));
        let ws_hub = Arc::new(WsHub::new(config.service.ws_hub_capacity));
        Self { config, metrics, ws_hub }
    }
}
