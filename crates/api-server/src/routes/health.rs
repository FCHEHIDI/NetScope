use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::app_state::AppState;

pub async fn live(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "data": {
            "service": "netscope",
            "state": "live"
        }
    }))
}

pub async fn ready(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "data": {
            "service": "netscope",
            "state": "ready",
            "dependencies": {
                "kafka": "up",
                "ceph_rgw": "up",
                "metrics_engine": "up"
            }
        }
    }))
}
