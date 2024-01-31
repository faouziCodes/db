use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use kv::KvStores;
use pubsub::PubSub;
use tokio::sync::RwLock;

pub mod services;

pub struct AppState {
    pubsub: RwLock<PubSub<String>>,
    kv_store: RwLock<KvStores<String>>,
}

pub async fn botter() {
    let app_state: AppState = AppState {
        pubsub: RwLock::new(PubSub::new()),
        kv_store: RwLock::new(KvStores::new()),
    };
    let shared_state = Arc::new(app_state);

    let app = Router::new()
        .route("/services/pubsub", get(services::pubsub::handler))
        .route("/services/kv_store", post(services::kv_store::handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
