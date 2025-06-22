use crate::{
    deserializer::{
        consumed::Consumed,
        number::{read_signed_int, read_unsigned_int},
        string::read_string,
    },
    error::{Result, TypedStreamError},
};

/// In the original source there are several variants of the header, but we
/// only need to validate that this is the header used by macOS/iOS, as iMessage
/// is probably not available on any `NeXT` platform
pub fn validate_header(data: &[u8]) -> Result<Consumed<bool>> {
    // Encoding type
    let typedstream_version = read_unsigned_int(data)?;
    // Encoding signature
    let signature = read_string(&data[typedstream_version.bytes_consumed..])?;
    // System version
    let system_version =
        read_signed_int(&data[typedstream_version.bytes_consumed + signature.bytes_consumed..])?;

    if typedstream_version.value != 4
        || signature.value != "streamtyped"
        || system_version.value != 1000
    {
        return Err(TypedStreamError::InvalidHeader);
    }

    Ok(Consumed::new(
        true,
        typedstream_version.bytes_consumed
            + signature.bytes_consumed
            + system_version.bytes_consumed,
    ))
}

#[cfg(test)]
mod header_tests {
    use std::{env::current_dir, fs::File, io::Read};

    use crate::{constants::I_16, deserializer::header::validate_header};

    #[test]
    fn can_validate_header() {
        let data = [
            0x04, // TypedStream version (4)
            0x0b, // Length of the signature
            b's', b't', b'r', b'e', b'a', b'm', b't', b'y', b'p', b'e', b'd', // Signature
            I_16, 0xe8, 0x03, // System version (1000 in little-endian)
        ];
        let result = validate_header(&data).unwrap();

        assert!(result.value);
        assert_eq!(result.bytes_consumed, 16);
    }

    #[test]
    fn can_validate_real_header() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AttributedBodyTextOnly");
        println!("Parsing file: {:?}", typedstream_path);
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        // Skip the header for now
        let validated = validate_header(&bytes).unwrap();

        assert!(validated.value);
        assert_eq!(validated.bytes_consumed, 16);
    }

    #[test]
    fn fails_on_invalid_header() {
        let data = [0x01]; // Invalid TypedStream version
        assert!(validate_header(&data).is_err());
    }
}
