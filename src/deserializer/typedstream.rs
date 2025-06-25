/*!
 Logic used to deserialize data from a `typedstream`, focussing specifically on [`NSAttributedString`](https://developer.apple.com/documentation/foundation/nsattributedstring).

 A writeup about the reverse engineering of `typedstream` can be found [here](https://chrissardegna.com/blog/reverse-engineering-apples-typedstream-format/).
*/

use std::collections::HashSet;

use crate::{
    constants::{EMPTY, END, START},
    deserializer::{
        header::validate_header,
        iter::PropertyIterator,
        number::{read_double, read_float, read_signed_int, read_unsigned_int},
        read::{read_byte_at, read_exact_bytes, read_pointer},
        string::read_string,
    },
    error::{Result, TypedStreamError},
    models::{archivable::Archived, class::Class, output_data::OutputData, types::Type},
};

/// Contains logic and data used to deserialize data from a `typedstream`.
///
/// `typedstream` is a binary serialization format developed by `NeXTSTEP` and later adopted by Apple.
/// It's designed to serialize and deserialize complex object graphs and data structures in C and Objective-C.
///
/// A `typedstream` begins with a header that includes format version and architecture information,
/// followed by a stream of typed data elements. Each element is prefixed with type information,
/// allowing the [`TypedStreamDeserializer`] to understand the original data structures.
pub struct TypedStreamDeserializer<'a> {
    /// The `typedstream` we want to parse
    pub data: &'a [u8],
    /// The current index we are at in the stream
    pub(crate) position: usize,
    /// As we parse the `typedstream`, build a table of seen [`Type`]s to reference in the future
    ///
    /// The first time a [`Type`] is seen, it is present in the stream literally,
    /// but afterwards are only referenced by index in order of appearance.
    pub type_table: Vec<Vec<Type<'a>>>,
    /// As we parse the `typedstream`, build a table of seen archivable data to reference in the future
    pub object_table: Vec<Archived<'a>>,
    /// We want to copy embedded types the first time they are seen, even if the types were resolved through references
    pub(crate) seen_embedded_types: HashSet<usize>,
}

impl<'a> TypedStreamDeserializer<'a> {
    /// Create a new `TypedStreamDeserializer` for the provided byte slice.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::deserializer::typedstream::TypedStreamDeserializer;
    ///
    /// let data: &[u8] = &[];
    /// let deserializer = TypedStreamDeserializer::new(data);
    /// ```
    #[must_use]
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            type_table: Vec::with_capacity(16),
            object_table: Vec::with_capacity(32),
            seen_embedded_types: HashSet::with_capacity(8),
        }
    }

    /// Creates an iterator that resolves the properties of the root object in the `typedstream`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::deserializer::typedstream::TypedStreamDeserializer;
    ///
    /// let data: &[u8] = &[];
    /// let mut deserializer = TypedStreamDeserializer::new(data);
    ///
    /// // Walk the object root, printing each primitive value
    /// deserializer.iter_root().into_iter().for_each(|prop| {
    ///    prop.primitives().into_iter().for_each(|data| println!("{data}"));
    /// });
    /// ```
    pub fn iter_root(&mut self) -> Result<PropertyIterator<'a, '_>> {
        let root = self.oxidize()?;
        self.resolve_properties(root)
    }

    /// Parse the typed stream, consuming header and objects, returning the index of the top-level archived object.
    ///
    /// # Errors
    ///
    /// Returns a [`TypedStreamError`] if parsing fails or the stream ends unexpectedly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let mut deserializer = TypedStreamDeserializer::new(&[]);
    /// let result = deserializer.oxidize();
    /// ```
    pub fn oxidize(&mut self) -> Result<usize> {
        let mut obj = None;
        let validation = validate_header(self.data)?;

        // Advance by the number of bytes consumed by the header validation
        self.position += validation.bytes_consumed;

        // while self.position <= self.data.len() {
        let found_type = self.read_type(false)?;

        if let Some(type_index) = found_type {
            // Read the types at the specified index
            obj = self.read_types(type_index)?;
        }

        match obj
            .ok_or(TypedStreamError::UnmatchedEnd)?
            .first()
            .ok_or(TypedStreamError::UnmatchedEnd)?
        {
            OutputData::Object(idx) => Ok(*idx),
            _ => Err(TypedStreamError::UnmatchedEnd),
        }
    }

    /// Creates an iterator that resolves the properties of an object
    /// at the specified index in the `object_table`, preserving nested structure.
    ///
    /// This should be called after [`oxidize()`](Self::oxidize).
    ///
    /// # Arguments
    ///
    /// * `root_object_index` - Index of the object in the deserializer's `object_table` to iterate.
    ///
    /// # Errors
    ///
    /// Returns [`TypedStreamError::InvalidPointer`] if the index is not a valid object reference.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let mut ts = TypedStreamDeserializer::new(&[]);
    /// let root = ts.oxidize().unwrap();
    ///
    /// let iter = ts.resolve_properties(root).unwrap();
    /// ```
    pub fn resolve_properties(&self, root_object_index: usize) -> Result<PropertyIterator<'a, '_>> {
        PropertyIterator::new(&self.object_table, &self.type_table, root_object_index)
            .ok_or(TypedStreamError::InvalidPointer(root_object_index as u8))
    }

    /// Reads the next byte from the stream, advancing the position.
    fn consume_current_byte(&mut self) -> Result<&u8> {
        let byte = read_byte_at(self.data, self.position)?;
        self.position += 1;
        Ok(byte)
    }

    fn read_unsigned_int(&mut self) -> Result<u64> {
        let unsigned_int = read_unsigned_int(&self.data[self.position..])?;
        self.position += unsigned_int.bytes_consumed;
        Ok(unsigned_int.value)
    }

    /// [`Archivable`] data can be embedded on a class or in a C String marked as [`Type::EmbeddedData`]
    fn read_embedded_type(&mut self) -> Result<Option<usize>> {
        match *self.consume_current_byte()? {
            START => {
                // 0x84 indicates the start of embedded data
                self.read_type(true)
            }
            EMPTY => Ok(None),
            ptr => {
                let pointer = read_pointer(&ptr)?.map(|v| v as usize);
                if let Some(Archived::Type(idx)) = self.object_table.get(pointer.value) {
                    Ok(Some(*idx))
                } else {
                    Err(TypedStreamError::InvalidPointer(pointer.value as u8))
                }
            }
        }
    }

    fn read_string(&mut self) -> Result<usize> {
        let current_byte = *self.consume_current_byte()?;
        match current_byte {
            START => {
                let string_data = read_string(&self.data[self.position..])?;
                self.position += string_data.bytes_consumed;
                self.type_table
                    .push(vec![Type::new_string(string_data.value)]);
                Ok(self.type_table.len() - 1)
            }
            EMPTY => Err(TypedStreamError::EmptyString),
            ptr => {
                let pointer = read_pointer(&ptr)?.map(|v| v as usize);
                if let Some(Type::String(_)) = self
                    .type_table
                    .get(pointer.value)
                    .and_then(|inner| inner.first())
                {
                    Ok(pointer.value)
                } else {
                    Err(TypedStreamError::InvalidPointer(pointer.value as u8))
                }
            }
        }
    }

    fn read_class(&mut self) -> Result<Option<usize>> {
        // index of the first START we encounter (the bottom-most child)
        let mut first_new: Option<usize> = None;
        // index of the most recently pushed class (current “child”)
        let mut prev_new: Option<usize> = None;
        // parent for the outer-most new class (set by EMPTY or a pointer)
        let final_parent: Option<usize>;

        loop {
            match *self.consume_current_byte()? {
                START => {
                    let name_idx = self.read_string()?;
                    let version = self.read_unsigned_int()?;

                    // Append the new class with no parent yet
                    let idx = self.object_table.len();
                    self.object_table
                        .push(Archived::Class(Class::new(name_idx, version, None)));

                    // The class we just appended (*idx*) is the **parent** of the
                    // class we appended in the previous iteration (*prev_new*)
                    if let Some(child_idx) = prev_new {
                        if let Archived::Class(ref mut child_cls) = self.object_table[child_idx] {
                            child_cls.parent_index = Some(idx);
                        }
                    }

                    // remember the first class we ever pushed
                    first_new.get_or_insert(idx);
                    // and mark the current class as “last pushed”
                    prev_new = Some(idx);
                }
                EMPTY => {
                    final_parent = None;
                    break;
                }
                ptr => {
                    let pointer = read_pointer(&ptr)?;

                    final_parent = Some(pointer.value as usize);
                    break;
                }
            }
        }

        // If we did not create any new classes, just return what we found.
        let first_idx = match first_new {
            None => return Ok(final_parent),
            Some(i) => i,
        };

        // Patch the outer-most newly created class so that it points to the
        // already-existing parent (or to `None` if EMPTY terminated the list).
        if let Some(outer_idx) = prev_new {
            if let Archived::Class(ref mut outer_cls) = self.object_table[outer_idx] {
                outer_cls.parent_index = final_parent;
            }
        }

        // Return the index of the bottom-most child we created first.
        Ok(Some(first_idx))
    }

    fn read_object(&mut self) -> Result<usize> {
        match *read_byte_at(self.data, self.position)? {
            START => {
                let placeholder_index = self.object_table.len();
                // This placeholder will be replaced with the actual object data once we read the class
                self.object_table.push(Archived::Placeholder);
                // Advance the position to the next byte, which should be the start of a class
                self.position += 1;

                if let Some(cls) = self.read_class()? {
                    self.object_table[placeholder_index] = Archived::Object {
                        class: cls,
                        data: Vec::with_capacity(8),
                    };
                    while self.position < self.data.len()
                        && *read_byte_at(self.data, self.position)? != END
                    {
                        // Read the next type, which should be an object
                        if let Some(next_index) = self.read_type(false)? {
                            // Recursively read the types for this object
                            if let Some(data) = self.read_types(next_index)? {
                                if let Some(Archived::Object {
                                    class: _,
                                    data: data_vec,
                                }) = self.object_table.get_mut(placeholder_index)
                                {
                                    // Add the data to the object
                                    data_vec.push(data);
                                }
                            }
                        }
                    }
                }
                Ok(placeholder_index)
            }
            ptr => {
                let pointer = read_pointer(&ptr)?;
                Ok(pointer.value as usize)
            }
        }
    }

    /// Reads numeric types (signed, unsigned, float, double) and returns the corresponding `OutputData`
    fn read_number(&mut self, ty: &Type<'a>) -> Result<OutputData<'a>> {
        match ty {
            Type::SignedInt => {
                let signed_int = read_signed_int(&self.data[self.position..])?;
                self.position += signed_int.bytes_consumed;
                Ok(OutputData::SignedInteger(signed_int.value as i64))
            }
            Type::UnsignedInt => {
                let unsigned_int = read_unsigned_int(&self.data[self.position..])?;
                self.position += unsigned_int.bytes_consumed;
                Ok(OutputData::UnsignedInteger(unsigned_int.value))
            }
            Type::Float => {
                let float = read_float(&self.data[self.position..])?;
                self.position += float.bytes_consumed;
                Ok(OutputData::Float(float.value as f32))
            }
            Type::Double => {
                let double = read_double(&self.data[self.position..])?;
                self.position += double.bytes_consumed;
                Ok(OutputData::Double(double.value as f64))
            }
            _ => unreachable!(),
        }
    }

    fn read_types(&mut self, types_index: usize) -> Result<Option<Vec<OutputData<'a>>>> {
        // Clone types to avoid holding an immutable borrow on self during parsing
        let types = self.type_table[types_index].clone();
        let mut out_v = Vec::with_capacity(types.len());

        for ty in types {
            match ty {
                Type::Utf8String => {
                    let str_data = read_string(&self.data[self.position..])?;
                    self.position += str_data.bytes_consumed;
                    out_v.push(OutputData::String(str_data.value));
                }
                Type::EmbeddedData => {
                    if let Some(idx) = self.read_embedded_type()? {
                        self.position += 1;
                        return self.read_types(idx);
                    }
                    return Ok(None);
                }
                Type::Object => {
                    let obj_idx = self.read_object()?;
                    self.position += 1;
                    out_v.push(OutputData::Object(obj_idx));
                }
                Type::String(s) => {
                    out_v.push(OutputData::String(s));
                }
                Type::Array(length) => {
                    let array_data = read_exact_bytes(&self.data[self.position..], length)?;
                    self.position += length;
                    out_v.push(OutputData::Array(array_data));
                }
                Type::Unknown(byte) => {
                    // Read a single byte for unknown data
                    out_v.push(OutputData::Byte(byte));
                }
                // numeric types
                Type::SignedInt | Type::UnsignedInt | Type::Float | Type::Double => {
                    let val = self.read_number(&ty)?;
                    out_v.push(val);
                }
            }
        }

        Ok(Some(out_v))
    }

    /// Gets the current type from the stream, either by reading it from the stream or reading it from
    /// the specified index of [`Self::type_table`]. Returns an index into the types table
    /// to avoid cloning large type vectors.
    fn read_type(&mut self, is_embedded_type: bool) -> Result<Option<usize>> {
        let byte = *self.consume_current_byte()?;

        match byte {
            START => {
                // Get the type of the object
                let new_types = Type::read_new_type(&self.data[self.position..])?;
                let new_type_index = self.type_table.len();
                // Embedded data is stored as a String in the objects table
                if is_embedded_type {
                    self.object_table.push(Archived::Type(new_type_index));
                    // We only want to include the first embedded reference tag, not subsequent references to the same embed
                    self.seen_embedded_types
                        .insert(self.object_table.len().saturating_sub(1));
                }

                self.type_table.push(new_types.value);
                self.position += new_types.bytes_consumed;
                Ok(Some(self.type_table.len() - 1))
            }
            EMPTY => Ok(None),
            END => Ok(None),
            ptr => {
                let pointer = read_pointer(&ptr)?;
                let ref_tag = pointer.value as usize;

                if ref_tag as usize >= self.type_table.len() {
                    return Ok(None);
                }

                if is_embedded_type {
                    // We only want to include the first embedded reference tag, not subsequent references to the same embed
                    if !self.seen_embedded_types.contains(&ref_tag)
                        && self.type_table.get(ref_tag as usize).is_some()
                    {
                        self.object_table.push(Archived::Type(ref_tag));
                        self.seen_embedded_types.insert(ref_tag);
                    }
                }

                Ok(Some(ref_tag as usize))
            }
        }
    }
}
