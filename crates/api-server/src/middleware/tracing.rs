use tower_http::trace::TraceLayer;

/// Crée un `TraceLayer` Tower-HTTP pour la corrélation des requêtes HTTP.
///
/// Émet des spans `tracing` à chaque requête (méthode, URI, status, latence).
/// Les spans sont automatiquement corrélés avec `x-request-id` injecté
/// en amont par `set_request_id_layer()`.
pub fn trace_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
}
