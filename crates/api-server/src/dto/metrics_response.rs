use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub requests_total: u64,
    pub errors_total: u64,
    pub retries_total: u64,
    pub timeouts_total: u64,
}
