pub mod app_state;
pub mod dto;
pub mod extractors;
pub mod middleware;
pub mod routes;
pub mod services;

use axum::{Router, routing::get};
use config::AppConfig;

use app_state::AppState;

pub fn build_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    Router::new()
        .route("/v1/health/live", get(routes::health::live))
        .route("/v1/health/ready", get(routes::health::ready))
        .route("/v1/metrics/network", get(routes::metrics::network))
        .route("/v1/resilience/state", get(routes::resilience::state))
        .route("/v1/ws/events", get(routes::websocket::events))
        .with_state(state)
}
