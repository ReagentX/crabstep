//! `as_string`: the string cluster and attributed strings.

use crate::deserializer::foundation::names::{ATTRIBUTED_STRING_CLASSES, STRING_CLASSES};
use crate::deserializer::iter::{Property, PropertyIterator};
use crate::models::output_data::OutputData;

/// Extract the backing UTF-8 of a string-cluster object: the first `String`
/// primitive in the object's first data group.
fn backing_string<'a, 'b: 'a>(mut data: PropertyIterator<'a, 'b>) -> Option<&'a str> {
    if let Property::Group(group) = data.next()?
        && let Some(Property::Primitive(OutputData::String(s))) = group.first()
    {
        return Some(s);
    }
    None
}

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// The backing string of an `NSString` / `NSMutableString`, or the plain text
    /// of an `NSAttributedString` / `NSMutableAttributedString` (its attributes
    /// remain reachable through the generic [`Property`] tree).
    #[must_use]
    pub fn as_string(&self) -> Option<&'a str> {
        if let Some(data) = self.object_in_classes(STRING_CLASSES) {
            return backing_string(data);
        }
        // An attributed string stores its backing store as a nested
        // `NSString`/`NSMutableString`; its position among the groups varies by
        // producer (the attributes dictionary often comes first), so scan.
        if let Some(data) = self.object_in_classes(ATTRIBUTED_STRING_CLASSES) {
            for prop in data {
                if let Property::Group(group) = prop
                    && let Some(Property::Object {
                        name, data: inner, ..
                    }) = group.first()
                    && STRING_CLASSES.contains(&name)
                {
                    return backing_string(inner);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn as_string_reads_both_string_variants() {
        // NSArray([NSString "imm", NSMutableString "mut"])
        let bytes = load("foundation/NestedStrings");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let strings: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();

        assert_eq!(strings.len(), 2, "{strings:?}");
        assert!(strings.contains(&"imm"), "{strings:?}");
        assert!(strings.contains(&"mut"), "{strings:?}");
    }

    #[test]
    fn as_string_reads_attributed_backing_text() {
        // NSArray([NSAttributedString "styled"]) — exercises the scan path.
        let bytes = load("foundation/NestedAttributed");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let strings: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();

        assert_eq!(strings, vec!["styled"]);
    }

    #[test]
    fn as_string_reads_real_mutable_attributed_body() {
        // Real Messages NSMutableAttributedString: backing store is an
        // NSMutableString reached as one of the root's groups (plain path).
        let bytes = load("AttributedBodyTextOnly");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let strings: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();

        assert!(strings.contains(&"Noter test"), "{strings:?}");
    }
}
