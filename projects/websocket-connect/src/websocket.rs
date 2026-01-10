use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};

/// Echo handler for testing (will be replaced with TCP bridge)
pub async fn echo_handler(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                tracing::debug!("Received text: {}", text);
                if sender.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Binary(data)) => {
                tracing::debug!("Received {} bytes", data.len());
                if sender.send(Message::Binary(data)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket closed by client");
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    tracing::info!("WebSocket connection closed");
}

// TODO: Implement TCP bridge
// pub async fn tcp_bridge_handler(socket: WebSocket, host: String, port: u16) {
//     // Connect to TCP server
//     // Bridge messages between WebSocket and TCP
// }
