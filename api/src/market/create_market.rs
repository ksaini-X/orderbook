use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::middleware::jwt::Role;

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateMarketRequestData {
    name: String,
    created_at: DateTime<Utc>,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct CreateMarketResponseData {
    market_id: Uuid,
}

pub async fn create_market(
    Extension(user_id): Extension<Uuid>,
    Extension(role): Extension<Role>,
    Json(payload): Json<CreateMarketRequestData>,
) {
}
