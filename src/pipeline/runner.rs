use std::error::Error;
use std::sync::Arc;

use tokio::sync::mpsc::channel;
use tokio::task::spawn;
use tracing::{error, info};

use crate::message::{FromBytes, Message, ToBytes};
use crate::pipeline::Pipeline;

pub async fn run_pipeline<T>(
    input: Arc<dyn super::input::InputSource<T>>,
    output: Arc<dyn super::output::OutputSource<T>>,
    pipeline: Pipeline<T>,
    buffer_size: usize,
) -> Result<(), Box<dyn Error>>
where
    // tokio::spawn requires static lifetime (spawned task must not contain any references to data owned outside the task)
    T: Send + Sync + FromBytes + ToBytes<T> + 'static,
{
    let (tx, mut rx) = channel::<Message<T>>(buffer_size);

    let consumer_task = { spawn(async move { input.consume(&tx).await }) };

    let processor_task = {
        spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Some(messages) = pipeline.process(message).await {
                    for msg in messages {
                        if let Err(e) = output.produce(&msg).await {
                            error!("Failed to produce message: {:?}", e);
                        }
                    }
                }
            }
            info!("Processor task has finished processing all messages.");
        })
    };
    let _ = consumer_task.await.unwrap();
    processor_task.await.unwrap();

    Ok(())
}
