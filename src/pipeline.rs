use std::sync::Arc;

use crate::message::Message;
use crate::{
    message::{FromBytes, ToBytes},
    operators::Operator,
};

pub mod input;
pub mod output;
pub mod runner;

pub struct Pipeline<T>
where
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    operators: Vec<Arc<dyn Operator<T>>>,
}

impl<T> Default for Pipeline<T>
where
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Pipeline<T>
where
    T: Send + Sync + FromBytes + ToBytes<T>,
{
    pub fn new() -> Self {
        Pipeline {
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
}
