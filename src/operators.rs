use std::marker::PhantomData;

use crate::message::{FromBytes, Message, ToBytes};


pub enum OperatorResult<T>
where 
    T: Send + FromBytes + ToBytes<T>,
{
    Single(Message<T>),
    Multiple(Vec<Message<T>>),
    Empty
}


pub trait Operator<T>: Send + Sync
where
    T: Send + FromBytes + ToBytes<T>,
{
    fn process(&self, message: Message<T>) -> OperatorResult<T>;
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
    T: Send + Sync + FromBytes + ToBytes<T> + 'static,
{
    fn process(&self, message: Message<T>) -> OperatorResult<T> {
        if (self.predicate)(message.get_data()) {
            OperatorResult::Single(message)
        } else {
            OperatorResult::Empty
        }

    }
}

// Map Operator
pub struct Map<F, T>
where
    F: Fn(&mut T) -> T + Send + Sync,
    T: Send,
{
    mapper: F,
    _marker: PhantomData<T>, // Marker to associate T with the struct
}

impl<F, T> Map<F, T>
where
    F: Fn(&mut T) -> T + Send + Sync,
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
    F: Fn(&mut T) -> T + Send + Sync,
    T: Send + Sync + FromBytes + ToBytes<T> + 'static,
{
    fn process(&self, mut message: Message<T>) -> OperatorResult<T> {
        let result = (self.mapper)(message.get_data_mut());
        OperatorResult::Single(Message::new(result))
    }
}


// FlatMap Operator
pub struct FlatMap<F, T>
where
    F: Fn(&T) -> Vec<T> + Send + Sync,
    T: Send,
{
    mapper: F,
    _marker: PhantomData<T>, // Marker to associate T with the struct
}

impl<F, T> FlatMap<F, T>
where
    F: Fn(&T) -> Vec<T> + Send + Sync,
    T: Send,
{
    pub fn new(mapper: F) -> Self {
        FlatMap {
            mapper,
            _marker: PhantomData,
        }
    }
}

impl<F, T> Operator<T> for FlatMap<F, T>
where
    F: Fn(&T) -> Vec<T> + Send + Sync,
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    fn process(&self, message: Message<T>) -> OperatorResult<T> {
        let results = (self.mapper)(message.get_data());
        if results.is_empty() {
            OperatorResult::Empty
        } else {
            let msgs: Vec<Message<T>> = results.into_iter().map(Message::new).collect();
            OperatorResult::Multiple(msgs)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_filter_operator() {
        let predicate = |data: &String| data.contains("Little Prince");
        let filter = Filter::new(predicate);

        let mut test_str = String::from("Little Prince was here!");
        let message = Message::new(test_str.clone());
        match filter.process(message) {
            OperatorResult::Single(result) => {
                assert_eq!(*result.get_data(), test_str);
            }
            _ => panic!("Expected Single result"),
        }

        test_str = String::from("Little fella");
        let message = Message::new(test_str);
        match filter.process(message) {
            OperatorResult::Empty => {} // Expected
            _ => panic!("Expected Empty result"),
        }
    }

    #[test]
    fn test_map_operator() {
        let map = Map::new(|data: &mut String| { format!("{} test", data) });

        let message = Message::new(String::from("Map"));
        match map.process(message) {
            OperatorResult::Single(result) => {
                assert_eq!(*result.get_data(), String::from("Map test"));
            }
            _ => panic!("Expected Single result"),
        }
    }

    #[test]
    fn test_flat_map_operator() {
        let flat_map = FlatMap::new(
            |data: &String| data.split(" ").map(str::to_string).collect()
        );

        let message = Message::new(String::from("Little Prince"));
        match flat_map.process(message) {
            OperatorResult::Multiple(results) => {
                let expected = vec!["Little", "Prince"];
                assert_eq!(results[0].get_data(), expected[0]);
                assert_eq!(results[1].get_data(), expected[1]);
            }
            _ => panic!("Expected Multiple result"),
        }
    }
}
