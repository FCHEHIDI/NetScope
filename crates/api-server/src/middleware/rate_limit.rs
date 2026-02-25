use tower::limit::ConcurrencyLimitLayer;

/// Crée un `ConcurrencyLimitLayer` limitant le nombre de requêtes
/// traitées simultanément.
///
/// Les requêtes excédentaires sont mises en attente dans la file Tower
/// (backpressure implicite). La valeur provient de
/// `ServiceConfig::max_concurrent_requests`.
pub fn concurrency_limit_layer(max: usize) -> ConcurrencyLimitLayer {
    ConcurrencyLimitLayer::new(max)
}
