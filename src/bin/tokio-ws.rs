use futures_util::stream::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    self, accept_async,
    tungstenite::protocol::{frame::coding::CloseCode, CloseFrame, Message},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        while let Ok((stream, _addr)) = listener.accept().await {
            let _ = tokio::spawn(handle_connection(stream));
        }
    }
}

async fn handle_connection(stream: TcpStream) {
    let mut ws_stream = accept_async(stream)
        .await
        .expect("Error during web socket handshake");

    while let Some(message_result) = ws_stream.next().await {
        match message_result {
            Ok(msg) => {
                println!("Received a message: {:?}", msg);

                // Close the WebSocket connection gracefully
                ws_stream
                    .close(Some(CloseFrame {
                        code: CloseCode::Normal,
                        reason: "Finished processing".into(),
                    }))
                    .await
                    .expect("Error closing connection");
                if let Some(message_result) = ws_stream.next().await {
                    if let Ok(Message::Close(_)) = message_result {
                        println!("Client acknowledged close, terminating connection.");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading message: {:?}", e);
            }
        }
    }
}
