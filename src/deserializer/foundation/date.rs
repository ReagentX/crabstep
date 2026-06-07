//! `as_date` / `as_unix_time`: `NSDate`.

use crate::deserializer::iter::Property;
use crate::models::output_data::OutputData;

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// An `NSDate` as seconds since the Cocoa reference epoch (2001-01-01 00:00:00
    /// UTC). Use [`as_unix_time`](Self::as_unix_time) for seconds since the Unix
    /// epoch.
    #[must_use]
    pub fn as_date(&self) -> Option<f64> {
        let mut data = self.object_in_classes(&["NSDate"])?;
        match data.next()? {
            Property::Group(group) => match group.first()? {
                Property::Primitive(OutputData::Double(seconds)) => Some(*seconds),
                _ => None,
            },
            _ => None,
        }
    }

    /// An `NSDate` as seconds since the Unix epoch (1970-01-01 00:00:00 UTC),
    /// i.e. [`as_date`](Self::as_date)` + 978_307_200.0` (the offset between the
    /// Unix and Cocoa reference epochs).
    #[must_use]
    pub fn as_unix_time(&self) -> Option<f64> {
        self.as_date().map(|seconds| seconds + 978_307_200.0)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn root_object_resolves_as_date() {
        // Root NSDate (timeIntervalSinceReferenceDate: 21692800) via `root()`.
        let bytes = load("foundation/NSDate");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        assert_eq!(ts.root().unwrap().as_date(), Some(21692800.0));
    }

    #[test]
    fn as_date_and_unix_time() {
        // NestedScalars holds NSDate(timeIntervalSinceReferenceDate: 21692800).
        let bytes = load("foundation/NestedScalars");

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let dates: Vec<f64> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_date())
            .collect();
        assert_eq!(dates, vec![21692800.0]);

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let unix: Vec<f64> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_unix_time())
            .collect();
        assert_eq!(unix, vec![1_000_000_000.0]);
    }

    #[test]
    fn scalar_accessors_reject_wrong_types() {
        let bytes = load("foundation/NumberInt");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert!(!group.is_null());
        assert_eq!(group.as_date(), None);
        assert_eq!(group.as_unix_time(), None);
        assert_eq!(group.as_url(), None);
    }
}
