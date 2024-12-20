use std::error::Error;

pub mod base;

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>>;
}

pub trait ToBytes<T> {
    fn to_bytes(message: &T) -> Vec<u8>;
}

pub struct Message<T>
where
    T: Send + FromBytes + ToBytes<T>,
{
    data: T,
}

impl<T> Message<T>
where
    T: Send + FromBytes + ToBytes<T>,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
