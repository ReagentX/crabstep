//! Provides a deserializer for the `typedstream` format

pub mod constants;
pub mod consumed;
#[cfg(feature = "foundation")]
#[cfg_attr(docsrs, doc(cfg(feature = "foundation")))]
pub mod foundation;
pub mod header;
pub mod iter;
pub mod number;
pub mod read;
pub mod string;
pub mod typedstream;
