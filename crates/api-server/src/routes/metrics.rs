use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::app_state::AppState;

/// `GET /v1/metrics/network`
///
/// Retourne les compteurs agrégés et les percentiles de latence p50/p95/p99
/// par label (route_group × internal_ip × external_ip), conformément à
/// l'invariant I5 (isolation NAT) et à l'invariant I3 (cohérence métrique).
pub async fn network(State(state): State<AppState>) -> Json<Value> {
    let snapshot = state.metrics.snapshot();

    let latency_percentiles: Vec<Value> = snapshot
        .latency_by_label
        .iter()
        .map(|p| {
            json!({
                "route_group": p.label.route_group,
                "internal_ip":  p.label.internal_ip,
                "external_ip":  p.label.external_ip,
                "p50_ms":        p.p50_ms,
                "p95_ms":        p.p95_ms,
                "p99_ms":        p.p99_ms,
                "sample_count":  p.sample_count,
            })
        })
        .collect();

    Json(json!({
        "status": "ok",
        "data": {
            "requests_total":     snapshot.requests_total,
            "errors_total":       snapshot.errors_total,
            "retries_total":      snapshot.retries_total,
            "timeouts_total":     snapshot.timeouts_total,
            "latency_percentiles": latency_percentiles
        }
    }))
}
