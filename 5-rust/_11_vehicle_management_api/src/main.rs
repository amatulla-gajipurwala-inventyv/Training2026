mod handlers;
mod models;
mod routes;
mod state;

use state::AppState;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::net::TcpListener;
use tokio;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let state = AppState {
        db: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = routes::create_routes(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
