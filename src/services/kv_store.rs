use std::sync::Arc;

use axum::{extract::State, http::Response, Json};
use serde_json::{json, Value};

use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum KvStoreMessage {
    Get {
        namespace: String,
        key: String,
    },
    Store {
        namespace: String,
        key: String,
        value: String,
    },
}

impl KvStoreMessage {
    async fn handle(&self, state: Arc<AppState>) {}
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<KvStoreMessage>,
) -> Response<String> {
    match msg {
        KvStoreMessage::Get { namespace, key } => {
            let value = state.kv_store.read().await;

            if let Some(value) = value.get(&namespace, &key) {
                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(json!({"value": value}).to_string())
                    .unwrap()
            } else {
                Response::builder()
                    .status(404)
                    .header("Content-Type", "application/json")
                    .body(json!({"value": "null"}).to_string())
                    .unwrap()
            }
        }
        KvStoreMessage::Store {
            namespace,
            key,
            value,
        } => {
            let mut kv_store = state.kv_store.write().await;
            kv_store.store(&namespace, &key, value);
            Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(json!({ "success": true, "message": "Stored value." }).to_string())
                .unwrap()
        }
    }
}
