//! `as_bool`: a boolean `NSNumber` (or bare primitive).

use crate::deserializer::iter::Property;

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// An `NSNumber` (or bare primitive) interpreted as a boolean. Returns `None`
    /// for integer values other than `0` and `1`.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self.as_i64()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn as_bool_reads_boolean() {
        let bytes = load("foundation/NumberBool"); // NSNumber(true) -> SignedInteger(1)
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_bool(), Some(true));
        assert_eq!(group.as_i64(), Some(1));
    }
}
