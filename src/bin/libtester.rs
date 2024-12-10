use std::sync::Arc;

use fluxion::pipeline::Pipeline;
use fluxion::{
    kafka::{consumer::KafkaConsumer, producer::KafkaProducer},
    pipeline::runner::run_pipeline,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let consumer = KafkaConsumer::new(
        "localhost",
        9092,
        "input-test-topic",
        "example-consumer-group",
        "latest",
    )
    .expect("Error creating consumer");
    let producer = KafkaProducer::new("localhost", 9092, "output-test-topic")
        .expect("Error creating producer");

    let mut pipeline = Pipeline::<String>::new();
    pipeline.add_operator(fluxion::operators::Filter::new(|data: &String| {
        data.contains("Kafka")
    }));
    pipeline.add_operator(fluxion::operators::Map::new(|data: &String| {
        format!("{} Processed", data)
    }));
    let _ = run_pipeline(Arc::new(consumer), Arc::new(producer), pipeline, 100).await;
}
