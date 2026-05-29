//! Type tags that denote the type of data stored in a `typedstream`
use crate::{
    deserializer::{
        constants::ARRAY, consumed::Consumed, number::read_unsigned_int, read::read_exact_bytes,
    },
    error::{Result, TypedStreamError},
};
use alloc::vec::Vec;

/// Represents primitive types of data that can be stored in a `typedstream`
///
/// These type encodings are partially documented [here](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html#//apple_ref/doc/uid/TP40008048-CH100-SW1) by Apple.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub(crate) fn from_byte(byte: u8) -> Self {
        match byte {
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

    /// Parse the decimal length of an array descriptor of the form `[123]`,
    /// returning the length once the leading `[` and at least one digit are seen.
    pub(crate) fn get_array_length(types: &'_ [u8]) -> Option<usize> {
        if types.first() != Some(&ARRAY) {
            return None;
        }
        let mut len = 0usize;
        let mut saw_digit = false;
        for &byte in &types[1..] {
            if !byte.is_ascii_digit() {
                break;
            }
            len = len * 10 + usize::from(byte - b'0');
            saw_digit = true;
        }
        saw_digit.then_some(len)
    }

    pub(crate) fn read_new_type(data: &'_ [u8]) -> Result<Consumed<TypeEntry<'_>>> {
        // Get the type of the object
        let type_length = read_unsigned_int(data)?;

        // Get the bytes for the type
        let type_bytes = read_exact_bytes(
            &data[type_length.bytes_consumed..],
            type_length.value as usize,
        )?;
        let bytes_consumed = type_length.bytes_consumed + type_bytes.len();

        // Handle array size
        if type_bytes.first() == Some(&ARRAY) {
            let len =
                Type::get_array_length(type_bytes).ok_or(TypedStreamError::InvalidArray(0))?;
            return Ok(Consumed::new(
                TypeEntry::One(Type::Array(len)),
                bytes_consumed,
            ));
        }

        // The overwhelming majority of type descriptors are a single type, so
        // keep that case off the heap.
        let entry = if let [byte] = type_bytes {
            TypeEntry::One(Type::from_byte(*byte))
        } else {
            TypeEntry::Many(type_bytes.iter().copied().map(Type::from_byte).collect())
        };
        Ok(Consumed::new(entry, bytes_consumed))
    }
}

/// One entry in the deserializer's type table.
///
/// A type descriptor usually resolves to a single [`Type`]; storing that inline
/// avoids a heap allocation per entry (the table was previously a
/// `Vec<Vec<Type>>`). Multi-type descriptors fall back to a [`Vec`].
#[derive(Debug, Clone, PartialEq)]
pub enum TypeEntry<'a> {
    /// A single type, stored inline.
    One(Type<'a>),
    /// Two or more types.
    Many(Vec<Type<'a>>),
}

impl<'a> TypeEntry<'a> {
    /// The number of types in this entry.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            TypeEntry::One(_) => 1,
            TypeEntry::Many(types) => types.len(),
        }
    }

    /// Whether the entry has no types.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, TypeEntry::Many(types) if types.is_empty())
    }

    /// The first type in the entry, if any.
    #[must_use]
    pub fn first(&self) -> Option<&Type<'a>> {
        match self {
            TypeEntry::One(ty) => Some(ty),
            TypeEntry::Many(types) => types.first(),
        }
    }

    /// Build a [`TypeEntry`] from a list of types, normalizing the single-type
    /// case to [`TypeEntry::One`]. Used by tests to express expected type tables
    /// in the pre-existing nested style.
    #[cfg(test)]
    pub(crate) fn from_types(mut types: Vec<Type<'a>>) -> Self {
        if types.len() == 1 {
            TypeEntry::One(types.pop().unwrap())
        } else {
            TypeEntry::Many(types)
        }
    }
}

impl<'a> core::ops::Index<usize> for TypeEntry<'a> {
    type Output = Type<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            TypeEntry::One(ty) => {
                assert_eq!(index, 0, "index out of bounds for single-type entry");
                ty
            }
            TypeEntry::Many(types) => &types[index],
        }
    }
}
