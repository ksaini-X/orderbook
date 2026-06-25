use std::time::Duration;

use crate::{AppState, error::APIError, middleware::jwt::Role};
use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use shared::constants::TOPIC_MARKETS;
use tokio::time::timeout;
use tower_http::classify::GrpcCode::Ok;
use uuid::Uuid;
#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarletRequestData {
    market_id: Uuid,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarketResponseData {
    client_id: Uuid,
    market_id: Uuid,
    user_id: Uuid,
    paused: bool,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct PauseMarketEvent {
    client_id: Uuid,
    market_id: Uuid,
}

pub async fn pause_market(
    Extension(role): Extension(Role),
    State(state): State<AppState>,
    Json(payload): Json<PauseMarletRequestData>,
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
                    },
                )
                .await;
            let pubsub = state.redis.get_async_pubsub().await.unwrap();
            pubsub
                .subscribe(format!("market:paused:{}", client_id))
                .await;
            let mut stream = pubsub.on_message();
            let msg = tokio::time::timeout(std::time::Duration::from_secs(5), stream.next()).await;
            match msg {
                Ok(Some(m)) => {
                    let payload = m.get_payload().unwrap();
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
