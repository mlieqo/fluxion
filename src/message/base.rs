use std::error::Error;

use super::{FromBytes, ToBytes};

impl FromBytes for String {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(String::from_utf8(bytes.to_vec())?)
    }
}

impl ToBytes<String> for String {
    fn to_bytes(message: &String) -> Vec<u8> {
        message.as_bytes().to_vec()
    }
}
