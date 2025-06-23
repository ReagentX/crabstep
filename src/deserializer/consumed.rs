//! A wrapper containing a parsed value and the number of bytes consumed during deserialization.

/// A value of type `T` along with how many bytes were consumed to produce it.
pub struct Consumed<T> {
    /// The parsed value.
    pub value: T,
    /// Number of bytes read from the stream.
    pub bytes_consumed: usize,
}

impl<T> Consumed<T> {
    /// Create a new `Consumed<T>` with the given `value` and `bytes_consumed`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::deserializer::consumed::Consumed;
    /// let consumed = Consumed::new(42, 2);
    ///
    /// assert_eq!(consumed.value, 42);
    /// assert_eq!(consumed.bytes_consumed, 2);
    /// ```
    pub fn new(value: T, bytes_consumed: usize) -> Self {
        Consumed {
            value,
            bytes_consumed,
        }
    }

    /// Transform the contained value to another type, preserving `bytes_consumed`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::deserializer::consumed::Consumed;
    /// let consumed = Consumed::new(2u8, 1);
    /// let mapped = consumed.map(|v| v as u16);
    ///
    /// assert_eq!(mapped.value, 2u16);
    /// assert_eq!(mapped.bytes_consumed, 1);
    /// ```
    pub fn map<U, F>(self, f: F) -> Consumed<U>
    where
        F: FnOnce(T) -> U,
    {
        Consumed {
            value: f(self.value),
            bytes_consumed: self.bytes_consumed,
        }
    }
}

impl<T> std::ops::Deref for Consumed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Consumed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
