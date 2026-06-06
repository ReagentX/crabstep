//! `as_i64` / `as_u64` / `as_f64`: numeric `NSNumber`s (or bare primitives).

use crate::deserializer::iter::{Property, PropertyIterator};
use crate::models::output_data::OutputData;

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// An `NSNumber` (or bare primitive) interpreted as a signed integer. Accepts
    /// either integer encoding; an unsigned value that does not fit `i64` yields
    /// `None`. Float/double values are *not* coerced.
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        match self.scalar()? {
            OutputData::SignedInteger(v) => Some(*v),
            OutputData::UnsignedInteger(v) => i64::try_from(*v).ok(),
            _ => None,
        }
    }

    /// An `NSNumber` (or bare primitive) interpreted as an unsigned integer.
    /// Accepts either integer encoding; a negative signed value yields `None`.
    /// Float/double values are *not* coerced.
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        match self.scalar()? {
            OutputData::UnsignedInteger(v) => Some(*v),
            OutputData::SignedInteger(v) => u64::try_from(*v).ok(),
            _ => None,
        }
    }

    /// An `NSNumber` (or bare primitive) interpreted as a double. Accepts `Float`
    /// and `Double`; integer values are *not* coerced.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self.scalar()? {
            OutputData::Double(v) => Some(*v),
            OutputData::Float(v) => Some(f64::from(*v)),
            _ => None,
        }
    }

    /// The underlying scalar value of a group that is either a bare primitive or
    /// an `NSNumber` wrapping one.
    fn scalar(&self) -> Option<&'b OutputData<'a>> {
        match self {
            Property::Primitive(value) => Some(value),
            Property::Object {
                name: "NSNumber",
                data,
                ..
            } => number_value(data.clone()),
            Property::Group(group) => match group.first()? {
                Property::Primitive(value) => Some(value),
                Property::Object {
                    name: "NSNumber",
                    data,
                    ..
                } => number_value(data),
                _ => None,
            },
            _ => None,
        }
    }
}

/// The first primitive in an `NSNumber` object's first group.
fn number_value<'a, 'b: 'a>(mut data: PropertyIterator<'a, 'b>) -> Option<&'b OutputData<'a>> {
    match data.next()? {
        Property::Group(inner) => match inner.first()? {
            Property::Primitive(value) => Some(value),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn root_object_resolves_as_i64() {
        let bytes = load("foundation/NumberInt");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        assert_eq!(ts.root().unwrap().as_i64(), Some(42));
    }

    #[test]
    fn as_f64_reads_decimal_double() {
        // NumberDouble root: bare-primitive path, exercises the B1 DECIMAL fix.
        let bytes = load("foundation/NumberDouble");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_f64(), Some(100.5));
    }

    #[test]
    fn as_f64_reads_float() {
        let bytes = load("foundation/NumberFloat");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_f64(), Some(3.5));
    }

    #[test]
    fn as_i64_reads_large_negative() {
        // NumberInt64 = -9_000_000_000: bare-primitive path, exercises the B2 fix.
        let bytes = load("foundation/NumberInt64");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_i64(), Some(-9_000_000_000));
    }

    #[test]
    fn integer_coercion_and_strictness() {
        // NumberInt = SignedInteger(42).
        let bytes = load("foundation/NumberInt");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_i64(), Some(42));
        assert_eq!(group.as_u64(), Some(42)); // non-negative signed coerces to u64
        assert_eq!(group.as_f64(), None); // no silent int -> float
        assert_eq!(group.as_string(), None); // a number is not a string
        assert_eq!(group.as_data(), None);
    }

    #[test]
    fn unsigned_coercion_rejects_negative() {
        let bytes = load("foundation/NumberInt64"); // -9_000_000_000
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_u64(), None);
    }

    #[test]
    fn unwraps_nsnumber_object_and_reads_dict_entries() {
        // NSDictionaryNested = { "arr": [..], "data": <bytes>, "num": NSNumber(7) }.
        // Keys are NSString objects; the NSNumber value exercises the unwrap path;
        // the NSData value exercises as_data on a dictionary value.
        let bytes = load("foundation/NSDictionaryNested");

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let keys: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();
        assert!(
            keys.contains(&"arr") && keys.contains(&"data") && keys.contains(&"num"),
            "{keys:?}"
        );

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let ints: Vec<i64> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_i64())
            .collect();
        assert!(ints.contains(&7), "{ints:?}"); // NSNumber(7) value, unwrapped

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let datas: Vec<&[u8]> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_data())
            .collect();
        assert!(datas.contains(&&[0x01, 0x02][..]), "{datas:?}"); // NSData value
    }
}
