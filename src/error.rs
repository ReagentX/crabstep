//! Error types and result alias for `typedstream` deserialization

use core::{array::TryFromSliceError, fmt::Display};

/// A specialized [`Result`] type for `typedstream` operations.
///
/// # Examples
///
/// ```no_run
/// use crabstep::error::Result;
/// use crabstep::TypedStreamDeserializer;
///
/// fn get_root(data: &[u8]) -> Result<usize> {
///    let mut deserializer = TypedStreamDeserializer::new(data);
///    deserializer.oxidize()
/// }
/// ```
pub type Result<T> = core::result::Result<T, TypedStreamError>;

/// Errors that can occur while deserializing a `typedstream`.
#[derive(Debug)]
pub enum TypedStreamError {
    /// An invalid object was encountered, such as an unmatched end marker.
    InvalidObject,
    /// Attempted to access an index outside the stream bounds (requested, length).
    OutOfBounds(usize, usize),
    /// Error converting a slice into an array of fixed size.
    SliceError(TryFromSliceError),
    /// Error parsing a string as UTF-8.
    StringParseError(core::str::Utf8Error),
    /// The `typedstream` header was invalid.
    InvalidHeader,
    /// Encountered an invalid pointer value.
    InvalidPointer(u8),
    /// The array header is malformed or too large.
    InvalidArray(usize),
    /// Encountered an empty string where data was expected.
    EmptyString,
}

impl Display for TypedStreamError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TypedStreamError::InvalidObject => {
                write!(f, "Invalid object encountered in typedstream!")
            }
            TypedStreamError::OutOfBounds(n, len) => write!(
                f,
                "Out of bounds access: tried to access byte {n} in a stream of length {len}"
            ),
            TypedStreamError::SliceError(try_from_slice_error) => {
                write!(f, "Slice conversion error: {try_from_slice_error}")
            }
            TypedStreamError::StringParseError(utf8_error) => {
                write!(f, "String parsing error: {utf8_error}")
            }
            TypedStreamError::InvalidHeader => write!(f, "Invalid header in typedstream!"),
            TypedStreamError::InvalidPointer(pointer) => {
                write!(f, "Invalid pointer: {pointer:x}")
            }
            TypedStreamError::InvalidArray(offset) => {
                write!(f, "Invalid array at index: {offset:x}")
            }
            TypedStreamError::EmptyString => write!(f, "Empty string encountered in typedstream"),
        }
    }
}

impl From<TryFromSliceError> for TypedStreamError {
    fn from(error: TryFromSliceError) -> Self {
        TypedStreamError::SliceError(error)
    }
}

impl From<core::str::Utf8Error> for TypedStreamError {
    fn from(error: core::str::Utf8Error) -> Self {
        TypedStreamError::StringParseError(error)
    }
}

impl core::error::Error for TypedStreamError {}
