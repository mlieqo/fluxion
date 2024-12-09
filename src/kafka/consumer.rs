use std::fmt::Debug;

use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::error::KafkaError;
use tracing::{info, error};
use tokio::sync::mpsc::Sender;
use async_trait::async_trait;


use crate::message::{FromBytes};
use crate::pipeline::input::InputSource;



pub struct KafkaConsumer {
    consumer: StreamConsumer,
    topic: String,
}


impl KafkaConsumer {
    pub fn new(host: &str, port: i16, topic: &str, group: &str, offset: &str) -> Result<Self, KafkaError> {
        Ok(
            Self {
                consumer: ClientConfig::new()
                .set("bootstrap.servers", format!("{host}:{port}"))
                .set("group.id", group)   // Consumer group ID
                .set("auto.offset.reset", offset)        // Start from the beginning if no offset is found
                .create()?,
                topic: topic.to_owned(),
            }
        )
    }

    fn is_subscribed(&self, topic: &str) -> Result<bool, KafkaError> {
        let subscribed_topics = self.consumer.subscription()?;
        // I am getting panic when this count is zero and we check for elements_for_topic
        if subscribed_topics.count() == 0 {
            return Ok(false);
        }
        Ok(!subscribed_topics.elements_for_topic(topic).is_empty())
    }

    async fn consume_internal<T>(&self, tx: &tokio::sync::mpsc::Sender<crate::message::Message<T>>) -> Result<(), KafkaError>
    where
        T: Send + FromBytes + Debug
    {
        if !self.is_subscribed(self.topic.as_str())? {
            self.consumer.subscribe(&[self.topic.as_str()])?;
        }
        let mut stream = self.consumer.stream();

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => {
                    match message.payload_view::<[u8]>() {
                        Some(Ok(payload)) => {

                            match T::from_bytes(payload) {
                                Ok(deserialized) => {
                                    let kafka_msg = crate::message::Message::new(deserialized);
                                    info!("Received data {:?}", &kafka_msg.get_data());
                                    if tx.send(kafka_msg).await.is_err() {
                                        error!("Receiver dropped, stopping consumer.");
                                        break;
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to deserialize message: {:?}", e);

                                }
                            }
                        },
                        Some(Err(_)) => {},
                        None => {},
                    }
                },
                Err(e) => error!("Error while reading from Kafka: {:?}", e),
            }
        }
        Ok(())
    }
    
}


#[async_trait]
impl<T> InputSource<T> for KafkaConsumer
where
    T: Send + FromBytes + Debug + 'static,
{
    async fn consume(&self, tx: &Sender<crate::message::Message<T>>) -> Result<(), Box<dyn std::error::Error + Send>> {
        self.consume_internal(tx).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
    }
}
