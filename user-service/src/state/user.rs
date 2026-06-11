use rust_decimal::{Decimal, dec};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct User {
    pub user_id: Uuid,
    pub locked_balance: Decimal,    // locked balance in open orders
    pub available_balance: Decimal, // available balance to spend
}

impl User {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            locked_balance: dec!(0),
            available_balance: dec!(0),
        }
    }
}
