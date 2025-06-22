use crate::models::class::Class;

/// Rust structures containing data stored in the `typedstream`
#[derive(Debug, PartialEq)]
pub enum OutputData<'a> {
    /// Text data, denoted in the stream by [`Type::String`]
    String(&'a str),
    /// Signed integer types are coerced into this container, denoted in the stream by [`Type::SignedInt`]
    SignedInteger(i64),
    /// Unsigned integer types are coerced into this container, denoted in the stream by [`Type::UnsignedInt`]
    UnsignedInteger(u64),
    /// Floating point numbers, denoted in the stream by [`Type::Float`]
    Float(f32),
    /// Double precision floats, denoted in the stream by [`Type::Double`]
    Double(f64),
    /// Bytes whose type is not known, denoted in the stream by [`Type::Unknown`]
    Byte(u8),
    /// Arbitrary collection of bytes in an array, denoted in the stream by [`Type::Array`]
    Array(&'a [u8]),
    /// A found class, in order of inheritance, used by [`Archivable::Class`]
    Class(Class),
    /// An object reference in the stream
    Object(usize),
}
