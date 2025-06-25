//! Iterators for resolving properties in an [`Archived::Object`]

use std::slice::Iter;

use crate::models::{archived::Archived, class::Class, output_data::OutputData, types::Type};

/// A single resolved property from an [`Archived::Object`].
#[derive(Debug)]
pub enum Property<'a, 'b> {
    /// An object with its class metadata, class name, and nested properties iterator.
    Object {
        /// The class of the object
        class: &'a Class,
        /// The name of the class, typically a string from the type table
        name: &'a str,
        /// An iterator over the properties of this object
        data: PropertyIterator<'a, 'b>,
    },
    /// A group of properties (primitives or nested objects).
    Group(Vec<Property<'a, 'b>>),
    /// A primitive value (string, number, byte, etc.).
    Primitive(&'b OutputData<'a>),
}

/// An iterator that resolves the top-level properties of a single [`Archived::Object`].
///
/// This iterator will yield `Property` items, which can be either nested objects or primitive values.
/// It is created from an `Archived` object and its associated type table.
///
/// It is designed to traverse the properties of an object, allowing you to access nested objects and their properties recursively.
///
/// # Example
///
/// ```no_run
/// use crabstep::deserializer::typedstream::TypedStreamDeserializer;
/// use crabstep::deserializer::iter::PropertyIterator;
///
/// // Create a new `TypedStreamDeserializer` and oxidize the data to get the root index.
/// let data: &[u8] = &[];
/// let mut deserializer = TypedStreamDeserializer::new(data);
/// let root_idx = deserializer.oxidize().unwrap();
///
/// // This creates a `PropertyIterator` over the root object.
/// let root_object = deserializer.resolve_properties(root_idx).unwrap();
///
/// // Create a property iterator for the root object.
/// root_object.into_iter().for_each(|property| {
///     println!("{:?}", property);
/// });
/// ```
#[derive(Debug, Clone)]
pub struct PropertyIterator<'a, 'b> {
    object_table: &'b [Archived<'a>],
    type_table: &'b [Vec<Type<'a>>],
    property_groups: Iter<'b, Vec<OutputData<'a>>>,
}

impl<'a, 'b> PropertyIterator<'a, 'b> {
    pub(crate) fn new(
        object_table: &'b [Archived<'a>],
        type_table: &'b [Vec<Type<'a>>],
        root_object_index: usize,
    ) -> Option<Self> {
        let root_object = object_table.get(root_object_index)?;

        let properties = if let Archived::Object { data, .. } = root_object {
            data
        } else {
            return None;
        };

        Some(Self {
            object_table,
            type_table,
            property_groups: properties.iter(),
        })
    }
}

impl<'a, 'b: 'a> PropertyIterator<'a, 'b> {
    /// Collects only primitive data values from a `typedstream` using a depth-first-search over the deserialized object graph.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::deserializer::typedstream::TypedStreamDeserializer;
    ///
    /// // Create a new `TypedStreamDeserializer` and oxidize the data to get the root index.
    /// let data: &[u8] = &[];
    /// let mut deserializer = TypedStreamDeserializer::new(data);
    /// let root_idx = deserializer.oxidize().unwrap();
    ///
    /// // This creates a `PropertyIterator` over the root object.
    /// let root_obj = deserializer.resolve_properties(root_idx).unwrap();
    ///
    /// // Emit the primitive values from the root object.
    /// let primitives = root_obj.primitives();
    /// primitives.into_iter().for_each(|primitive| {
    ///     println!("{primitive}");
    /// });
    /// ```
    #[must_use]
    pub fn primitives(self) -> Vec<&'b OutputData<'a>> {
        let mut primitives = Vec::new();
        // Use an explicit stack for depth-first traversal
        let mut stack: Vec<Property<'a, 'b>> = self.collect();
        while let Some(prop) = stack.pop() {
            match prop {
                Property::Primitive(p) => primitives.push(p),
                Property::Group(mut group) => {
                    // push children in reverse to preserve order
                    while let Some(child) = group.pop() {
                        stack.push(child);
                    }
                }
                Property::Object { data, .. } => {
                    // data is a PropertyIterator; collect its items
                    let mut nested: Vec<_> = data.collect();
                    while let Some(child) = nested.pop() {
                        stack.push(child);
                    }
                }
            }
        }
        primitives.reverse();
        primitives
    }
}

impl<'a, 'b: 'a> Iterator for PropertyIterator<'a, 'b> {
    type Item = Property<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let groups = self.property_groups.next()?;

        let mut resolved = Vec::with_capacity(groups.len());

        for group in groups {
            match group {
                OutputData::Object(idx) => {
                    if let Some(Archived::Object {
                        class: cls,
                        data: _,
                    }) = self.object_table.get(*idx)
                    {
                        if let Some(Archived::Class(cls)) = self.object_table.get(*cls) {
                            let class_name = self
                                .type_table
                                .get(cls.name_index)
                                .and_then(|types| types.first())
                                .and_then(|t| match t {
                                    Type::String(name) => Some(*name),
                                    _ => None,
                                })
                                .unwrap_or("Unknown Class");
                            // recurse into that object’s own data
                            let sub_iter =
                                PropertyIterator::new(self.object_table, self.type_table, *idx)?;
                            resolved.push(Property::Object {
                                class: cls,
                                name: class_name,
                                data: sub_iter,
                            });
                        }
                    }
                }
                prim => resolved.push(Property::Primitive(prim)),
            }
        }
        Some(Property::Group(resolved))
    }
}

/// Print a resolved [`PropertyIterator`] in a human-readable tree format for debugging.
///
/// This function recursively prints all properties with proper indentation to show the nested structure
/// of the deserialized object graph.
///
/// # Arguments
///
/// * `iter` - The property iterator to print
/// * `indent` - Number of spaces to indent each level (typically 2 or 4)
///
/// # Examples
/// ```no_run
/// use crabstep::deserializer::iter::print_resolved;
/// use crabstep::deserializer::typedstream::TypedStreamDeserializer;
///
/// let mut ds = TypedStreamDeserializer::new(&[]);
/// let root = ds.oxidize().unwrap();
///
/// if let Ok(iter) = ds.resolve_properties(root) {
///     print_resolved(iter, 2);
/// }
/// ```
pub fn print_resolved(iter: PropertyIterator<'_, '_>, indent: usize) {
    for prop in iter {
        print_property(prop, indent);
    }
}

/// Print a single `ResolvedProperty` with indentation, recursing for nested data.
/// ```
pub(crate) fn print_property<'a, 'b: 'a>(prop: Property<'a, 'b>, indent: usize) {
    match prop {
        Property::Object {
            class: _,
            name,
            data,
        } => {
            // Print the object itself
            println!("{:indent$}Object: {:?}", "", name, indent = indent);
            // Recurse into its children with increased indent
            print_resolved(data, indent + 2);
        }
        Property::Group(slice) => {
            println!("{:indent$}Group:", "", indent = indent);
            // drill into every slot in the group
            for slot in slice {
                print_property(slot, indent + 2);
            }
        }
        Property::Primitive(p) => {
            println!("{:indent$}Primitive: {:?}", "", p, indent = indent);
        }
    }
}
