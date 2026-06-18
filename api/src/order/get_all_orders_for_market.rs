use shared::engine::order::Order;
use uuid::Uuid;
pub struct GetAllOrderForMarketRequestData {
    market_id: Uuid,
}

pub struct GetAllOrderForMarketResponseData {
    orders: Vec<Order>,
}

pub async fn get_all_orders_for_market() {}
