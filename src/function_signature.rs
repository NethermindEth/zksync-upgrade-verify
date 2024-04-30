use ethers::types::Bytes;

// Decode function signature from bytes
pub trait FunctionSignature {
    // get the first four bytes
    fn sig(&self) -> [u8; 4];
}
// Implement decoding of function signature from bytes
impl FunctionSignature for Bytes {
    fn sig(&self) -> [u8; 4] {
        if self.len() < 4 {
            return [0; 4];
        }
        let bytes: &[u8] = self.as_ref();
        [bytes[0], bytes[1], bytes[2], bytes[3]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::Bytes;
    use std::str::FromStr;

    #[test]
    fn test_sig() {
        let bytes = Bytes::from_str("0x12345678ab09").unwrap();
        assert_eq!(bytes.sig(), [0x12, 0x34, 0x56, 0x78]);
    }
}
