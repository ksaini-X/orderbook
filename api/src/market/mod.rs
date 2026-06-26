use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, post, put},
};

use crate::{
    AppState,
    market::{
        create_market::create_market, delete_market::delete_market, pause_market::pause_market,
    },
};

pub mod create_market;
pub mod delete_market;
pub mod pause_market;

pub fn market_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_market))
        .route("/", put(pause_market))
        .route("/", delete(delete_market))
}
