//! `as_data`: the data cluster.

use crate::deserializer::foundation::names::DATA_CLASSES;
use crate::deserializer::iter::Property;
use crate::models::output_data::OutputData;

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// The raw bytes of an `NSData` / `NSMutableData`.
    ///
    /// crabstep does not interpret the bytes — they may be a `bplist00`, a
    /// compressed blob, an image, etc. The caller decides what they are.
    #[must_use]
    pub fn as_data(&self) -> Option<&'a [u8]> {
        let data = self.object_in_classes(DATA_CLASSES)?;
        for prop in data {
            if let Property::Group(group) = prop {
                for child in group {
                    if let Property::Primitive(OutputData::Array(bytes)) = child {
                        return Some(bytes);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn as_data_reads_both_data_variants() {
        // NSArray([NSData [1,2], NSMutableData [3,4,5]])
        let bytes = load("foundation/NestedData");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let datas: Vec<&[u8]> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_data())
            .collect();

        assert_eq!(datas.len(), 2, "{datas:?}");
        assert!(datas.contains(&&[0x01, 0x02][..]), "{datas:?}");
        assert!(datas.contains(&&[0x03, 0x04, 0x05][..]), "{datas:?}");
    }
}
