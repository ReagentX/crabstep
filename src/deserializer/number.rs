use crate::{
    constants::{DECIMAL, END, I_16, I_32, REFERENCE_TAG},
    deserializer::{
        consumed::Consumed,
        read::{read_byte_at, read_exact_bytes},
    },
    error::Result,
};

/// Read a signed integer from the stream. Because we don't know the size of the integer ahead of time,
/// we store it in the largest possible value.
pub fn read_signed_int(data: &[u8]) -> Result<Consumed<i64>> {
    let current_byte = read_byte_at(data, 0)?;
    let (value, consumed) = match *current_byte {
        // The number is 2 bytes long
        I_16 => {
            let size = 2;
            let value =
                i16::from_le_bytes(<[u8; 2]>::try_from(read_exact_bytes(&data[1..], size)?)?);
            (i64::from(value), size + 1)
        }
        // The number is 4 bytes long
        I_32 => {
            let size = 4;
            let value =
                i32::from_le_bytes(<[u8; 4]>::try_from(read_exact_bytes(&data[1..], size)?)?);
            (i64::from(value), size + 1)
        }
        // The number is 1 byte long and is the current byte
        _ => {
            // If the current byte is greater than the REFERENCE_TAG, it indicates an index in the table of already-seen types.
            if current_byte > &(REFERENCE_TAG as u8) && *read_byte_at(data, 1)? != END {
                return read_signed_int(&data[1..]);
            }

            let value = i8::from_le_bytes([*current_byte]);
            (i64::from(value), 1)
        }
    };

    Ok(Consumed::new(value, consumed))
}

/// Read a signed integer from the stream. Because we don't know the size of the integer ahead of time,
/// we store it in the largest possible value.
pub fn read_unsigned_int(data: &[u8]) -> Result<Consumed<u64>> {
    let current_byte = read_byte_at(data, 0)?;
    let (value, consumed) = match *current_byte {
        // The number is 2 bytes long
        I_16 => {
            let size = 2;
            let value =
                u16::from_le_bytes(<[u8; 2]>::try_from(read_exact_bytes(&data[1..], size)?)?);
            (u64::from(value), size + 1)
        }
        // The number is 4 bytes long
        I_32 => {
            let size = 4;
            let value =
                u32::from_le_bytes(<[u8; 4]>::try_from(read_exact_bytes(&data[1..], size)?)?);
            (u64::from(value), size + 1)
        }
        // The number is 1 byte long
        _ => {
            // If the current byte is greater than the REFERENCE_TAG, it indicates an index in the table of already-seen types.
            if current_byte > &(REFERENCE_TAG as u8) && *read_byte_at(data, 1)? != END {
                return read_unsigned_int(&data[1..]);
            }

            let value = u8::from_le_bytes([*current_byte]);
            (u64::from(value), 1)
        }
    };

    Ok(Consumed::new(value, consumed))
}

/// Read a single-precision float from the byte stream
pub fn read_float(data: &[u8]) -> Result<Consumed<f32>> {
    let current_byte = read_byte_at(data, 0)?;
    match *current_byte {
        DECIMAL => {
            let size = 4;
            let value = f32::from_le_bytes(<[u8; 4]>::try_from(read_exact_bytes(data, size)?)?);
            Ok(Consumed::new(value, size + 1))
        }
        I_16 | I_32 => Ok(read_signed_int(data)?.map(|v| v as f32)),
        _ => Ok(read_signed_int(data)?.map(|v| v as f32)),
    }
}

/// Read a double-precision float from the byte stream
pub fn read_double(data: &[u8]) -> Result<Consumed<f64>> {
    let current_byte = read_byte_at(data, 0)?;
    match *current_byte {
        DECIMAL => {
            let size = 8;
            let value = f64::from_le_bytes(<[u8; 8]>::try_from(read_exact_bytes(data, size)?)?);
            Ok(Consumed::new(value, size + 1))
        }
        I_16 | I_32 => Ok(read_signed_int(data)?.map(|v| v as f64)),
        _ => Ok(read_signed_int(data)?.map(|v| v as f64)),
    }
}

#[cfg(test)]
mod int_tests {
    use crate::{
        constants::{I_16, I_32},
        deserializer::number::{read_signed_int, read_unsigned_int},
    };

    #[test]
    fn can_read_signed_int_small() {
        let data = [0x01];
        let result = read_signed_int(&data).unwrap();

        assert_eq!(result.value, 1);
        assert_eq!(result.bytes_consumed, 1);
    }

    #[test]
    fn can_read_signed_int_16() {
        let data = [I_16, 0x01, 0x01];
        let result = read_signed_int(&data).unwrap();

        assert_eq!(result.value, 257);
        assert_eq!(result.bytes_consumed, 3);
    }

    #[test]
    fn can_read_signed_int_1000() {
        let data = [I_16, 0x01, 0x01];
        let result = read_signed_int(&data).unwrap();

        assert_eq!(result.value, 257);
        assert_eq!(result.bytes_consumed, 3);
    }

    #[test]
    fn can_read_signed_int_32() {
        let data = [I_32, 0x01, 0x01, 0x01, 0x01];
        let result = read_signed_int(&data).unwrap();

        assert_eq!(result.value, 16843009);
        assert_eq!(result.bytes_consumed, 5);
    }

    #[test]
    fn can_read_unsigned_int_small() {
        let data = [0x01];
        let result = read_unsigned_int(&data).unwrap();

        assert_eq!(result.value, 1);
        assert_eq!(result.bytes_consumed, 1);
    }

    #[test]
    fn can_read_unsigned_int_16() {
        let data = [I_16, 0x01, 0x01];
        let result = read_unsigned_int(&data).unwrap();

        assert_eq!(result.value, 257);
        assert_eq!(result.bytes_consumed, 3);
    }

    #[test]
    fn can_read_unsigned_int_32() {
        let data = [I_32, 0x01, 0x01, 0x01, 0x01];
        let result = read_unsigned_int(&data).unwrap();

        assert_eq!(result.value, 16843009);
        assert_eq!(result.bytes_consumed, 5);
    }

    #[test]
    fn cant_read_unsigned_int_16_too_short() {
        let data = [I_16, 0x01];
        let result = read_unsigned_int(&data);

        assert!(result.is_err());
    }

    #[test]
    fn cant_read_unsigned_int_32_too_short() {
        let data = [I_32, 0x01, 0x01, 0x01];
        let result = read_unsigned_int(&data);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod float_tests {
    use crate::{
        constants::{I_16, I_32},
        deserializer::number::{read_double, read_float},
    };

    #[test]
    fn can_read_float_small() {
        let data = [0x01];
        let result = read_float(&data).unwrap();

        assert_eq!(result.value, 1.);
        assert_eq!(result.bytes_consumed, 1);
    }

    #[test]
    fn can_read_float_16() {
        let data = [I_16, 0x01, 0x01];
        let result = read_float(&data).unwrap();

        assert_eq!(result.value, 257.);
        assert_eq!(result.bytes_consumed, 3);
    }

    #[test]
    fn can_read_float_32() {
        let data = [I_32, 0x01, 0x01, 0x01, 0x01];
        let result = read_float(&data).unwrap();

        assert_eq!(result.value, 16843009.);
        assert_eq!(result.bytes_consumed, 5);
    }

    #[test]
    fn can_read_double_small() {
        let data = [0x01];
        let result = read_double(&data).unwrap();

        assert_eq!(result.value, 1.);
        assert_eq!(result.bytes_consumed, 1);
    }

    #[test]
    fn can_read_double_16() {
        let data = [I_16, 0x01, 0x01];
        let result = read_double(&data).unwrap();

        assert_eq!(result.value, 257.);
        assert_eq!(result.bytes_consumed, 3);
    }

    #[test]
    fn can_read_double_32() {
        let data = [I_32, 0x01, 0x01, 0x01, 0x01];
        let result = read_double(&data).unwrap();

        assert_eq!(result.value, 16843009.);
        assert_eq!(result.bytes_consumed, 5);
    }
}
