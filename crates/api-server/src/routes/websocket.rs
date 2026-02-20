use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};

pub async fn events(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let payload = Message::Text(
        "{\"event_type\":\"system.heartbeat\",\"event_version\":\"1.0\"}".into(),
    );
    let _ = socket.send(payload).await;
}
