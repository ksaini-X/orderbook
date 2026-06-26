use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
pub enum EngineIncomingMessage {
    CreateMarket {
        client_id: Uuid,
        user_id: Uuid,
        name: String,
    },
    DeleteMarket {
        client_id: Uuid,
        user_id: Uuid,
        market_id: Uuid,
    },
    PauseMarket {
        client_id: Uuid,
        user_id: Uuid,
        market_id: Uuid,
    },
    PlaceOrder,
    CancelOrder,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum EngineOutGoingMessage {
    MarketCreated {
        market_id: Uuid,
        creater: Uuid,
        created_at: DateTime<Utc>,
    },
    MarketPaused {
        client_id: Uuid,
        market_id: Uuid,
        user_id: Uuid,
        deleted: bool,
        timestamp: DateTime<Utc>,
    },
    MarketDeleted {
        client_id: Uuid,
        market_id: Uuid,
        user_id: Uuid,
        deleted: bool,
        timestamp: DateTime<Utc>,
    },
    OrderPlaced {},
    OrderCancelled {},
}
