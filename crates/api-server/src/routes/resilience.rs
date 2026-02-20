use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::app_state::AppState;

pub async fn state(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "data": {
            "timeouts": {
                "connect_ms": 200,
                "read_ms": 1000,
                "global_ms": 1500
            },
            "retry": {
                "max_attempts": 2,
                "backoff_base_ms": 50,
                "jitter": true,
                "idempotent_only": true
            },
            "circuit_breaker": {
                "state": "closed"
            }
        }
    }))
}
