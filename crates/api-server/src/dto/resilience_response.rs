use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceResponse {
    pub circuit_state: String,
    pub inflight_requests: u64,
    pub queue_depth: u64,
    pub pool_saturation: f64,
}
