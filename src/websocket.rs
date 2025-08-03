use crate::events::UpdateEvent;
use axum::{
    extract::{ws::WebSocketUpgrade, ws::WebSocket, ws::Message, State},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

#[derive(Clone)]
pub struct ConnectionManager {
    pub broadcast_tx: broadcast::Sender<UpdateEvent>,
    pub connection_count: Arc<RwLock<usize>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        let connection_count = Arc::new(RwLock::new(0));

        Self {
            broadcast_tx,
            connection_count,
        }
    }

    pub async fn send_update(&self, event: UpdateEvent) {
        match self.broadcast_tx.send(event) {
            Ok(_) => {
                println!(
                    "Update broadcast to {} receivers",
                    self.broadcast_tx.receiver_count()
                );
            }
            Err(_) => {
                println!("No Websocket clients connected, updtate ignored");
            }
        }
    }

    async fn increment_connections(&self) {
        let mut count = self.connection_count.write().await;
        *count += 1;
        println!("WebSocket connection count increased to {}", *count);
    }

    async fn decrement_connections(&self) {
        let mut count = self.connection_count.write().await;
        *count -= 1;
        println!("WebSocket connection count decreased to {}", *count);
    }

    pub async fn get_connection_count(&self) -> usize {
        let count = self.connection_count.read().await;
        *count
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(connection_manager): State<ConnectionManager>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, connection_manager))
}

async fn handle_socket(socket: WebSocket, connection_manager: ConnectionManager) {
    println!("New websocket connnected");
    connection_manager.increment_connections().await;
    let mut rx = connection_manager.broadcast_tx.subscribe();
    let (mut sender, mut receiver) = socket.split();
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(_)) => {
                    println!("Received text message from client");
                }
                Ok(Message::Ping(data)) => {
                    println!("received ping, should send pong");
                }
                Ok(Message::Close(_)) => {
                    println!("Client closed conenction");
                    break;
                }
                Err(e) => {
                    println!("WebSocket eroor: {}", e);
                    break;
                }
                _ => {
                    // place holder: anything else
                }
            }
        }
    });

    let send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            match event.to_json() {
                Ok(json_str) => {
                    if sender.send(Message::Text(json_str.into())).await.is_err() {
                        println!("Failed to send message to client");
                        break;
                    }
                }
                Err(e) => {
                    println!("Failed to serialize event: {} ", e);
                }
            }
        }
    });

    tokio::select! {
        _ = recv_task => {
            println!("receive task complete");
        }
        _ = send_task => {
            println!("send task complete");
        }
    }

    connection_manager.decrement_connections().await;
    println!("Websocket conectino closed and cleaned up");
}

