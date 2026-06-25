use std::sync::Arc;

use axum::Router;
use redis::Client;
use shared::kafka::producer::KafkaProducer;
use tokio::net::TcpListener;

use crate::market::market_router;

pub mod auth;
pub mod error;
pub mod market;
pub mod middleware;
pub mod order;
pub mod user;

pub struct AppState {
    pub kafka_producer: KafkaProducer,
    pub redis: Client,
}
#[tokio::main]
async fn main() {
    let kafka_broker = "";
    let redis_client = redis::Client::open("params").unwrap();

    let shared_state = Arc::new(AppState {
        kafka_producer: KafkaProducer::new(kafka_broker),
        redis: redis_client,
    });

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let router = Router::new()
        .nest("/api/market", market_router())
        .with_state(shared_state);

    axum::serve(listener, router).await.unwrap()
}
