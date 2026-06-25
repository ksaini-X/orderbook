use crate::engine::Engine;
use redis::AsyncCommands;
use shared::{
    constants::{self, TOPIC_CANCELS, TOPIC_FILLS, TOPIC_MARKETS, TOPIC_ORDERS},
    engine::messages::{EngineIncomingMessage, EngineOutGoingMessage},
    kafka::consumer::KafkaConsumer,
};
use tokio::net::windows::named_pipe::PipeMode::Message;

pub mod book;
pub mod error;
pub mod market_task;

pub mod engine;
#[tokio::main]
async fn main() {
    let engine = Engine::new();
    let kafka_broker = "";
    let redis_client = redis::Client::open("params").unwrap();
    let pubsub = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    let topics = [TOPIC_CANCELS, TOPIC_FILLS, TOPIC_ORDERS, TOPIC_MARKETS];
    let consumer = KafkaConsumer::new(kafka_broker, "group_id", topics);

    loop {
        let message = consumer.poll::<EngineIncomingMessage>().await;
        match message {
            Some(msg) => match msg {
                EngineIncomingMessage::CreateMarket {
                    client_id,
                    user_id,
                    name,
                } => {
                    let (market_id, timestamp) = engine.create_market(name, user_id);
                    let payload = serde_json::to_string(&EngineOutGoingMessage::MarketCreated {
                        market_id,
                        creater: user_id,
                        created_at: timestamp,
                    })
                    .unwrap();
                    pubsub
                        .publish(format!("market:created:{}", client_id), payload)
                        .await;
                }
                EngineIncomingMessage::DeleteMarket {
                    client_id,
                    user_id,
                    market_id,
                } => {}
                EngineIncomingMessage::PauseMarket {
                    client_id,
                    user_id,
                    market_id,
                } => {}
                EngineIncomingMessage::PlaceOrder => {}
                EngineIncomingMessage::CancelOrder => {}
            },
            _ => {}
        }
    }
}
