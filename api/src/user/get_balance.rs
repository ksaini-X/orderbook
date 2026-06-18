use rust_decimal::Decimal;

pub struct GetBalanceResponse {
    locked_balance: Decimal,
    available_balance: Decimal,
}

pub async fn get_balance() {}
