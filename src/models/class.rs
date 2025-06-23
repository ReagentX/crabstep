/// Represents a class stored in the `typedstream`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    /// A reference to the class name stored in the [`type_table`](crate::deserializer::typedstream::TypedStreamDeserializer::type_table)
    pub name_index: usize,
    /// The encoded version of the class
    pub version: u64,
    /// The parent class reference into the [`object_table`](crate::deserializer::typedstream::TypedStreamDeserializer::object_table), if any
    pub parent_index: Option<usize>,
}

impl Class {
    /// Creates a new class with the given name, version, and optional child
    #[must_use]
    pub fn new(name: usize, version: u64, parent: Option<usize>) -> Self {
        Self {
            name_index: name,
            version,
            parent_index: parent,
        }
    }
}
