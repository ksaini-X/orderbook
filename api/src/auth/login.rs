use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::APIError,
    middleware::{jwt::generate_jwt, user},
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct LoginRequestData {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct LoginResponseData {
    jwt: String,
}

pub async fn login(
    Json(payload): Json<LoginRequestData>,
) -> Result<Json<LoginResponseData>, APIError> {
    //TODO: DB call to get user
    let user_id = Uuid::new_v4();
    let role = crate::middleware::jwt::Role::Admin;
    let jwt = generate_jwt(user_id, role)?;
    Ok(Json(LoginResponseData { jwt }))
}
