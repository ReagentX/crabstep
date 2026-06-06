//! Constants used in the serialization and deserialization process

/// Indicates an [`i16`] in the byte stream
pub const I_16: u8 = 0x81;
/// Indicates an [`i32`] in the byte stream
pub const I_32: u8 = 0x82;
/// Indicates an [`i64`]/[`u64`] (8-byte) in the byte stream, used for `NSNumber`
/// values whose magnitude does not fit a 32-bit integer (C type `q`/`Q`)
pub const I_64: u8 = 0x87;
/// Indicates an [`f32`] or [`f64`] in the byte stream; the [`Type`](crate::models::types::Type) determines the size
pub const DECIMAL: u8 = 0x83;
/// Indicates the start of a new object
pub const START: u8 = 0x84;
/// Indicates that there is no more data to parse, for example the end of a class inheritance chain
pub const EMPTY: u8 = 0x85;
/// Indicates the last byte of an object
pub const END: u8 = 0x86;
/// Bytes equal or greater in value than the reference tag indicate an index in the table of already-seen types
pub const REFERENCE_TAG: u64 = 0x92;
/// Indicates an array in the byte stream
pub const ARRAY: u8 = 0x5b;
