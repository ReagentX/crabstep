use crate::{
    constants::REFERENCE_TAG,
    deserializer::consumed::Consumed,
    error::{Result, TypedStreamError},
};

/// Read exactly `n` bytes from a slice of the stream.
///
/// Returns a slice of length `n` or an error if the data is too short.
///
/// # Errors
///
/// Returns [`TypedStreamError::OutOfBounds`] when `data.len() < n`.
///
/// # Examples
/// ```no_run
/// use crabstep::deserializer::read::read_exact_bytes;
///
/// let data = [0x01, 0x02, 0x03];
/// let slice = read_exact_bytes(&data, 2).unwrap();
///
/// assert_eq!(slice, &[0x01, 0x02]);
/// ```
pub fn read_exact_bytes(data: &[u8], n: usize) -> Result<&[u8]> {
    let range = data
        .get(0..n)
        .ok_or(TypedStreamError::OutOfBounds(n, data.len()))?;

    Ok(range)
}

/// Read a single byte from a slice of the stream at index `idx`.
///
/// # Errors
///
/// Returns [`TypedStreamError::OutOfBounds`] when `idx >= data.len()`.
///
/// # Examples
/// ```no_run
/// use crabstep::deserializer::read::read_byte_at;
///
/// let data = [0xFF];
/// let byte = read_byte_at(&data, 0).unwrap();
///
/// assert_eq!(*byte, 0xFF);
/// ```
pub fn read_byte_at(data: &[u8], idx: usize) -> Result<&u8> {
    data.get(idx)
        .ok_or(TypedStreamError::OutOfBounds(idx, data.len()))
}

/// Read a reference pointer encoded as a single byte.
///
/// Subtracts the [`REFERENCE_TAG`] constant to yield the zero-based index.
///
/// # Errors
///
/// Returns [`TypedStreamError::InvalidPointer`] if the byte is less than `REFERENCE_TAG`.
///
/// # Examples
/// ```no_run
/// use crabstep::constants::REFERENCE_TAG;
/// use crabstep::deserializer::read::read_pointer;
///
/// let raw = REFERENCE_TAG + 3;
/// let consumed = read_pointer(&raw).unwrap();
///
/// assert_eq!(consumed.value, 3);
/// assert_eq!(consumed.bytes_consumed, 1);
/// ```
pub fn read_pointer(pointer: &u8) -> Result<Consumed<u64>> {
    let result = u64::from(*pointer)
        .checked_sub(REFERENCE_TAG)
        .ok_or(TypedStreamError::InvalidPointer(*pointer))?;

    Ok(Consumed::new(result, 1))
}

#[cfg(test)]
mod read_tests {
    use crate::deserializer::read::{read_exact_bytes, read_pointer};
    use crate::error::TypedStreamError;

    #[test]
    fn can_read_exact_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let result = read_exact_bytes(&data, 4).unwrap();
        assert_eq!(result, &data);
    }

    #[test]
    fn can_read_exact_bytes_partial() {
        let data = [0x01, 0x02, 0x03];
        let result = read_exact_bytes(&data, 2).unwrap();
        assert_eq!(result, &data[..2]);
    }

    #[test]
    fn cannot_read_exact_bytes_out_of_bounds() {
        let data = [0x01, 0x02];
        let result = read_exact_bytes(&data, 3);
        assert!(matches!(result, Err(TypedStreamError::OutOfBounds(3, 2))));
    }

    #[test]
    fn can_read_pointer() {
        // Pointer value 3 (0x03) + REFERENCE_TAG (0x92)
        let result = read_pointer(&0x95).unwrap();
        assert_eq!(result.value, 3);
        assert_eq!(result.bytes_consumed, 1);
    }

    #[test]
    fn can_read_pointer_zero() {
        // Pointer value 30(0x00) + REFERENCE_TAG (0x92)
        let result = read_pointer(&0x92).unwrap();
        assert_eq!(result.value, 0);
        assert_eq!(result.bytes_consumed, 1);
    }

    #[test]
    fn cant_read_invalid_pointer() {
        // Invalid pointer
        let result = read_pointer(&0x10);
        assert!(matches!(result, Err(TypedStreamError::InvalidPointer(16))));
    }
}
