//! Utilities for reading UTF-8 strings from a typed stream.

use crate::{
    deserializer::{consumed::Consumed, number::read_unsigned_int},
    error::Result,
};

/// Read a UTF-8 string from the stream, prefixed by its length encoded as an unsigned integer.
///
/// The function reads the length (1, 2, or 4 bytes as needed), then the UTF-8 bytes.
/// Returns a slice to the string and total bytes consumed.
///
/// # Errors
///
/// Returns [`TypedStreamError::OutOfBounds`] if not enough bytes for length or data,
/// or [`TypedStreamError::StringParseError`] if the bytes are not valid UTF-8.
///
/// # Examples
/// ```no_run
/// use crabstep::deserializer::string::read_string;
///
/// let data = [0x05, b'H', b'e', b'l', b'l', b'o'];
/// let consumed = read_string(&data).unwrap();
///
/// assert_eq!(consumed.value, "Hello");
/// assert_eq!(consumed.bytes_consumed, 6);
/// ```
pub fn read_string(data: &[u8]) -> Result<Consumed<&str>> {
    let length = read_unsigned_int(data)?;
    Ok(Consumed::new(
        std::str::from_utf8(
            &data[length.bytes_consumed..(length.bytes_consumed + length.value as usize)],
        )?,
        length.bytes_consumed + length.value as usize,
    ))
}

#[cfg(test)]
mod string_tests {
    use crate::{
        constants::{I_16, I_32},
        deserializer::string::read_string,
    };

    #[test]
    fn can_read_string() {
        let data = [0x05, b'H', b'e', b'l', b'l', b'o'];
        let result = read_string(&data).unwrap();

        assert_eq!(result.value, "Hello");
        assert_eq!(result.bytes_consumed, 6);
    }

    #[test]
    fn can_read_string_extra() {
        let data = [
            0x05, b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd',
        ];
        let result = read_string(&data).unwrap();

        assert_eq!(result.value, "Hello");
        assert_eq!(result.bytes_consumed, 6);
    }

    #[test]
    fn can_read_string_long() {
        // Bytes to indicate 1000 characters
        let start_bytes = [I_16, 0xE8, 0x03]; // 1000 in little-endian
        let long_string: Vec<u8> = (0..1000).map(|_| b'a').collect();
        let combined = start_bytes
            .iter()
            .chain(long_string.iter())
            .copied()
            .collect::<Vec<u8>>();

        let result = read_string(&combined).unwrap();

        assert_eq!(result.value.len(), 1000);
        assert!(result.value.starts_with('a'));
        assert!(result.value.ends_with('a'));
        // 1 byte for the int type, 2 bytes for I_16, 1000 bytes for the string
        assert_eq!(result.bytes_consumed, 1003);
    }

    #[test]
    fn can_read_string_insanely_long() {
        // Bytes to indicate 1000 characters
        let start_bytes = [I_32, 0x01, 0x01, 0x01, 0x01]; // 1000 in little-endian
        let long_string: Vec<u8> = (0..16843009).map(|_| b'a').collect();
        let combined = start_bytes
            .iter()
            .chain(long_string.iter())
            .copied()
            .collect::<Vec<u8>>();

        let result = read_string(&combined).unwrap();

        assert_eq!(result.value.len(), 16843009);
        assert!(result.value.starts_with('a'));
        assert!(result.value.ends_with('a'));
        // 1 byte for the int type, 4 bytes for I_32, 16843009 bytes for the string
        assert_eq!(result.bytes_consumed, 16843014);
    }

    #[test]
    fn can_read_empty_string() {
        let data = [0x00];
        let result = read_string(&data).unwrap();

        assert_eq!(result.value, "");
        assert_eq!(result.bytes_consumed, 1);
    }
}
