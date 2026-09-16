//! `is_null`: `NSNull` and nil references.

use crate::deserializer::iter::Property;
use crate::models::output_data::OutputData;

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// Whether this property is an `NSNull` instance or a nil object reference
    /// ([`OutputData::Null`]).
    #[must_use]
    pub fn is_null(&self) -> bool {
        match self {
            Property::Object { name: "NSNull", .. } | Property::Primitive(OutputData::Null) => true,
            Property::Group(group) => matches!(
                group.first(),
                Some(
                    Property::Object { name: "NSNull", .. } | Property::Primitive(OutputData::Null)
                )
            ),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn is_null_detects_nsnull() {
        let bytes = load("foundation/NestedScalars");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let null_count = ts
            .resolve_properties(root)
            .unwrap()
            .filter(|group| group.is_null())
            .count();

        assert_eq!(null_count, 1);
    }
}
