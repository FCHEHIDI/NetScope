use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::app_state::AppState;

pub async fn network(State(state): State<AppState>) -> Json<Value> {
    let snapshot = state.metrics.snapshot();
    Json(json!({
        "status": "ok",
        "data": {
            "requests_total": snapshot.requests_total,
            "errors_total": snapshot.errors_total,
            "retries_total": snapshot.retries_total,
            "timeouts_total": snapshot.timeouts_total
        }
    }))
}
