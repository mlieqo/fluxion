use std::error::Error;

use async_trait::async_trait;

use crate::message::{FromBytes, Message, ToBytes};

#[async_trait]
pub trait OutputSource<T>: Send + Sync
where
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    async fn produce(&self, payload: &Message<T>) -> Result<(), Box<dyn Error + Send>>;
}
