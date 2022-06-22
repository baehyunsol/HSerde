use crate::{HSerde, HSerdeError};

impl<T: HSerde> HSerde for Vec<T> {

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.len() + 1);

        result.push((self.len() as u32).to_bytes());

        for n in self.iter() {
            result.push(n.to_bytes());
        }

        result.concat()
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