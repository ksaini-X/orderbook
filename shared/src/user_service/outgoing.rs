use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BalanceError {
    UserNotFound,
    InsufficientFunds,
    InvalidAmount,
}
