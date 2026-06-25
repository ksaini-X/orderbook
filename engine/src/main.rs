use crate::engine::Engine;
use shared::{
    constants::{self, TOPIC_CANCELS, TOPIC_FILLS, TOPIC_MARKETS, TOPIC_ORDERS},
    kafka::consumer::KafkaConsumer,
};

pub mod book;
pub mod market_task;
pub mod messages;

pub mod engine;
#[tokio::main]
async fn main() {
    let engine = Engine::new();
    let kafka_broker = "";
    let topics = [TOPIC_CANCELS, TOPIC_FILLS, TOPIC_ORDERS, TOPIC_MARKETS];

    let consumer = KafkaConsumer::new(kafka_broker, "group_id", topics);

    loop {
        let message = consumer.poll().await;
    }
}
