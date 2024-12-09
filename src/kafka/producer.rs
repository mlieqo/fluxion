
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::KafkaError;
use std::time::Duration;
use tracing::{info, error};


pub struct KafkaProducer {
    producer: FutureProducer,
}


pub fn create_producer(host: &str, port: i16) -> Result<FutureProducer, KafkaError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", format!("{host}:{port}")) // Kafka server address
        .create()?;
    Ok(producer)
}


pub async fn produce_to_kafka(producer: &FutureProducer, topic: &str, payload: &str) -> Result<(i32, i64), KafkaError> {
    let record = FutureRecord::<(), _>::to(topic)
        .payload(payload);

    // Send the record asynchronously and await the result
    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => {
            // Delivery successful: (partition, offset)
            info!(
                "Message = '{}' delivered to partition {} at offset {}",
                payload, delivery.0, delivery.1
            );
            Ok(delivery)
        }
        Err((e, _)) => {
            // Error occurred
            error!("Failed to deliver message: {}", e);
            Err(e)
        }
    }
}