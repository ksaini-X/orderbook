use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
pub struct KafkaProducer {
    inner: FutureProducer,
}
impl KafkaProducer {
    pub fn new(broker: &str) -> Self {
        let inner = ClientConfig::new()
            .set("bootstrap.servers", broker)
            .create()
            .expect("Failed to create producer");
        Self { inner }
    }

    pub async fn publish<T: serde::Serialize>(&self, topic: &str, key: &str, payload: &T) {
        let payload = serde_json::to_string(payload).unwrap();
        self.inner
            .send(
                FutureRecord::to(topic).key(key).payload(&payload),
                std::time::Duration::from_secs(5),
            )
            .await
            .ok();
    }
}
