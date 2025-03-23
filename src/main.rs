mod db;
mod store;
mod models;

use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use db::get_db_pool;
use store::{set_value, get_value, delete_value};

#[derive(Deserialize)]
struct KeyValueRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct KeyValueResponse {
    key: String,
    value: String,
}

#[tokio::main]
async fn main() {
    let pool = get_db_pool().await;
    let shared_pool = Arc::new(pool);

    let app = Router::new()
        .route("/", get(|| async { "🚀 Key-Value Store API is running!" }))
        .route("/set", post(set_handler))
        .route("/get/{key}", get(get_handler))
        .route("/delete/{key}", delete(delete_handler))
        .with_state(shared_pool); // Axum 0.8 uses `with_state()` instead of `.layer()`

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn set_handler(
    State(pool): State<Arc<MySqlPool>>,
    Json(payload): Json<KeyValueRequest>,
) -> Json<&'static str> {
    if let Err(err) = set_value(&pool, &payload.key, &payload.value).await {
        eprintln!("Error setting value: {}", err);
        return Json("Failed to set key");
    }
    Json("Key set successfully")
}

async fn get_handler(
    State(pool): State<Arc<MySqlPool>>,
    Path(key): Path<String>,
) -> Json<Option<KeyValueResponse>> {
    match get_value(&pool, &key).await {
        Ok(Some(value)) => Json(Some(KeyValueResponse { key, value })),
        Ok(None) => Json(None),
        Err(err) => {
            eprintln!("Error retrieving value: {}", err);
            Json(None)
        }
    }
}

async fn delete_handler(
    State(pool): State<Arc<MySqlPool>>,
    Path(key): Path<String>,
) -> Json<&'static str> {
    if let Err(err) = delete_value(&pool, &key).await {
        eprintln!("Error deleting key: {}", err);
        return Json("Failed to delete key");
    }
    Json("Key deleted successfully")
}
