use std::sync::atomic::{AtomicU64, Ordering};

use domain::{MetricsRecorder, RequestMetricPoint};

#[derive(Debug, Default)]
pub struct InMemoryMetricsEngine {
    requests_total: AtomicU64,
    errors_total: AtomicU64,
    retries_total: AtomicU64,
    timeouts_total: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct NetworkMetricsSnapshot {
    pub requests_total: u64,
    pub errors_total: u64,
    pub retries_total: u64,
    pub timeouts_total: u64,
}

impl InMemoryMetricsEngine {
    pub fn snapshot(&self) -> NetworkMetricsSnapshot {
        NetworkMetricsSnapshot {
            requests_total: self.requests_total.load(Ordering::Relaxed),
            errors_total: self.errors_total.load(Ordering::Relaxed),
            retries_total: self.retries_total.load(Ordering::Relaxed),
            timeouts_total: self.timeouts_total.load(Ordering::Relaxed),
        }
    }
}

impl MetricsRecorder for InMemoryMetricsEngine {
    fn record(&self, point: &RequestMetricPoint) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.retries_total
            .fetch_add(u64::from(point.retry_count), Ordering::Relaxed);
        self.timeouts_total
            .fetch_add(u64::from(point.timeout_count), Ordering::Relaxed);

        if point.is_error {
            self.errors_total.fetch_add(1, Ordering::Relaxed);
        }
    }
}
