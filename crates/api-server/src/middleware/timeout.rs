use std::time::Duration;

use tower_http::timeout::TimeoutLayer;

/// Crée un `TimeoutLayer` Tower-HTTP avec la durée fournie.
///
/// Si le handler dépasse ce délai, la couche interrompt la requête
/// et renvoie automatiquement une réponse `408 Request Timeout`.
/// La valeur provient de `ServiceConfig::request_timeout_ms` pour
/// garantir la cohérence entre environnements.
#[allow(deprecated)] // TimeoutLayer::new reste fonctionnel — migration vers
                     // with_status_code planifiée après stabilisation tower-http 0.6
pub fn timeout_layer(timeout_ms: u64) -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_millis(timeout_ms))
}
