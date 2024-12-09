
use fluxion::kafka::consumer::KafkaConsumer;
use tracing_subscriber;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let consumer = KafkaConsumer::new("localhost", 9092, "test-topic", "example-consumer-group", "latest").expect("bitch cannot do that");
    // let (tx, _rx) = tokio::sync::mpsc::channel::<Message<String>>(100);
  
    // let mut pipeline = Pipeline::new(consumer);

    // // Add a filter operator to retain only messages containing "Kafka".
    // pipeline.add_operator(Filter::new(|data: &String| data.contains("Kafka")));

    // // Add a map operator to append " Processed" to each message.
    // pipeline.add_operator(Map::new(|data: &String| format!("{} Processed", data)));

    // // Process a message through the pipeline.
    // pipeline.run().await
}