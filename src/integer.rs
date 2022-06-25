use crate::{HSerde, HSerdeError};

// 127 -> [0_127]
// 129 -> [1_1, 0_1]
// 16384 -> [1_1, 1_0, 0_0]
// 16385 -> [1_1, 1_0, 0_1]
impl HSerde for u32 {

    fn to_bytes(&self) -> Vec<u8> {

        if *self < 128 {
            vec![*self as u8]
        }

        else if *self < 128 * 128 {
            vec![(self / 128) as u8 + 128, (self % 128) as u8]
        }

        else if *self < 128 * 128 * 128 {
            vec![(self / 128 / 128) as u8 + 128, (self / 128 % 128) as u8 + 128, (self % 128) as u8]
        }

        else {
            let mut n = *self;
            let mut result = vec![];

            while n > 0 {
                result.push((n % 128) as u8 + 128);
                n /= 128;
            }

            result[0] -= 128;
            result.into_iter().rev().collect()
        }

    }

    fn from_bytes_internal(bytes: &[u8], mut index: usize) -> Result<(u32, usize), HSerdeError> {
        let mut result = 0;

        if index >= bytes.len() {
            return Err(HSerdeError::IndexError);
        }

        while bytes[index] >= 128 {
            result *= 128;
            result += (bytes[index] - 128) as u32;
            index += 1;

            if index >= bytes.len() {
                return Err(HSerdeError::IndexError);
            }

        }

        result *= 128;
        result += bytes[index] as u32;

        Ok((result, index + 1))
    }

}

impl HSerde for usize {

    fn to_bytes(&self) -> Vec<u8> {

        if *self < 128 {
            vec![*self as u8]
        }

        else if *self < 128 * 128 {
            vec![(self / 128) as u8 + 128, (self % 128) as u8]
        }

        else if *self < 128 * 128 * 128 {
            vec![(self / 128 / 128) as u8 + 128, (self / 128 % 128) as u8 + 128, (self % 128) as u8]
        }

        else {
            let mut n = *self;
            let mut result = vec![];

            while n > 0 {
                result.push((n % 128) as u8 + 128);
                n /= 128;
            }

            result[0] -= 128;
            result.into_iter().rev().collect()
        }

    }

    fn from_bytes_internal(bytes: &[u8], mut index: usize) -> Result<(usize, usize), HSerdeError> {
        let mut result = 0;

        if index >= bytes.len() {
            return Err(HSerdeError::IndexError);
        }

        while bytes[index] >= 128 {
            result *= 128;
            result += (bytes[index] - 128) as usize;
            index += 1;

            if index >= bytes.len() {
                return Err(HSerdeError::IndexError);
            }

        }

        result *= 128;
        result += bytes[index] as usize;

        Ok((result, index + 1))
    }

}

impl HSerde for i32 {

    fn to_bytes(&self) -> Vec<u8> {
        signed_to_unsigned(*self).to_bytes()
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {
        let (result, next_index) = u32::from_bytes_internal(bytes, index)?;

        Ok((unsigned_to_signed(result), next_index))
    }

}

impl HSerde for u8 {

    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }

    fn from_bytes_internal(bytes: &[u8], index: usize) -> Result<(Self, usize), HSerdeError> {

        if index >= bytes.len() {
            Err(HSerdeError::IndexError)
        }

        else {
            Ok((bytes[index], index + 1))
        }

    }

}

/*
u32: 0  1   2  3   4  5   6  7   8  9  10
i32: 0  1  -1  2  -2  3  -3  4  -4  5  -5
*/

pub fn signed_to_unsigned(n: i32) -> u32 {

    if n > 0 {
        (n * 2 - 1) as u32
    }

    else {
        n.abs() as u32 * 2
    }

}

pub fn unsigned_to_signed(n: u32) -> i32 {
    ((n + 1) / 2) as i32 * ((n % 2 * 2) as i32 - 1)
}