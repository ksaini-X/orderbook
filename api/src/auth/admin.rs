use axum::{extract::FromRequestParts, http::StatusCode};
use uuid::Uuid;
use axum_extra::headers::{Authorization, authorization::Bearer};
use axum_extra::TypedHeader;pub struct AuthUser {
    pub user_id: Uuid,
}

impl<S> FromRequestParts<S> for AuthUser {
