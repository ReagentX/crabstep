use std::{array::TryFromSliceError, fmt::Display};

pub type Result<T> = std::result::Result<T, TypedStreamError>;

#[derive(Debug)]
pub enum TypedStreamError {
    UnexpectedEnd,
    UnmatchedStart,
    UnmatchedEnd,
    OutOfBounds(usize, usize),
    SliceError(TryFromSliceError),
    StringParseError(std::str::Utf8Error),
    InvalidHeader,
    InvalidPointer(u8),
    InvalidArray(usize),
    EmptyString,
}

impl Display for TypedStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedStreamError::UnexpectedEnd => write!(f, "Unexpected end of stream"),
            TypedStreamError::UnmatchedStart => write!(f, "Unmatched start in stream"),
            TypedStreamError::UnmatchedEnd => write!(f, "Unmatched end in stream"),
            TypedStreamError::OutOfBounds(n, len) => write!(
                f,
                "Out of bounds access: tried to access byte {} in a stream of length {}",
                n, len
            ),
            TypedStreamError::SliceError(try_from_slice_error) => {
                write!(f, "Slice conversion error: {}", try_from_slice_error)
            }
            TypedStreamError::StringParseError(utf8_error) => {
                write!(f, "String parsing error: {}", utf8_error)
            }
            TypedStreamError::InvalidHeader => write!(f, "Invalid header in typedstream!"),
            TypedStreamError::InvalidPointer(pointer) => {
                write!(f, "Invalid pointer: {}", pointer)
            }
            TypedStreamError::InvalidArray(offset) => {
                write!(f, "Invalid array at index: {:x}", offset)
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
