use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url; // Import the necessary traits

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080").expect("Failed to parse URL");

    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    ws_stream
        .send(Message::Text("Hello Server".to_string()))
        .await
        .expect("Failed to send message");

    // Receive a message
    if let Some(message) = ws_stream.next().await {
        match message {
            Ok(Message::Close(frame)) => {
                println!("Received close frame from server: {:?}", frame);
                // The client can simply close the connection here.
                // No need to send another close message.
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
            _ => {}
        }
    }
}
