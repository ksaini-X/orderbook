use rust_decimal::Decimal;
use uuid::Uuid;
pub struct CreateOrderRequestData {
    price: Decimal,
    quantity: Decimal,
    market_id: Uuid,
}

pub struct CreateOrderResponseData {
    order_id: Uuid,
}

pub async fn create_order() {}
