//! Output data types for the `typedstream` deserializer

/// Rust structures containing data stored in the `typedstream`
#[derive(Debug, PartialEq)]
pub enum OutputData<'a> {
    /// Text data, denoted in the stream by [`Type::String`](crate::models::types::Type::String)
    String(&'a str),
    /// Signed integer types are coerced into this container, denoted in the stream by [`Type::SignedInt`](crate::models::types::Type::SignedInt)
    SignedInteger(i64),
    /// Unsigned integer types are coerced into this container, denoted in the stream by [`Type::UnsignedInt`](crate::models::types::Type::UnsignedInt)
    UnsignedInteger(u64),
    /// Floating point numbers, denoted in the stream by [`Type::Float`](crate::models::types::Type::Float)
    Float(f32),
    /// Double precision floats, denoted in the stream by [`Type::Double`](crate::models::types::Type::Double)
    Double(f64),
    /// Bytes whose type is not known, denoted in the stream by [`Type::Unknown`](crate::models::types::Type::Unknown)
    Byte(u8),
    /// Arbitrary collection of bytes in an array, denoted in the stream by [`Type::Array`](crate::models::types::Type::Array)
    Array(&'a [u8]),
    /// Reference to another object by index in the [`object_table`](crate::deserializer::typedstream::TypedStreamDeserializer::object_table).
    Object(usize),
    /// Represents a null value.
    Null,
}

impl<'a> OutputData<'a> {
    /// Returns the inner string if this is a `String` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::String("Hello");
    /// assert_eq!(data.as_str(), Some("Hello"));
    /// ```
    #[must_use]
    pub fn as_str(&self) -> Option<&'a str> {
        if let OutputData::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Returns the inner signed integer if this is a `SignedInteger` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::SignedInteger(42);
    /// assert_eq!(data.as_i64(), Some(42));
    /// ```
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        if let OutputData::SignedInteger(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    /// Returns the inner unsigned integer if this is an `UnsignedInteger` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::UnsignedInteger(100);
    /// assert_eq!(data.as_u64(), Some(100));
    /// ```
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        if let OutputData::UnsignedInteger(u) = self {
            Some(*u)
        } else {
            None
        }
    }

    /// Returns the inner float if this is a `Float` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::Float(3.14);
    /// assert_eq!(data.as_f32(), Some(3.14));
    /// ```
    #[must_use]
    pub fn as_f32(&self) -> Option<f32> {
        if let OutputData::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    /// Returns the inner double if this is a `Double` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::Double(2.71828);
    /// assert_eq!(data.as_f64(), Some(2.71828));
    /// ```
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        if let OutputData::Double(d) = self {
            Some(*d)
        } else {
            None
        }
    }

    /// Returns the inner byte if this is a `Byte` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::Byte(0xFF);
    /// assert_eq!(data.as_byte(), Some(0xFF));
    /// ```
    #[must_use]
    pub fn as_byte(&self) -> Option<u8> {
        if let OutputData::Byte(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Returns the inner byte slice if this is an `Array` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::Array(&[1, 2, 3]);
    /// assert_eq!(data.as_array(), Some(&[1, 2, 3][..]));
    /// ```
    #[must_use]
    pub fn as_array(&self) -> Option<&'a [u8]> {
        if let OutputData::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }

    /// Returns the inner object index if this is an `Object` variant.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::models::output_data::OutputData;
    ///
    /// let data = OutputData::Object(42);
    /// assert_eq!(data.as_object(), Some(42));
    /// ```
    #[must_use]
    pub fn as_object(&self) -> Option<usize> {
        if let OutputData::Object(idx) = self {
            Some(*idx)
        } else {
            None
        }
    }
}

// Implement Display for human-friendly formatting
impl core::fmt::Display for OutputData<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OutputData::String(s) => write!(f, "{s}"),
            OutputData::SignedInteger(i) => write!(f, "{i}"),
            OutputData::UnsignedInteger(u) => write!(f, "{u}"),
            OutputData::Float(fp) => write!(f, "{fp}"),
            OutputData::Double(d) => write!(f, "{d}"),
            OutputData::Byte(b) => write!(f, "0x{b:02x}"),
            OutputData::Array(arr) => write!(f, "[{arr:02x?}]"),
            OutputData::Object(idx) => write!(f, "Object({idx})"),
            OutputData::Null => write!(f, "Null"),
        }
    }
}
