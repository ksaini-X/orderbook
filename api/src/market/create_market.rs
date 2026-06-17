use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize)]
pub struct CreatearketRequestData {
    name: String,
    created_at: DateTime<Utc>,
}

pub async fn create_market() {}
