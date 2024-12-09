use input::InputSource;
use tokio::task::JoinError;

use crate::{message::FromBytes, operators::Operator};
use std::sync::Arc;
use crate::message::Message;
use tracing::{info, error};


pub mod input;


pub struct Pipeline<T>
where
    T: Send + Sync + FromBytes
{
    input: Arc<dyn InputSource<T>>,
    operators: Vec<Arc<dyn Operator<T>>>,
}

impl<T> Pipeline<T>
where T: Send + Sync + FromBytes
{
    pub fn new(input: impl InputSource<T> + 'static) -> Self {
        Pipeline {
            input: Arc::new(input),
            operators: Vec::new(),
        }
    }

    pub fn add_operator(&mut self, operator: impl Operator<T> + 'static) {
        self.operators.push(Arc::new(operator));
    }

    pub fn process(&self, message: Message<T>) -> Option<Message<T>> {
        let mut msg = message;
        for op in &self.operators {
            match op.process(msg) {
                Some(m) => msg = m,
                None => return None, // Message filtered out
            }
        }
        Some(msg)
    }

    pub async fn run(&self) -> Result<(), JoinError> {
        let (tx, rx) = tokio::sync::mpsc::channel::<crate::message::Message<T>>(100);
        
        let consumer_task = {
            tokio::task::spawn(async move {
                
                let consume_result = self.input.consume(&tx).await;
                consume_result
                // if let Err(e) = consume_result {
                    // error!("Consumer encountered an error: {:?}", e);
                // }
            })
        };
        consumer_task.await.map_err(|e| Box::new(e)).unwrap()

    // Spawn a task to process messages and produce them
    // let processor_task = {
    //     let pipeline = Arc::clone(&pipeline);
    //     let producer = producer.clone();
    //     task::spawn(async move {
    //         while let Some(message) = rx.recv().await {
    //             if let Some(processed) = pipeline.process(message) {
    //                 if let Err(e) = produce_to_kafka(&producer, output_topic, &processed).await {
    //                     error!("Failed to produce message: {:?}", e);
    //                 }
    //             }
    //         }
    //         info!("Processor task has finished processing all messages.");
    //     })
    // };

    //     self.input.consume(tx).await
    }    
}
