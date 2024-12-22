use std::error::Error;

pub mod base;

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>>;
}

pub trait ToBytes<T> {
    fn to_bytes(message: &T) -> Vec<u8>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Message<T> {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct MockData {
        value: String,
    }

    impl FromBytes for MockData {
        fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
            let value = String::from_utf8(bytes.to_vec())?;
            Ok(MockData { value })
        }
    }

    impl ToBytes<MockData> for MockData {
        fn to_bytes(message: &MockData) -> Vec<u8> {
            message.value.as_bytes().to_vec()
        }
    }

    #[test]
    fn test_message_new_and_get_data() {
        let data = MockData {
            value: "test_data".to_string(),
        };
        let message = Message::new(data.clone());

        assert_eq!(message.get_data(), &data);
    }

    #[test]
    fn test_message_get_data_mut() {
        let data = MockData {
            value: "initial_data".to_string(),
        };
        let mut message = Message::new(data);

        message.get_data_mut().value = "modified_data".to_string();

        assert_eq!(message.get_data().value, "modified_data");
    }

    #[test]
    fn test_from_bytes() {
        let bytes = b"mock_data";
        let data = MockData::from_bytes(bytes).expect("Failed to deserialize from bytes");

        assert_eq!(data, MockData {
            value: "mock_data".to_string(),
        });
    }

    #[test]
    fn test_to_bytes() {
        let data = MockData {
            value: "mock_data".to_string(),
        };
        let bytes = MockData::to_bytes(&data);

        assert_eq!(bytes, b"mock_data");
    }

    #[test]
    fn test_message_with_bytes_conversion() {
        let bytes = b"end_to_end";
        let data = MockData::from_bytes(bytes).expect("Failed to deserialize");

        let message = Message::new(data);
        let serialized = MockData::to_bytes(message.get_data());

        assert_eq!(serialized, bytes);
    }
}
