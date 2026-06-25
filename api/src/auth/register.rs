use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::APIError, middleware::jwt::generate_jwt};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterRequestData {
    email: String,
    password: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterResponseData {
    jwt: String,
    user_id: Uuid,
}

pub async fn login(
    Json(payload): Json<RegisterRequestData>,
) -> Result<Json<RegisterResponseData>, APIError> {
    //TODO: DB call to get user
    let user_id = Uuid::new_v4();
    let role = crate::middleware::jwt::Role::Admin;
    let jwt = generate_jwt(user_id, role)
        .map_err(|_| APIError::ServiceUnavailable)
        .unwrap();
    Ok(Json(RegisterResponseData { jwt, user_id }))
}
