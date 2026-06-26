use std::time::Duration;

use crate::{AppState, error::APIError};
use axum::{Extension, Json, extract::State};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::constants::TOPIC_ORDERS;
use tokio::time::timeout;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrderRequestData {
    price: Decimal,
    quantity: Decimal,
    market_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrderResponseData {
    order_id: Uuid,
    timestamp: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrderEvent {
    price: Decimal,
    quantity: Decimal,
    market_id: Uuid,
    user_id: Uuid,
    client_id: Uuid,
}

pub async fn place_order(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<PlaceOrderRequestData>,
) -> Result<Json<PlaceOrderResponseData>, APIError> {
    let client_id = Uuid::new_v4();
    state
        .kafka_producer
        .publish(
            TOPIC_ORDERS,
            &"place_order".to_string(),
            &PlaceOrderEvent {
                market_id: payload.market_id,
                price: payload.price,
                quantity: payload.quantity,
                user_id,
                client_id,
            },
        )
        .await;

    let mut pubsub = state.redis.get_async_pubsub().await.unwrap();
    pubsub
        .subscribe(format!("order:place:{}", client_id))
        .await
        .unwrap();
    let mut stream = pubsub.on_message();
    let mut msg = timeout(Duration::from_secs(5), stream.next()).await;

    match msg {
        Ok(Some(m)) => {
            let data: String = m.get_payload().unwrap();
            let payload: PlaceOrderResponseData = serde_json::from_str(&data).unwrap();
            Ok(Json(payload))
        }
        _ => Err(APIError::ServiceUnavailable),
    }
}
