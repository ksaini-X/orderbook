use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum BookError {
    InvalidBook,
    InvalidPrice,
    InvalidQuantity,
    InvalidOrderId,
    InvalidUserId,
}
#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for BookError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, msg) = match self {
            BookError::InvalidBook => (StatusCode::BAD_REQUEST, "Invalid book"),
            BookError::InvalidQuantity => (StatusCode::BAD_REQUEST, "Invalid quantity"),
            BookError::InvalidPrice => (StatusCode::BAD_REQUEST, "Invalid price"),
            BookError::InvalidOrderId => (StatusCode::BAD_REQUEST, "Invalid order ID"),
            BookError::InvalidUserId => (StatusCode::BAD_REQUEST, "Invalid user ID"),
        };

        (
            status_code,
            Json(ErrorBody {
                error: msg.to_string(),
            }),
        )
            .into_response()
    }
}
