use crate::{HSerde, HSerdeError};

impl HSerde for String {

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.len().to_bytes(),
            self.as_bytes().iter().map(|n| *n).collect::<Vec<u8>>()
        ].concat()
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