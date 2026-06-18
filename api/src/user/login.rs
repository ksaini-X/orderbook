use rust_decimal::Decimal;
use shared::engine::order::Order;
use uuid::Uuid;

pub struct LoginRequestData {
    email: String,
    password: String,
}
pub struct LoginResponseData {
    user_id: Uuid,
}
pub async fn login() {}
