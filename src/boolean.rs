use crate::{HSerde, HSerdeError};

impl HSerde for bool {

    fn to_bytes(&self) -> Vec<u8> {

        if *self {
            vec![1]
        }

        else {
            vec![0]
        }

    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(bool, usize), HSerdeError> {

        if index >= bytes.len() {
            Err(HSerdeError::IndexError)
        }

        else {
            Ok((bytes[index] != 0, index + 1))
        }

    }

}