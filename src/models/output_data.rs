/// Rust structures containing data stored in the `typedstream`
#[derive(Debug, PartialEq)]
pub enum OutputData<'a> {
    /// Text data, denoted in the stream by [`Type::String`](crate::models::types::Type::String)
    String(&'a str),
    /// Signed integer types are coerced into this container, denoted in the stream by [`Type::SignedInt`](crate::models::types::Type::String)
    SignedInteger(i64),
    /// Unsigned integer types are coerced into this container, denoted in the stream by [`Type::UnsignedInt`](crate::models::types::Type::String)
    UnsignedInteger(u64),
    /// Floating point numbers, denoted in the stream by [`Type::Float`](crate::models::types::Type::String)
    Float(f32),
    /// Double precision floats, denoted in the stream by [`Type::Double`](crate::models::types::Type::String)
    Double(f64),
    /// Bytes whose type is not known, denoted in the stream by [`Type::Unknown`](crate::models::types::Type::String)
    Byte(u8),
    /// Arbitrary collection of bytes in an array, denoted in the stream by [`Type::Array`](crate::models::types::Type::String)
    Array(&'a [u8]),
    /// Reference to another object by index in the [`object_table`](crate::deserializer::typedstream::TypedStreamDeserializer::object_table).
    Object(usize),
}
