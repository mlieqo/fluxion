use std::time::Duration;

use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tracing::{error, info};

use crate::message::{FromBytes, ToBytes};
use crate::pipeline::output::OutputSource;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(host: &str, port: i16, topic: &str) -> Result<Self, KafkaError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", format!("{host}:{port}")) // Kafka server address
            .create()?;
        Ok(Self {
            producer,
            topic: topic.to_owned(),
        })
    }
}

#[async_trait]
impl<T> OutputSource<T> for KafkaProducer
where
    T: Send + Sync + FromBytes + ToBytes<T> + 'static,
{
    async fn produce(
        &self,
        payload: &crate::message::Message<T>,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        let payload_bytes = &T::to_bytes(payload.get_data());
        let record: FutureRecord<'_, (), Vec<u8>> =
            FutureRecord::<(), Vec<u8>>::to(self.topic.as_str()).payload(payload_bytes);

        // Send the record asynchronously and await the result
        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(delivery) => {
                // Delivery successful: (partition, offset)
                info!(
                    "Message delivered to partition {} at offset {}",
                    delivery.0, delivery.1
                );
                Ok(())
            }
            Err((e, _)) => {
                // Error occurred
                error!("Failed to deliver message: {}", e);
                todo!()
            }
        }
    }
}
