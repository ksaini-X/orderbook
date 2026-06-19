use serde::{Deserialize, Serialize};
use shared::engine::order::Order;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum EngineMessage {
    PlaceOrder(Order),
}
