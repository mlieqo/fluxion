use std::sync::Arc;

use crate::message::Message;
use crate::operators::OperatorResult;
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

    pub async fn process(&self, message: Message<T>) -> Option<Vec<Message<T>>> {
        let mut current_messages = vec![message];

        for op in &self.operators {
            let mut next_messages = Vec::new();

            for msg in current_messages {

                match op.process(msg) {
                    OperatorResult::Single(m) => next_messages.push(m),
                    OperatorResult::Multiple(m) => next_messages = m,
                    OperatorResult::Empty => return None,
                }
            }
            current_messages = next_messages;
        }
        Some(current_messages)
    }
}
