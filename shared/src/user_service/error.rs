use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BalanceError {
    UserNotFound,
    InsufficientFunds,
    InvalidAmount,
    StateReadingFailed,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for BalanceError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            BalanceError::UserNotFound => (
                StatusCode::NOT_FOUND, // 404
                "user not found",
            ),
            BalanceError::InsufficientFunds => (
                StatusCode::UNPROCESSABLE_ENTITY, // 422
                "insufficient funds",
            ),
            BalanceError::InvalidAmount => (
                StatusCode::BAD_REQUEST, // 400
                "invalid amount",
            ),
            BalanceError::StateReadingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR, // 400
                "State reading ailed",
            ),
        };

        (
            status,
            Json(ErrorBody {
                error: message.to_string(),
            }),
        )
            .into_response()
    }
}
