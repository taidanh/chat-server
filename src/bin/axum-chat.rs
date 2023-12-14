use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use axum::response::IntoResponse;
use axum::{routing::get, Router};
// use tokio_tungstenite::tungstenite::protocol::Message;
// use tokio_tungstenite::WebSocketStream;

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(message)) = socket.recv().await {
        // if message.is_text() || message.is_binary() {
        //     socket.send(message).await.unwrap();
        // }
        match message {
            Message::Text(text) => {
                // Handle text message
                socket.send(Message::Text(text)).await.unwrap();
            }
            Message::Binary(bin) => {
                // Handle binary message
                socket.send(Message::Binary(bin)).await.unwrap();
            }
            // Handle other message types as needed
            _ => {}
        }
    }
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
