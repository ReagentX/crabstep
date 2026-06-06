//! The `class_name` escape hatch.

use crate::deserializer::iter::Property;

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
    use alloc::vec::Vec;

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::iter::Property;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn class_name_resolves_element_classes() {
        // Iterating an `NSArray` yields one group per element; each element group's
        // `class_name()` is the element's class, and the bare count group is None.
        let bytes = load("foundation/NSArray");
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
        let bytes = load("foundation/NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let object_names: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| match group {
                Property::Group(g) => g.first(),
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
        let bytes = load("foundation/NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let has_primitive_group = ts
            .resolve_properties(root)
            .unwrap()
            .any(|group| group.class_name().is_none());

        assert!(has_primitive_group);
    }
}
