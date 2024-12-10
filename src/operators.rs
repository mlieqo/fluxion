use std::marker::PhantomData;

use crate::message::{FromBytes, Message, ToBytes};

pub trait Operator<T>: Send + Sync
where
    T: Send + FromBytes + ToBytes<T>,
{
    fn process(&self, message: Message<T>) -> Option<Message<T>>;
}

// Filter Operator
pub struct Filter<F, T>
where
    F: Fn(&T) -> bool + Send + Sync,
    T: Send,
{
    predicate: F,
    _marker: PhantomData<T>, // Marker to associate T with the struct
}

impl<F, T> Filter<F, T>
where
    F: Fn(&T) -> bool + Send + Sync,
    T: Send,
{
    pub fn new(predicate: F) -> Self {
        Filter {
            predicate,
            _marker: PhantomData,
        }
    }
}

impl<F, T> Operator<T> for Filter<F, T>
where
    F: Fn(&T) -> bool + Send + Sync,
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    fn process(&self, message: Message<T>) -> Option<Message<T>> {
        if (self.predicate)(message.get_data()) {
            Some(message)
        } else {
            None
        }
    }
}

// Map Operator
pub struct Map<F, T>
where
    F: Fn(&T) -> T + Send + Sync,
    T: Send,
{
    mapper: F,
    _marker: PhantomData<T>, // Marker to associate T with the struct
}

impl<F, T> Map<F, T>
where
    F: Fn(&T) -> T + Send + Sync,
    T: Send,
{
    pub fn new(mapper: F) -> Self {
        Map {
            mapper,
            _marker: PhantomData,
        }
    }
}

impl<F, T> Operator<T> for Map<F, T>
where
    F: Fn(&T) -> T + Send + Sync,
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    fn process(&self, message: Message<T>) -> Option<Message<T>> {
        Some(Message::new((self.mapper)(message.get_data())))
    }
}
