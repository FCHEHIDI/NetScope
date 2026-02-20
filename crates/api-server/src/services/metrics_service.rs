use metrics_engine::{InMemoryMetricsEngine, NetworkMetricsSnapshot};

pub fn snapshot(metrics: &InMemoryMetricsEngine) -> NetworkMetricsSnapshot {
    metrics.snapshot()
}
