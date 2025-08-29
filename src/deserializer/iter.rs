//! Iterators for resolving properties in an [`Archived::Object`]

use core::slice::Iter;

use alloc::vec::Vec;

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

        let Archived::Object {
            data: properties, ..
        } = root_object
        else {
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
    /// Note: There is a max depth of 100 and a max item limit of 1,000,000.
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
        self.primitives_with_limits(100, 1_000_000)
    }

    /// Collects primitive data values with safety limits to prevent infinite loops.
    ///
    /// # Arguments
    ///
    /// * `max_depth` - Maximum depth to traverse (prevents infinite recursion on cycles)
    /// * `max_items` - Maximum total items to process (prevents runaway expansion)
    #[must_use]
    pub fn primitives_with_limits(
        self,
        max_depth: usize,
        max_items: usize,
    ) -> Vec<&'b OutputData<'a>> {
        let mut primitives = Vec::new();
        let mut processed_items = 0;

        // Use an explicit stack for depth-first traversal with depth tracking
        let initial_props: Vec<Property<'a, 'b>> = self.collect();
        let mut stack: Vec<(Property<'a, 'b>, usize)> =
            initial_props.into_iter().map(|p| (p, 0)).collect();

        while let Some((prop, depth)) = stack.pop() {
            // Safety checks to prevent infinite expansion
            if processed_items >= max_items {
                break;
            }
            if depth >= max_depth {
                continue;
            }

            processed_items += 1;

            match prop {
                Property::Primitive(p) => primitives.push(p),
                Property::Group(mut group) => {
                    // push children in reverse to preserve order
                    while let Some(child) = group.pop() {
                        stack.push((child, depth + 1));
                    }
                }
                Property::Object { data, .. } => {
                    // data is a PropertyIterator; collect its items
                    let mut nested: Vec<_> = data.collect();
                    while let Some(child) = nested.pop() {
                        stack.push((child, depth + 1));
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
                        && let Some(Archived::Class(cls)) = self.object_table.get(*cls)
                    {
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
                prim => resolved.push(Property::Primitive(prim)),
            }
        }
        Some(Property::Group(resolved))
    }
}

/// Print a resolved [`PropertyIterator`] in a human-readable tree format for debugging.
///
/// This function iteratively prints all properties with proper indentation to show the nested structure
/// of the deserialized object graph. Uses an explicit stack to avoid stack overflow for large structures.
///
/// Note: There is a max depth of 100 and a max item limit of 1,000,000.
///
/// # Arguments
///
/// * `iter` - The property iterator to print
/// * `indent` - Number of spaces to indent each level (typically `2` or `4`)
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
///
/// This function is intended for debugging purposes. Example output appears as follows:
///
/// ```txt
///   Group:
///     Object: "NSMutableString"
///       Group:
///         Primitive: String("Noter test")
///   Group:
///     Primitive: SignedInteger(1)
///     Primitive: UnsignedInteger(10)
///   Group:
///     Object: "NSDictionary"
///       Group:
///         Primitive: SignedInteger(1)
///       Group:
///         Object: "NSString"
///           Group:
///             Primitive: String("__kIMMessagePartAttributeName")
///       Group:
///         Object: "NSNumber"
///           Group:
///             Primitive: SignedInteger(0)
/// ```
#[cfg(any(feature = "std", test))]
pub fn print_resolved(iter: PropertyIterator<'_, '_>, indent: usize) {
    print_resolved_with_limits(iter, indent, 100, 1_000_000);
}

/// Print a resolved [`PropertyIterator`] with depth and item limits to prevent infinite expansion.
///
/// # Arguments
///
/// * `iter` - The property iterator to print
/// * `indent` - Number of spaces to indent each level
/// * `max_depth` - Maximum depth to traverse (prevents infinite recursion on cycles)
/// * `max_items` - Maximum total items to print (prevents runaway output)
#[cfg(any(feature = "std", test))]
fn print_resolved_with_limits(
    iter: PropertyIterator<'_, '_>,
    indent: usize,
    max_depth: usize,
    max_items: usize,
) {
    extern crate std;
    use std::println;
    // Use an explicit stack to avoid recursion and potential stack overflow
    let mut stack: Vec<(Property<'_, '_>, usize)> = Vec::new();
    let mut items_printed = 0;

    // Push all properties from the iterator onto the stack with their indent level
    for prop in iter {
        stack.push((prop, indent));
    }

    // Process the stack
    while let Some((prop, current_indent)) = stack.pop() {
        // Safety checks to prevent infinite expansion
        if items_printed >= max_items {
            println!(
                "{:indent$}... (truncated after {max_items} items)",
                "",
                indent = current_indent
            );
            break;
        }

        let depth = (current_indent - indent) / 2;
        if depth >= max_depth {
            println!(
                "{:indent$}... (max depth {max_depth} reached)",
                "",
                indent = current_indent
            );
            continue;
        }

        items_printed += 1;

        match prop {
            Property::Object {
                class: _,
                name,
                data,
            } => {
                // Print the object itself
                println!("{:indent$}Object: {:?}", "", name, indent = current_indent);
                // Push its children onto the stack with increased indent (in reverse order)
                let children: Vec<_> = data.collect();
                for child in children.into_iter().rev() {
                    stack.push((child, current_indent + 2));
                }
            }
            Property::Group(group) => {
                println!("{:indent$}Group:", "", indent = current_indent);
                // Push every slot in the group onto the stack with increased indent (in reverse order)
                for slot in group.into_iter().rev() {
                    stack.push((slot, current_indent + 2));
                }
            }
            Property::Primitive(p) => {
                println!("{:indent$}Primitive: {:?}", "", p, indent = current_indent);
            }
        }
    }
}
