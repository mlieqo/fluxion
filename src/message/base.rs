use std::error::Error;

use super::FromBytes;


impl FromBytes for String {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(String::from_utf8(bytes.to_vec())?)
    }
}

