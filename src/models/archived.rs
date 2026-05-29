//! Types that can be archived into a `typedstream`

use alloc::{vec, vec::Vec};

use crate::models::{class::Class, output_data::OutputData};

/// The data attached to an [`Archived::Object`].
///
/// `typedstream` objects store their values in one or more *groups* (each group
/// is the result of decoding one type descriptor). In practice the overwhelming
/// majority of objects hold a single group containing a single value (an
/// `NSString`'s text, an `NSNumber`'s number, a reference to another object) so
/// that case is stored inline without any heap allocation. Only objects with
/// multiple groups, or a group holding multiple values, fall back to the nested
/// [`Vec`] representation.
#[derive(Debug, PartialEq)]
pub enum ObjectData<'a> {
    /// The object has no data groups.
    Empty,
    /// A single group containing a single value, stored inline. This is by far
    /// the most common shape and avoids two heap allocations per object.
    Inline(OutputData<'a>),
    /// The general case: one or more groups, each holding one or more values.
    Groups(Vec<Vec<OutputData<'a>>>),
}

impl<'a> ObjectData<'a> {
    /// Append a group that contains exactly one value.
    #[inline]
    pub(crate) fn push_one(&mut self, value: OutputData<'a>) {
        match self {
            // Common path: the object's first (and usually only) group.
            ObjectData::Empty => *self = ObjectData::Inline(value),
            ObjectData::Groups(groups) => groups.push(vec![value]),
            // Promote a previously-inline object to the general representation.
            ObjectData::Inline(_) => {
                let ObjectData::Inline(first) = core::mem::replace(self, ObjectData::Empty) else {
                    unreachable!()
                };
                *self = ObjectData::Groups(vec![vec![first], vec![value]]);
            }
        }
    }

    /// Append a group that contains multiple values.
    #[inline]
    pub(crate) fn push_many(&mut self, values: Vec<OutputData<'a>>) {
        match self {
            ObjectData::Empty => *self = ObjectData::Groups(vec![values]),
            ObjectData::Groups(groups) => groups.push(values),
            ObjectData::Inline(_) => {
                let ObjectData::Inline(first) = core::mem::replace(self, ObjectData::Empty) else {
                    unreachable!()
                };
                *self = ObjectData::Groups(vec![vec![first], values]);
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
            ObjectData::Groups(groups)
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
