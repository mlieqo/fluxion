use crate::message::{FromBytes, Message};
use tokio::sync::mpsc::Sender;
use async_trait::async_trait;


#[async_trait]
pub trait InputSource<T>
where
    T: Send + FromBytes,
{
    async fn consume(&self, tx: &tokio::sync::mpsc::Sender<crate::message::Message<T>>) -> Result<(), Box<dyn std::error::Error + Send>>;
}