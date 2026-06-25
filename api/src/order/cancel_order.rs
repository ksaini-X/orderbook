use std::time::Duration;

use axum::{Extension, Json, extract::State};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::constants::TOPIC_CANCELS;
use tokio::time::timeout;
use uuid::Uuid;

use crate::{AppState, error::APIError, market::create_market::CreateMarketResponseData};
#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrderRequestData {
    order_id: Uuid,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrderResponseData {
    order_id: Uuid,
    timestamp: DateTime<Utc>,
    filled_quantity: Decimal,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrderEvent {
    client_id: Uuid,
    order_id: Uuid,
}
pub async fn cancel_order(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<CancelOrderRequestData>,
) -> Result<Json<CancelOrderResponseData>, APIError> {
    let client_id = Uuid::new_v4();
    let payload = CancelOrderEvent {
        client_id,
        order_id: payload.order_id,
    };
    state
        .kafka_producer
        .publish(TOPIC_CANCELS, &"cancel_order".to_string(), &payload)
        .await;

    let mut pubsub = state.redis.get_async_pubsub().await.unwrap();

    pubsub
        .subscribe(format!("order:cancel:{}", client_id))
        .await
        .unwrap();

    let mut stream = pubsub.on_message();
    let msg = timeout(Duration::from_secs(5), stream.next()).await;

    match msg {
        Ok(Some(m)) => {
            let data = m.get_payload().unwrap();
            let payload: CancelOrderResponseData = serde_json::from_str(data).unwrap();
            Ok(Json(payload))
        }
        _ => Err(APIError::ServiceUnavailable),
    }
}
