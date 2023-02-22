use crate::{HSerde, HSerdeError};

impl<T: HSerde, U: HSerde> HSerde for (T, U) {

    fn to_bytes_internal(&self, result: &mut Vec<u8>) {
        self.0.to_bytes_internal(result);
        self.1.to_bytes_internal(result);
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.0.to_bytes(),
            self.1.to_bytes()
        ].concat()
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {
        let (e0, ind0) = T::from_bytes_internal(bytes, index)?;
        let (e1, ind1) = U::from_bytes_internal(bytes, ind0)?;

        Ok(((e0, e1), ind1))
    }

}

impl<T: HSerde, U: HSerde, V: HSerde> HSerde for (T, U, V) {

    fn to_bytes_internal(&self, result: &mut Vec<u8>) {
        self.0.to_bytes_internal(result);
        self.1.to_bytes_internal(result);
        self.2.to_bytes_internal(result);
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.0.to_bytes(),
            self.1.to_bytes(),
            self.2.to_bytes(),
        ].concat()
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {
        let (e0, ind0) = T::from_bytes_internal(bytes, index)?;
        let (e1, ind1) = U::from_bytes_internal(bytes, ind0)?;
        let (e2, ind2) = V::from_bytes_internal(bytes, ind1)?;

        Ok(((e0, e1, e2), ind2))
    }

}

impl<T: HSerde, U: HSerde, V: HSerde, W: HSerde> HSerde for (T, U, V, W) {

    fn to_bytes_internal(&self, result: &mut Vec<u8>) {
        self.0.to_bytes_internal(result);
        self.1.to_bytes_internal(result);
        self.2.to_bytes_internal(result);
        self.3.to_bytes_internal(result);
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.0.to_bytes(),
            self.1.to_bytes(),
            self.2.to_bytes(),
            self.3.to_bytes(),
        ].concat()
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {
        let (e0, ind0) = T::from_bytes_internal(bytes, index)?;
        let (e1, ind1) = U::from_bytes_internal(bytes, ind0)?;
        let (e2, ind2) = V::from_bytes_internal(bytes, ind1)?;
        let (e3, ind3) = W::from_bytes_internal(bytes, ind2)?;

        Ok(((e0, e1, e2, e3), ind3))
    }

}

impl<T: HSerde, U: HSerde, V: HSerde, W: HSerde, X: HSerde> HSerde for (T, U, V, W, X) {

    fn to_bytes_internal(&self, result: &mut Vec<u8>) {
        self.0.to_bytes_internal(result);
        self.1.to_bytes_internal(result);
        self.2.to_bytes_internal(result);
        self.3.to_bytes_internal(result);
        self.4.to_bytes_internal(result);
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.0.to_bytes(),
            self.1.to_bytes(),
            self.2.to_bytes(),
            self.3.to_bytes(),
            self.4.to_bytes(),
        ].concat()
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {
        let (e0, ind0) = T::from_bytes_internal(bytes, index)?;
        let (e1, ind1) = U::from_bytes_internal(bytes, ind0)?;
        let (e2, ind2) = V::from_bytes_internal(bytes, ind1)?;
        let (e3, ind3) = W::from_bytes_internal(bytes, ind2)?;
        let (e4, ind4) = X::from_bytes_internal(bytes, ind3)?;

        Ok(((e0, e1, e2, e3, e4), ind4))
    }

}