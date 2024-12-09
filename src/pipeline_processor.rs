// src/pipeline_processor.rs

// use tokio::sync::mpsc;
// use tokio::task;
// // use crate::{produce_to_kafka, Pipeline, KafkaConsumer, create_producer};
// use crate::pipeline::Pipeline;
// use crate::kafka::{producer::{produce_to_kafka, create_producer}, consumer::KafkaConsumer};
// use rdkafka::error::KafkaError;
// use std::sync::Arc;
// use tokio::signal;
// use tracing::{info, error};


// pub async fn run_pipeline(pipeline: Pipeline, input_topic: &str, output_topic: &str) -> Result<(), KafkaError> {
//     let consumer = KafkaConsumer::new("localhost", 9092)?;
//     let producer = create_producer("localhost", 9092);
//     let pipeline = Arc::new(pipeline);
//     let (tx, rx) = mpsc::channel::<String>(100);

//     // Clone Arc for producer_task if needed
//     let producer_clone = producer.clone();

//     // Spawn a task to consume messages and send them to the channel
//     let consumer_task = {
//         let tx = tx.clone();
//         let consumer = consumer;
//         task::spawn(async move {
//             if let Err(e) = consumer.consume(input_topic, tx).await {
//                 error!("Consumer encountered an error: {:?}", e);
//             }
//         })
//     };

//     // Spawn a task to process messages and produce them
//     let processor_task = {
//         let pipeline = Arc::clone(&pipeline);
//         let producer = producer.clone();
//         task::spawn(async move {
//             while let Some(message) = rx.recv().await {
//                 if let Some(processed) = pipeline.process(message) {
//                     if let Err(e) = produce_to_kafka(&producer, output_topic, &processed).await {
//                         error!("Failed to produce message: {:?}", e);
//                     }
//                 }
//             }
//             info!("Processor task has finished processing all messages.");
//         })
//     };

//     // Spawn a task to listen for shutdown signals
//     let shutdown_task = {
//         task::spawn(async move {
//             signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
//             info!("Shutdown signal received.");
//         })
//     };

//     // Await all tasks concurrently
//     consumer_task.await.unwrap();
//     processor_task.await.unwrap();

//     // Optionally, perform any cleanup here

//     Ok(())
// }
