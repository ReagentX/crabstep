/// Represents a class stored in the `typedstream`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    /// A reference to the class name stored in the [`TypedStreamDeserializer`](crate::deserializer::typedstream::TypedStreamDeserializer)'s `type_table`
    pub name_index: usize,
    /// The encoded version of the class
    pub version: u64,
    /// The parent class reference into the [`TypedStreamDeserializer`](crate::deserializer::typedstream::TypedStreamDeserializer)'s `object_table`, if any
    pub parent_index: Option<usize>,
}

impl Class {
    /// Creates a new class with the given name, version, and optional child
    pub fn new(name: usize, version: u64, parent: Option<usize>) -> Self {
        Self {
            name_index: name,
            version,
            parent_index: parent,
        }
    }
}
