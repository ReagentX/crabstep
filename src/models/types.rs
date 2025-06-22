use crate::{
    constants::ARRAY,
    deserializer::{consumed::Consumed, number::read_unsigned_int, read::read_exact_bytes},
    error::{Result, TypedStreamError},
};

/// Represents primitive types of data that can be stored in a `typedstream`
///
/// These type encodings are partially documented [here](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html#//apple_ref/doc/uid/TP40008048-CH100-SW1) by Apple.
#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    /// Encoded string data, usually embedded in an object. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x28` | [`+`](https://www.compart.com/en/unicode/U+002B) |
    Utf8String,
    /// Encoded bytes that can be parsed again as data. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x2A` | [`*`](https://www.compart.com/en/unicode/U+002A) |
    EmbeddedData,
    /// An instance of a class, usually with data. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x40` | [`@`](https://www.compart.com/en/unicode/U+0040) |
    Object,
    /// An [`i8`], [`i16`], or [`i32`]. Denoted by:
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x63` | [`c`](https://www.compart.com/en/unicode/U+0063) |
    /// | `0x69` | [`i`](https://www.compart.com/en/unicode/U+0069) |
    /// | `0x6c` | [`l`](https://www.compart.com/en/unicode/U+006c) |
    /// | `0x71` | [`q`](https://www.compart.com/en/unicode/U+0071) |
    /// | `0x73` | [`s`](https://www.compart.com/en/unicode/U+0073) |
    ///
    /// The width is determined by the prefix: [`i8`] has none, [`i16`] has `0x81`, and [`i32`] has `0x82`.
    SignedInt,
    /// A [`u8`], [`u16`], or [`u32`]. Denoted by:
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x43` | [`C`](https://www.compart.com/en/unicode/U+0043) |
    /// | `0x49` | [`I`](https://www.compart.com/en/unicode/U+0049) |
    /// | `0x4c` | [`L`](https://www.compart.com/en/unicode/U+004c) |
    /// | `0x51` | [`Q`](https://www.compart.com/en/unicode/U+0051) |
    /// | `0x53` | [`S`](https://www.compart.com/en/unicode/U+0053) |
    ///
    /// The width is determined by the prefix: [`u8`] has none, [`u16`] has `0x81`, and [`u32`] has `0x82`.
    UnsignedInt,
    /// An [`f32`]. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x66` | [`f`](https://www.compart.com/en/unicode/U+0066) |
    Float,
    /// An [`f64`]. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x64` | [`d`](https://www.compart.com/en/unicode/U+0064) |
    Double,
    /// Some text we can reuse later, i.e. a class name.
    String(&'a str),
    /// An array containing some data of a given length. Denoted in the stream by braced digits: `[123]`.
    Array(usize),
    /// Data for which we do not know the type, likely for something this parser does not implement.
    Unknown(u8),
}

impl<'a> Type<'a> {
    /// Convert a byte to a Type enum variant
    #[inline]
    pub(crate) fn from_byte(byte: &u8) -> Self {
        match *byte {
            0x40 => Self::Object,
            0x2B => Self::Utf8String,
            0x2A => Self::EmbeddedData,
            0x66 => Self::Float,
            0x64 => Self::Double,
            0x63 | 0x69 | 0x6c | 0x71 | 0x73 => Self::SignedInt,
            0x43 | 0x49 | 0x4c | 0x51 | 0x53 => Self::UnsignedInt,
            other => Self::Unknown(other),
        }
    }

    #[inline]
    pub(crate) fn new_string(str: &'a str) -> Self {
        Self::String(str)
    }

    pub(crate) fn get_array_length(types: &[u8]) -> Option<Vec<Type>> {
        if types.first() == Some(&0x5b) {
            let len =
                types[1..]
                    .iter()
                    .take_while(|a| a.is_ascii_digit())
                    .fold(None, |acc, ch| {
                        char::from_u32(u32::from(*ch))?
                            .to_digit(10)
                            .map(|b| acc.unwrap_or(0) * 10 + b)
                    })?;
            return Some(vec![Type::Array(len as usize)]);
        }
        None
    }

    pub(crate) fn read_new_type(data: &[u8]) -> Result<Consumed<Vec<Type>>> {
        // Get the type of the object
        let type_length = read_unsigned_int(data)?;

        // Get the bytes for the type
        let type_bytes = read_exact_bytes(
            &data[type_length.bytes_consumed..],
            type_length.value as usize,
        )?;

        // Handle array size
        if type_bytes.first() == Some(&ARRAY) {
            return Ok(Consumed::new(
                Type::get_array_length(type_bytes).ok_or(TypedStreamError::InvalidArray(0))?,
                type_length.bytes_consumed + type_bytes.len(),
            ));
        }

        Ok(Consumed::new(
            type_bytes.iter().map(Type::from_byte).collect(),
            type_length.bytes_consumed + type_bytes.len(),
        ))
    }
}
