use tokio::net::TcpListener;

use axum::{Json, Router, routing::get};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let router = Router::new().route("/health", get(health));

    axum::serve(listener, router).await.unwrap();
}

async fn health() -> Json<String> {
    Json("200".to_string())
}
