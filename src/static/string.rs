use crate::{HSerde, HSerdeError};

impl HSerde for String {

    fn to_bytes_internal(&self, result: &mut Vec<u8>) {
        self.len().to_bytes_internal(result);
        self.as_bytes().iter().map(|n| n.to_bytes_internal(result));
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.len() + 8);
        self.to_bytes_internal(&mut result);

        result
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {
        let (length, curr_index) = u32::from_bytes_internal(bytes, index)?;
        let length = length as usize;

        if curr_index + length > bytes.len() {
            return Err(HSerdeError::IndexError);
        }

        let result = match String::from_utf8(bytes[curr_index..curr_index + length].to_vec()) {
            Ok(s) => s,
            Err(_) => {return Err(HSerdeError::Utf8Error)}
        };

        Ok((result, curr_index + length))
    }

}