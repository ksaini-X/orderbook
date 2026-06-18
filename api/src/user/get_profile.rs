use rust_decimal::Decimal;
use shared::engine::order::Order;

pub struct GetProfileResponse {
    locked_balance: Decimal,
    available_balance: Decimal,
    orders: Vec<Order>,
}

pub async fn get_profile() {}
