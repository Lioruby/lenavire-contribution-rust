use crate::ledger::application::ports::event_streams::EventStream;
use async_trait::async_trait;
use futures_util::SinkExt;
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Clone)]
pub struct WebSocketEventStream {
    ws_stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    initialized: Arc<AtomicBool>,
}

impl WebSocketEventStream {
    pub fn new() -> Self {
        Self {
            ws_stream: Arc::new(Mutex::new(None)),
            initialized: Arc::new(AtomicBool::new(false)),
        }
    }

    async fn ensure_connected(&self) {
        if self.initialized.load(Ordering::SeqCst) {
            return;
        }

        let mut ws_guard = self.ws_stream.lock().await;
        if ws_guard.is_some() {
            return;
        }

        let url = "ws://127.0.0.1:8080/ws";
        let mut retries = 5;

        while retries > 0 {
            println!(
                "Attempting to connect to {} (attempts remaining: {})",
                url, retries
            );

            match connect_async(url).await {
                Ok((stream, _)) => {
                    println!("Successfully connected to WebSocket server");
                    *ws_guard = Some(stream);
                    self.initialized.store(true, Ordering::SeqCst);
                    return;
                }
                Err(e) => {
                    println!("Failed to connect, retrying in 2 seconds: {}", e);
                    sleep(Duration::from_secs(2)).await;
                    retries -= 1;
                }
            }
        }

        panic!("Failed to connect to WebSocket server after 5 attempts");
    }
}

#[async_trait]
impl EventStream for WebSocketEventStream {
    async fn send(&self, event: &str) {
        self.ensure_connected().await;

        if let Some(stream) = &mut *self.ws_stream.lock().await {
            stream.send(Message::text(event)).await.unwrap();
        }
    }
}
