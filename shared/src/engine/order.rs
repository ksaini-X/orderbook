use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Order {
    pub market_id: Uuid,
    pub user_id: Uuid,
    pub order_id: Uuid,
    pub price: Decimal,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub side: Side,
    pub status: Status,
    pub timestamp: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Status {
    Filled,
    PartiallyFilled,
    Cancelled,
    Pending,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fill {
    pub market_id: Uuid,
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,
    pub taker_user_id: Uuid,
    pub maker_user_id: Uuid,
    pub price: Decimal,
    pub quantity: Decimal,
    pub timestamp: DateTime<Utc>,
}
