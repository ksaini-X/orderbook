use std::{sync::Arc, time::Duration};

use crate::{AppState, error::APIError, middleware::jwt::Role};
use axum::{Extension, Json, extract::State};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use shared::constants::TOPIC_MARKETS;
use tokio::time::timeout;

use uuid::Uuid;
#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarketRequestData {
    market_id: Uuid,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarketResponseData {
    client_id: Uuid,
    market_id: Uuid,
    user_id: Uuid,
    paused: bool,
    timestamp: DateTime<Utc>,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarketEvent {
    client_id: Uuid,
    market_id: Uuid,
    user_id: Uuid,
}

pub async fn pause_market(
    Extension(role): Extension<Role>,
    Extension(user_id): Extension<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PauseMarketRequestData>,
) -> Result<Json<PauseMarketResponseData>, APIError> {
    match role {
        Role::Admin => {
            let client_id = Uuid::new_v4();
            state
                .kafka_producer
                .publish(
                    TOPIC_MARKETS,
                    &"pause".to_string(),
                    &PauseMarketEvent {
                        client_id,
                        market_id: payload.market_id,
                        user_id,
                    },
                )
                .await;
            let mut pubsub = state.redis.get_async_pubsub().await.unwrap();
            pubsub
                .subscribe(format!("market:paused:{}", client_id))
                .await
                .unwrap();
            let mut stream = pubsub.on_message();
            let mut msg =
                tokio::time::timeout(std::time::Duration::from_secs(5), stream.next()).await;
            match msg {
                Ok(Some(m)) => {
                    let payload: String = m.get_payload().unwrap();
                    let data: PauseMarketResponseData = serde_json::from_str(&payload).unwrap();
                    Ok(Json(data))
                }
                _ => Err(APIError::ServiceUnavailable),
            }
        }
        Role::User => {
            return Err(APIError::Unauthorized);
        }
    }
}
