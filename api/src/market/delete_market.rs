use std::time::Duration;

use axum::{Extension, Json, extract::State};
use chrono::{Date, DateTime, Utc};
use futures::StreamExt;
use redis::Msg;
use serde::{Deserialize, Serialize};
use shared::constants::TOPIC_MARKETS;
use uuid::Uuid;

use crate::{AppState, error::APIError, middleware::jwt::Role};

#[derive(Serialize, Debug, Deserialize)]
pub struct DeleteMarketEvent {
    client_id: Uuid,
    market_id: Uuid,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct DeleteMarketRequestData {
    market_id: Uuid,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct DeleteMarketResponseData {
    client_id: Uuid,
    market_id: Uuid,
    user_id: Uuid,
    deleted: bool,
    timestamp: DateTime<Utc>,
}

pub async fn delete_market(
    Extension(role): Extension<Role>,
    State(state): State<AppState>,
    Json(payload): Json<DeleteMarketRequestData>,
) -> Result<Json<DeleteMarketResponseData>, APIError> {
    match role {
        Role::Admin => {
            let client_id = Uuid::new_v4();
            state
                .kafka_producer
                .publish(
                    TOPIC_MARKETS,
                    &"delete".to_string(),
                    &DeleteMarketEvent {
                        client_id,
                        market_id: payload.market_id,
                    },
                )
                .await;
            let mut pubsub = state.redis.get_async_pubsub().await.unwrap();

            pubsub
                .subscribe(&format!("market:deleted:{}", client_id))
                .await
                .unwrap();

            let mut stream = pubsub.on_message();

            let mut msg = tokio::time::timeout(Duration::from_secs(5), stream.next()).await;

            match msg {
                Ok(Some(m)) => {
                    let payload: String = m.get_payload().unwrap();
                    let data: DeleteMarketResponseData = serde_json::from_str(&payload).unwrap();
                    Ok(Json(data))
                }
                _ => {
                    return Err(APIError::ServiceUnavailable);
                }
            }
        }
        Role::User => return Err(APIError::Unauthorized),
    }
}
