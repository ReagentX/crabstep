//! Helpers shared across the Foundation accessors.

use crate::deserializer::iter::{Property, PropertyIterator};

impl<'a, 'b: 'a> Property<'a, 'b> {
    /// If `self` is a group whose first item is an object whose class is in
    /// `classes`, return that object's data iterator.
    pub(crate) fn object_in_classes(&self, classes: &[&str]) -> Option<PropertyIterator<'a, 'b>> {
        let Property::Group(group) = self else {
            return None;
        };
        match group.first()? {
            Property::Object { name, data, .. } if classes.contains(&name) => Some(data),
            _ => None,
        }
    }
}

/// Read the leading count group of a container's data, returning the remaining
/// iterator (positioned at the first element/entry) and the declared count.
pub(crate) fn split_count<'a, 'b: 'a>(
    mut data: PropertyIterator<'a, 'b>,
) -> Option<(PropertyIterator<'a, 'b>, usize)> {
    let count = data.next()?;
    let len = usize::try_from(count.as_i64().unwrap_or(0)).unwrap_or(0);
    Some((data, len))
}
