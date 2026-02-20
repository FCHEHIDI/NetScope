use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    HalfOpen,
    Open,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResilienceSnapshot {
    pub circuit_state: CircuitState,
    pub inflight_requests: u64,
    pub queue_depth: u64,
    pub pool_saturation: f64,
}
