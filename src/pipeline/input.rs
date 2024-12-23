use std::error::Error;

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use crate::message::{FromBytes, Message, ToBytes};

#[async_trait]
pub trait InputSource<T>: Send + Sync
where
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    async fn consume(&self, tx: &Sender<Message<T>>) -> Result<(), Box<dyn Error + Send>>;
}
