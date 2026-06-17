use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarletRequestData {
    market_id: Uuid,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarletResponseData {}

pub async fn pause_market(Json(payload): Json<PauseMarletRequestData>) {}
