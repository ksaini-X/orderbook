use crate::{AppState, error::APIError, middleware::jwt::Role};
use axum::{Extension, Json, extract::State};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::constants::TOPIC_MARKETS;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize)]
pub struct CreateMarketRequestData {
    name: String,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct CreateMarketResponseData {
    market_id: Uuid,
    creater: Uuid,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct MarketEvent {
    client_id: Uuid,
    name: String,
    user_id: Uuid,
}

pub async fn create_market(
    Extension(user_id): Extension<Uuid>,
    Extension(role): Extension<Role>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMarketRequestData>,
) -> Result<Json<CreateMarketResponseData>, APIError> {
    match role {
        Role::Admin => {
            let client_id = Uuid::new_v4();

            state
                .kafka_producer
                .publish(
                    TOPIC_MARKETS,
                    &"create".to_string(),
                    &MarketEvent {
                        client_id,
                        user_id: user_id,
                        name: payload.name,
                    },
                )
                .await;

            let mut pubsub = state.redis.get_async_pubsub().await.unwrap();
            pubsub
                .subscribe(&format!("market:created:{}", client_id))
                .await
                .unwrap();

            let mut stream = pubsub.on_message();
            let msg = tokio::time::timeout(std::time::Duration::from_secs(5), stream.next()).await;

            match msg {
                Ok(Some(m)) => {
                    let payload: String = m.get_payload().unwrap();
                    let data: CreateMarketResponseData = serde_json::from_str(&payload).unwrap();
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
