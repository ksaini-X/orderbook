use axum::{Router, routing::post};

use crate::{
    AppState,
    routes::{deduct_request::deduct, lock_request::lock, unlock_request::unlock},
};

pub mod deduct_request;
pub mod lock_request;
pub mod unlock_request;

pub fn user_service_router() -> Router<AppState> {
    Router::new()
        .route("/deduct", post(deduct))
        .route("/lock", post(lock))
        .route("/unlock", post(unlock))
}
