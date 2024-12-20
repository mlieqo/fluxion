use std::sync::Arc;

use fluxion::pipeline::Pipeline;
use fluxion::pipeline::runner::run_pipeline;
use fluxion::kafka::{consumer::KafkaConsumer, producer::KafkaProducer};

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
    pipeline.add_operator(fluxion::operators::Map::new(|data: &mut String| {
        format!("{} Processed", data)
    }));
    pipeline.add_operator(fluxion::operators::FlatMap::new(|data: &String| {
        data.split(" ").map(str::to_string).collect()
    }));
    let _ = run_pipeline(Arc::new(consumer), Arc::new(producer), pipeline, 100).await;
}
