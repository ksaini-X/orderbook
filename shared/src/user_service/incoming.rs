use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct LockRequest {
    pub user_id: Uuid,
    pub amount: Decimal,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UnlockRequest {
    pub user_id: Uuid,
    pub amount: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DeductRequest {
    pub from: Uuid,
    pub to: Uuid,
    pub amount: Decimal,
}
