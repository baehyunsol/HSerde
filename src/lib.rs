mod integer;
mod vector;
mod string;
mod boolean;
mod error;

#[cfg(test)]
mod testbench;

pub use error::HSerdeError;

pub trait HSerde {

    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> where Self: Sized;  // (deserialized, next_index)

    /// `bytes` is the serialized data generatd by the `to_bytes` method.
    /// `index` is the index where you begin searching
    fn from_bytes(bytes: &[u8], index: usize) -> Result<Self, HSerdeError> where Self: Sized {
        match Self::from_bytes_internal(bytes, index) {
            Ok((v, _)) => Ok(v),
            Err(e) => Err(e)
        }
    }

}