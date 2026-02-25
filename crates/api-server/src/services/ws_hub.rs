use tokio::sync::broadcast;
use tracing::debug;

/// Capacité par défaut du canal broadcast (nombre de messages en transit).
const DEFAULT_CAPACITY: usize = 256;

/// Hub WebSocket basé sur `tokio::sync::broadcast`.
///
/// # Schéma de diffusion
/// Un seul `Arc<WsHub>` est partagé dans `AppState`. Chaque client WebSocket
/// reçoit un `Receiver` via `subscribe()`. Si un client est trop lent et
/// que le canal est plein, les messages les plus anciens sont écrasés
/// (stratégie **drop-oldest** implicite du broadcast — invariant S4).
///
/// # Thread-safety
/// `broadcast::Sender<String>` est `Send + Sync` : le hub peut être cloné
/// dans `Arc` et partagé entre threads sans verrou supplémentaire.
#[derive(Debug)]
pub struct WsHub {
    sender: broadcast::Sender<String>,
}

impl WsHub {
    /// Crée un hub avec une capacité de canal configurable.
    ///
    /// `capacity` doit correspondre à `ServiceConfig::ws_hub_capacity`.
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Publie un événement JSON vers tous les clients connectés.
    ///
    /// Retourne le nombre de récepteurs ayant reçu le message.
    /// Si aucun client n'est abonné, l'erreur est silencieuse (comportement normal).
    pub fn publish(&self, event: String) -> usize {
        match self.sender.send(event) {
            Ok(n) => {
                debug!(receivers = n, "ws event published");
                n
            }
            Err(_) => {
                debug!("ws publish: no active subscribers");
                0
            }
        }
    }

    /// Crée un `Receiver` pour un nouveau client WebSocket.
    ///
    /// Si le client accumule du retard (canal plein), il reçoit
    /// `RecvError::Lagged(n)` indiquant combien de messages ont été perdus.
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }

    /// Nombre de clients WebSocket actuellement abonnés.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for WsHub {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}
