use crate::ledger::application::ports::event_streams::EventStream;
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Clone)]
pub struct WebSocketEventStream {
    ws_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl WebSocketEventStream {
    pub async fn new() -> Self {
        let url = "wss://echo.websocket.events";
        println!("Connecting to {}", url);
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        Self {
            ws_stream: Arc::new(Mutex::new(ws_stream)),
        }
    }
}

#[async_trait]
impl EventStream for WebSocketEventStream {
    async fn send(&self, event: &str) {
        let mut stream = self.ws_stream.lock().await;
        stream.send(Message::Text(event.to_string())).await.unwrap();
    }
}
