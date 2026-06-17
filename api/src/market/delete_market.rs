use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize)]
pub struct DeleteMarketRequestData {
    market_id: Uuid,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct DeleteMarketResponseData {}

pub async fn delete_market(Json(payload): Json<DeleteMarketRequestData>) {}
