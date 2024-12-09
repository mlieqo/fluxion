use tokio::time::{sleep, Duration};
use fluxion::kafka::producer::{produce_to_kafka, create_producer};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let producer = create_producer("localhost", 9092).unwrap();
    loop {
        if let Err(_) = produce_to_kafka(&producer, "test-topic", "fucking CUNT!").await {
            println!("Error sending message to kafka!");
        }; 
        sleep(Duration::from_secs(1)).await;
    }
}