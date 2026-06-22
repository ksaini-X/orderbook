use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum APIError {
    Unauthorized,
}
#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, msg) = match self {
            APIError::Unauthorized => (StatusCode::BAD_REQUEST, "Invalid user role"),
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
