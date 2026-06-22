use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use serde::de::{Deserialize, DeserializeOwned};

pub struct KafkaConsumer {
    inner: StreamConsumer,
}
impl KafkaConsumer {
    pub fn new(broker: &str, group_id: &str, topic: &str) -> Self {
        let inner: StreamConsumer = ClientConfig::new()
            .set("bootstrap.server", broker)
            .set("group_id", group_id)
            .set("auto.offset.reset", "earliest")
            .create()
            .expect("Failed to create consumer");
        inner.subscribe(&[topic]).unwrap();
        Self { inner }
    }

    pub async fn poll<T: DeserializeOwned>(&self) -> Option<T> {
        let msg = self.inner.recv().await;
        match msg {
            Err(e) => {
                println!("kafka error: {:?}", e);
                None
            }
            Ok(msg) => {
                let payload = msg.payload()?;
                serde_json::from_slice(payload).ok()
            }
        }
    }
}
