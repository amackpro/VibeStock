use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};

use crate::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let mut rx = state.ws_tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    // Send welcome message
    let welcome = serde_json::json!({
        "event": "Connected",
        "payload": { "message": "VibeStock real-time feed connected" }
    });
    let _ = sender.send(Message::Text(welcome.to_string())).await;

    // Spawn task to forward broadcast events to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let json = match serde_json::to_string(&event) {
                Ok(j) => j,
                Err(_) => continue,
            };
            if sender.send(Message::Text(json)).await.is_err() {
                break; // client disconnected
            }
        }
    });

    // Keep connection alive until client disconnects
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Close(_) = msg {
                break;
            }
        }
    });

    // If either task finishes, abort both (cleanup)
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    tracing::info!("WebSocket client disconnected");
}
