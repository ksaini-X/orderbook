use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateMarketRequestData {
    name: String,
    created_at: DateTime<Utc>,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct CreateMarketResponseData {
    market_id: Uuid,
}

pub async fn create_market(Json(payload): Json<CreateMarketRequestData>) {}
