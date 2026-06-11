pub mod routes;
pub mod state;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{Json, Router, routing::get};
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::state::user::User;

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<RwLock<HashMap<Uuid, User>>>,
    //TODO : Add DB Pool
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        cache: Arc::new(RwLock::new(HashMap::<Uuid, User>::new())),
    };

    //TODO : fetch initial user state from DB

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let router = Router::new()
        .with_state(app_state.clone())
        .route("/health", get(health));

    axum::serve(listener, router).await.unwrap();
}

async fn health() -> Json<String> {
    Json("200".to_string())
}
