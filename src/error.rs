//! Error types and result alias for typed stream deserialization.

use std::{array::TryFromSliceError, fmt::Display};

/// A specialized [`Result`] type for typed stream operations.
pub type Result<T> = std::result::Result<T, TypedStreamError>;

/// Errors that can occur while deserializing a typed stream.
#[derive(Debug)]
pub enum TypedStreamError {
    /// A start tag without a matching end tag was found.
    UnmatchedStart,
    /// An end tag without a matching start tag was found.
    UnmatchedEnd,
    /// Attempted to access an index outside the stream bounds (requested, length).
    OutOfBounds(usize, usize),
    /// Error converting a slice into an array of fixed size.
    SliceError(TryFromSliceError),
    /// Error parsing a string as UTF-8.
    StringParseError(std::str::Utf8Error),
    /// The typed stream header was invalid.
    InvalidHeader,
    /// Encountered an invalid pointer value.
    InvalidPointer(u8),
    /// The array header is malformed or too large.
    InvalidArray(usize),
    /// Encountered an empty string where data was expected.
    EmptyString,
}

impl Display for TypedStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedStreamError::UnmatchedStart => write!(f, "Unmatched start in stream"),
            TypedStreamError::UnmatchedEnd => write!(f, "Unmatched end in stream"),
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
                write!(f, "Invalid pointer: {pointer}")
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

impl From<std::str::Utf8Error> for TypedStreamError {
    fn from(error: std::str::Utf8Error) -> Self {
        TypedStreamError::StringParseError(error)
    }
}

impl std::error::Error for TypedStreamError {}
