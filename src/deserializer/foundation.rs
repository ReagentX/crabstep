/*!
Typed accessors for common Apple [Foundation](https://developer.apple.com/documentation/foundation) classes.

Enabled by the `foundation` cargo feature. These methods interpret the generic
[`Property`] tree as specific Foundation types (`NSString`, `NSDictionary`, …)
so consumers do not have to re-implement class-name matching (and re-discover
its footguns, e.g. that the data cluster archives as both `NSData` and
`NSMutableData`.

This feature is purely for convenience: the parser and the [`Property`]/
[`OutputData`](crate::models::output_data::OutputData) model are unchanged
whether or not it is enabled, and any class not modeled here stays reachable
through [`Property::Object`], so nothing is ever lost.

Accessors are called on a *group-level* [`Property`]: the value yielded while
iterating an object's properties.
*/

use crate::deserializer::iter::Property;

// Cluster class-name sets. Foundation archives several types as both an immutable
// and a mutable variant (and the data cluster archives as both `NSData` and
// `NSMutableData`); matching all variants in one place keeps the footgun
// centralized. Consumed by the scalar/container accessors added from Phase 2 on.
#[allow(dead_code)]
pub(crate) const STRING_CLASSES: &[&str] = &["NSString", "NSMutableString", "NSAttributedString"];
#[allow(dead_code)]
pub(crate) const DATA_CLASSES: &[&str] = &["NSData", "NSMutableData"];
#[allow(dead_code)]
pub(crate) const ARRAY_CLASSES: &[&str] = &["NSArray", "NSMutableArray"];
#[allow(dead_code)]
pub(crate) const DICT_CLASSES: &[&str] = &["NSDictionary", "NSMutableDictionary"];
#[allow(dead_code)]
pub(crate) const SET_CLASSES: &[&str] = &["NSSet", "NSMutableSet"];

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// The Objective-C class name of the object this property refers to, if any.
    ///
    /// Resolves whether `self` is a [`Property::Object`] directly or a
    /// [`Property::Group`] whose first element is an object. This is the escape
    /// hatch for classes the `foundation` feature does not model: the class name
    /// plus the raw subtree (via [`Property::Object`]) are always available, so a
    /// consumer can handle the long tail of app-specific classes itself.
    #[must_use]
    pub fn class_name(&self) -> Option<&'a str> {
        match self {
            Property::Object { name, .. } => Some(*name),
            Property::Group(group) => match group.first()? {
                Property::Object { name, .. } => Some(name),
                _ => None,
            },
            Property::Primitive(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::{vec, vec::Vec};
    use std::{env::current_dir, fs::File, io::Read};

    use crate::deserializer::typedstream::TypedStreamDeserializer;

    fn load(name: &str) -> Vec<u8> {
        let path = current_dir()
            .unwrap()
            .join("src/test_data/foundation")
            .join(name);
        let mut file =
            File::open(&path).unwrap_or_else(|e| panic!("opening fixture {path:?}: {e}"));
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn class_name_resolves_element_classes() {
        // Iterating an `NSArray` yields one group per element; each element group's
        // class_name() is the element's class, and the bare count group is None.
        let bytes = load("NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let names: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.class_name())
            .collect();

        assert!(names.contains(&"NSString"), "names: {names:?}");
        assert!(names.contains(&"NSNumber"), "names: {names:?}");
    }

    #[test]
    fn class_name_on_direct_object() {
        // `class_name()` also works when called on a [`Property::Object`] directly
        // (e.g. after stepping into a group with `iter().next()`).
        let bytes = load("NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let object_names: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| match group {
                crate::deserializer::iter::Property::Group(g) => g.first(),
                _ => None,
            })
            .filter_map(|inner| inner.class_name())
            .collect();

        assert!(
            object_names.contains(&"NSString"),
            "names: {object_names:?}"
        );
    }

    #[test]
    fn class_name_is_none_for_primitive() {
        // The `NSArray`'s count group is a bare primitive, no class.
        let bytes = load("NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let has_primitive_group = ts
            .resolve_properties(root)
            .unwrap()
            .any(|group| group.class_name().is_none());

        assert!(has_primitive_group);
    }
}
