pub mod app_state;
pub mod dto;
pub mod extractors;
pub mod middleware;
pub mod routes;
pub mod services;

use axum::{Router, routing::get};
use config::AppConfig;
use tower::ServiceBuilder;

use app_state::AppState;
use middleware::{
    rate_limit::concurrency_limit_layer,
    request_id::{propagate_request_id_layer, set_request_id_layer},
    timeout::timeout_layer,
    tracing::trace_layer,
};

/// Construit le routeur Axum avec toutes les routes et middlewares Tower.
///
/// # Ordre des couches (ServiceBuilder — de l'extérieur vers l'intérieur)
///
/// 1. `SetRequestIdLayer`       — injecte `x-request-id` (UUID v4) si absent
/// 2. `PropagateRequestIdLayer` — recopie `x-request-id` dans la réponse
/// 3. `TraceLayer`              — span tracing par requête (méthode, URI, status)
/// 4. `TimeoutLayer`            — coupe à `request_timeout_ms` → 408
/// 5. `ConcurrencyLimitLayer`   — backpressure à `max_concurrent_requests`
pub fn build_router(config: AppConfig) -> Router {
    let timeout_ms = config.service.request_timeout_ms;
    let max_concurrent = config.service.max_concurrent_requests;
    let state = AppState::new(config);

    Router::new()
        .route("/v1/health/live",      get(routes::health::live))
        .route("/v1/health/ready",     get(routes::health::ready))
        .route("/v1/metrics/network",  get(routes::metrics::network))
        .route("/v1/resilience/state", get(routes::resilience::state))
        .route("/v1/ws/events",        get(routes::websocket::events))
        .layer(
            ServiceBuilder::new()
                .layer(set_request_id_layer())
                .layer(propagate_request_id_layer())
                .layer(trace_layer())
                .layer(timeout_layer(timeout_ms))
                .layer(concurrency_limit_layer(max_concurrent)),
        )
        .with_state(state)
}
