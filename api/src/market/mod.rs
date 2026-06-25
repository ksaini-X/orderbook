use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, post},
};

use crate::{AppState, market::create_market::create_market};

pub mod create_market;
pub mod delete_market;
pub mod pause_market;

pub fn market_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_market))
        .route("/", delete(create_market))
        .route("/", post(create_market))
}
