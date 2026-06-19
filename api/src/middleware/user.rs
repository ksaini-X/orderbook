use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::middleware::jwt::verify_jwt;

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_headers = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_headers
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(claims.role);
    req.extensions_mut().insert(claims.user_id);

    Ok(next.run(req).await)
}
