//! Types that can be archived into a `typedstream`

use alloc::{vec, vec::Vec};

use crate::models::{class::Class, output_data::OutputData};

/// A data group within an [`ObjectData::Groups`] collection: the values decoded
/// from one type descriptor.
///
/// A lone value is stored inline so each dictionary key and value costs no
/// allocation; anything else lives in a [`Vec`].
#[derive(Debug, PartialEq)]
pub enum DataGroup<'a> {
    /// Exactly one value, stored inline.
    One(OutputData<'a>),
    /// Zero, or multiple, values.
    Values(Vec<OutputData<'a>>),
}

impl<'a> DataGroup<'a> {
    /// Borrow the group's values in stream order.
    #[must_use]
    pub fn as_slice(&self) -> &[OutputData<'a>] {
        match self {
            Self::One(value) => core::slice::from_ref(value),
            Self::Values(values) => values,
        }
    }

    /// The number of values in the group.
    #[must_use]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Whether the group contains no values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
}

/// The data attached to an [`Archived::Object`].
///
/// Each group holds the values decoded from one type descriptor. An object with
/// one single-value group stores that value inline; every other nonempty object
/// holds its groups in stream order.
#[derive(Debug, PartialEq)]
pub enum ObjectData<'a> {
    /// The object has no data groups.
    Empty,
    /// A single group containing a single value, stored inline. This is by far
    /// the most common shape and avoids two heap allocations per object.
    Inline(OutputData<'a>),
    /// One or more groups in stream order.
    Groups(Vec<DataGroup<'a>>),
}

impl<'a> ObjectData<'a> {
    /// Append a group in stream order.
    ///
    /// The first single-value group is stored inline; a second group of any
    /// shape promotes the object to [`Groups`](Self::Groups).
    #[inline]
    pub(crate) fn push(&mut self, group: DataGroup<'a>) {
        match self {
            ObjectData::Groups(groups) => groups.push(group),
            // Common path: the object's first (and usually only) group.
            ObjectData::Empty => {
                *self = match group {
                    DataGroup::One(value) => ObjectData::Inline(value),
                    group => ObjectData::Groups(vec![group]),
                }
            }
            // Promote a previously-inline object to the general representation.
            ObjectData::Inline(_) => {
                let ObjectData::Inline(first) = core::mem::replace(self, ObjectData::Empty) else {
                    unreachable!()
                };
                *self = ObjectData::Groups(vec![DataGroup::One(first), group]);
            }
        }
    }

    /// Build an [`ObjectData`] from the nested group representation, normalizing
    /// to the inline form when there is a single single-value group. Used by
    /// tests to express expected object data in the pre-existing nested style.
    #[cfg(test)]
    pub(crate) fn from_groups(mut groups: Vec<Vec<OutputData<'a>>>) -> Self {
        if groups.is_empty() {
            ObjectData::Empty
        } else if groups.len() == 1 && groups[0].len() == 1 {
            ObjectData::Inline(groups.pop().unwrap().pop().unwrap())
        } else {
            ObjectData::Groups(
                groups
                    .into_iter()
                    .map(|mut group| {
                        if group.len() == 1 {
                            DataGroup::One(group.pop().unwrap())
                        } else {
                            DataGroup::Values(group)
                        }
                    })
                    .collect(),
            )
        }
    }

    /// The number of data groups in this object.
    #[must_use]
    pub fn group_count(&self) -> usize {
        match self {
            ObjectData::Empty => 0,
            ObjectData::Inline(_) => 1,
            ObjectData::Groups(groups) => groups.len(),
        }
    }
}

/// Types of data that can be archived into the `typedstream`
#[derive(Debug, PartialEq)]
pub enum Archived<'a> {
    /// An instance of a class that may contain some embedded data. `typedstream` data doesn't include property
    /// names, so data is stored in order of appearance. The class is stored in the [`object_table`](crate::deserializer::typedstream::TypedStreamDeserializer::object_table) and
    /// the data is stored in the `data` field.
    Object {
        /// Index into [`object_table`](crate::deserializer::typedstream::TypedStreamDeserializer::object_table) for this object’s class.
        class: usize,
        /// The data groups for this object. Each group represents a logically
        /// related set of values; for example, a class may have multiple
        /// properties, each represented as a group.
        data: ObjectData<'a>,
    },
    /// A class referenced in the `typedstream`, usually part of an inheritance hierarchy that does not contain any data itself.
    Class(Class),
    /// A placeholder, only used when reserving a spot in the objects table for a reference to be filled with read class information.
    /// In a `typedstream`, the classes are stored in order of inheritance, so the top-level class described by the `typedstream`
    /// comes before the ones it inherits from. To preserve the order, we reserve the first slot to store the actual object's data
    /// and then later add it back to the right place.
    Placeholder,
    /// An embedded type that describes the [`Type`](crate::models::types::Type) of the subsequent bytes, referred to by its index in the [`type_table`](crate::deserializer::typedstream::TypedStreamDeserializer::type_table).
    Type(usize),
}
