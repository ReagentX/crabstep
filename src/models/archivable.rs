use crate::models::{class::Class, output_data::OutputData, types::Type};

/// Types of data that can be archived into the `typedstream`
#[derive(Debug, PartialEq)]
pub enum Archivable<'a> {
    /// An instance of a class that may contain some embedded data. `typedstream` data doesn't include property
    /// names, so data is stored in order of appearance. The class is stored in the `object_table` and
    /// the data is stored in the `data` field.
    Object(usize, Vec<Vec<OutputData<'a>>>),
    /// Some data that is likely a property on the object described by the `typedstream` but not part of a class.
    Data(Vec<OutputData<'a>>),
    /// A class referenced in the `typedstream`, usually part of an inheritance hierarchy that does not contain any data itself.
    Class(Class),
    /// A placeholder, only used when reserving a spot in the objects table for a reference to be filled with read class information.
    /// In a `typedstream`, the classes are stored in order of inheritance, so the top-level class described by the `typedstream`
    /// comes before the ones it inherits from. To preserve the order, we reserve the first slot to store the actual object's data
    /// and then later add it back to the right place.
    Placeholder,
    /// A type that made it through the parsing process without getting replaced by an object, referred to by its index in the `types_table`.
    Type(usize),
}
