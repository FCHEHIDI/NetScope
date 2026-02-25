use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use serde_json::json;
use tokio::sync::broadcast::error::RecvError;
use tokio::time::{Duration, interval};
use tracing::{info, warn};

use crate::app_state::AppState;

/// `GET /v1/ws/events`
///
/// Upgrade HTTP → WebSocket. Chaque client reçoit :
/// - les événements réseau diffusés via le `WsHub` (broadcast),
/// - un heartbeat `system.heartbeat` toutes les 30 secondes,
/// - les réponses ping/pong pour maintenir la connexion.
///
/// **Politique de surcharge** : si le canal broadcast est plein
/// (client trop lent), les messages les plus anciens sont perdus
/// (`RecvError::Lagged`) — jamais de blocage côté serveur.
pub async fn events(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.ws_hub.subscribe();
    // Intervalle heartbeat — tick() consommé immédiatement pour ne pas
    // déclencher un ping dès la connexion.
    let mut heartbeat = interval(Duration::from_secs(30));
    heartbeat.tick().await;

    info!(subscribers = state.ws_hub.subscriber_count(), "ws client connected");

    loop {
        tokio::select! {
            // ── Événement diffusé depuis le hub ──
            result = rx.recv() => {
                match result {
                    Ok(payload) => {
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(RecvError::Lagged(n)) => {
                        // Le client est trop lent : on log le nombre de messages
                        // perdus et on continue — pas de déconnexion forcée.
                        warn!(dropped = n, "ws client lagged, messages dropped (drop-oldest)");
                    }
                    Err(RecvError::Closed) => break,
                }
            }

            // ── Heartbeat périodique ──
            _ = heartbeat.tick() => {
                let ping = json!({
                    "event_type": "system.heartbeat",
                    "event_version": "1.0"
                }).to_string();
                if socket.send(Message::Text(ping.into())).await.is_err() {
                    break;
                }
            }

            // ── Messages entrants du client ──
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Ping(payload))) => {
                        // Répondre au ping pour garder la connexion vivante.
                        let _ = socket.send(Message::Pong(payload)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {} // texte/binaire client ignoré (serveur-push only)
                }
            }
        }
    }

    info!("ws client disconnected");
}
