use domain::{CircuitState, ResilienceSnapshot};

pub fn snapshot() -> ResilienceSnapshot {
    ResilienceSnapshot {
        circuit_state: CircuitState::Closed,
        inflight_requests: 0,
        queue_depth: 0,
        pool_saturation: 0.0,
    }
}
