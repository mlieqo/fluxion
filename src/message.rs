use std::error::Error;


pub mod base;


pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>>;
}


pub struct Message<T>
where
    T: Send + FromBytes
{
    data: T
}


impl<T> Message<T> 
where 
    T: Send + FromBytes
{
    pub fn new(data: T) -> Self {
        Self {
            data
        }
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }
}
