use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use pubsub::Subscriber;

use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum PubSubMessage {
    Subscribe { channel: String },
    Publish { channel: String, msg: String },
}

pub async fn handler(ws: WebSocketUpgrade, State(mut state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (sender, receiver) = socket.split();
    let sub = state.pubsub.write().await.new_subscriber();

    tokio::spawn(read(receiver, state, sub.sender));
    tokio::spawn(write(sender, sub.reciever));
}

async fn read(mut receiver: SplitStream<WebSocket>, state: Arc<AppState>, sender: Sender<String>) {
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            let text = msg.into_text().unwrap();

            let Ok(msg) = serde_json::from_str::<PubSubMessage>(&text) else {
                continue;
            };

            if let PubSubMessage::Publish { channel, msg } = &msg {
                publish(&channel, msg, state.clone()).await;
            };

            if let PubSubMessage::Subscribe { channel } = &msg {
                let mut pubsub = state.pubsub.write().await;
                pubsub.new_channel(&channel);
                pubsub
                    .subsribe_with_sender(&channel, sender.clone())
                    .unwrap();
            };
        }
    }
}

async fn write(mut sender: SplitSink<WebSocket, Message>, receiver: Receiver<String>) {
    while let Ok(msg) = receiver.recv() {
        sender.send(Message::Text(msg)).await.unwrap();
    }
}

async fn publish(channel: &str, msg: &str, state: Arc<AppState>) {
    state.pubsub.write().await.publish(channel, msg.into());
}
