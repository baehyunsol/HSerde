use crate::{HSerde, HSerdeError};

impl<T: HSerde> HSerde for Vec<T> {

    fn to_bytes_internal(&self, result: &mut Vec<u8>) {
        self.len().to_bytes_internal(result);

        for n in self.iter() {
            n.to_bytes_internal(result);
        }

    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.len() * 4);
        self.to_bytes_internal(&mut result);

        result
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {

        let (length, curr_index) = u32::from_bytes_internal(bytes, index)?;
        let mut last_index = curr_index;
        let mut result = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let (element, curr_index) = T::from_bytes_internal(bytes, last_index)?;
            result.push(element);
            last_index = curr_index;
        }

        Ok((result, last_index))
    }

}