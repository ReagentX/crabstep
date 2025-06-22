use crate::{
    constants::REFERENCE_TAG,
    deserializer::consumed::Consumed,
    error::{Result, TypedStreamError},
};

/// Read exactly `n` bytes from a slice of the stream.
///
/// If the slice is shorter than `n`, it returns an `OutOfBounds` error.
pub fn read_exact_bytes(data: &[u8], n: usize) -> Result<&[u8]> {
    let range = data
        .get(0..n)
        .ok_or(TypedStreamError::OutOfBounds(n, data.len()))?;

    Ok(range)
}

/// Read a single byte from a slice of the stream.
///
/// If the slice is shorter than `n`, it returns an `OutOfBounds` error.
pub fn read_byte_at(data: &[u8], idx: usize) -> Result<&u8> {
    data.get(idx)
        .ok_or(TypedStreamError::OutOfBounds(idx, data.len()))
}

/// Read a reference pointer for a Type
///
/// While this does consume a byte, pointers refer to an index in the [`type_table`](crate::deserializer::typedstream::TypedStreamDeserializer::type_table) or
/// [`object_table`](crate::deserializer::typedstream::TypedStreamDeserializer::object_table), so it does generally need to advance the current position.
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
