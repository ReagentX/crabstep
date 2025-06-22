/*!
 Logic used to deserialize data from a `typedstream`, focussing specifically on [`NSAttributedString`](https://developer.apple.com/documentation/foundation/nsattributedstring).

 A writeup about the reverse engineering of `typedstream` can be found [here](https://chrissardegna.com/blog/reverse-engineering-apples-typedstream-format/).
*/

use std::collections::HashSet;

use crate::{
    constants::{EMPTY, END, START},
    deserializer::{
        header::validate_header,
        number::{read_double, read_float, read_signed_int, read_unsigned_int},
        read::{read_byte_at, read_exact_bytes, read_pointer},
        string::read_string,
    },
    error::{Result, TypedStreamError},
    models::{archivable::Archivable, class::Class, output_data::OutputData, types::Type},
};

#[derive(Debug, PartialEq)]
pub enum ParsedData {
    Object(Vec<ParsedData>),
    Data(u8),
    Null,
}

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
    pub position: usize,
    /// As we parse the `typedstream`, build a table of seen [`Type`]s to reference in the future
    ///
    /// The first time a [`Type`] is seen, it is present in the stream literally,
    /// but afterwards are only referenced by index in order of appearance.
    pub types_table: Vec<Vec<Type<'a>>>,
    /// As we parse the `typedstream`, build a table of seen archivable data to reference in the future
    object_table: Vec<Archivable<'a>>,
    /// We want to copy embedded types the first time they are seen, even if the types were resolved through references
    seen_embedded_types: HashSet<usize>,
}

impl<'a> TypedStreamDeserializer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            types_table: Vec::with_capacity(16),
            object_table: Vec::with_capacity(32),
            seen_embedded_types: HashSet::with_capacity(8),
        }
    }

    /// Parses the `typedstream` and extracts the data, returning a vector of parsed data.
    pub fn oxidize(&mut self) -> Result<&Archivable<'a>> {
        let validation = validate_header(self.data)?;

        // Advance by the number of bytes consumed by the header validation
        self.position += validation.bytes_consumed;

        // while self.position <= self.data.len() {
        let found_type = self.read_type(false)?;
        println!(
            "Found type at: {:?}: {:?}",
            found_type,
            self.types_table.get(found_type.unwrap())
        );

        if let Some(type_index) = found_type {
            // Read the types at the specified index
            let obj = self.read_types(type_index)?;
            println!("End of object: {:?}", obj);
        }

        self.object_table
            .first()
            .ok_or(TypedStreamError::UnexpectedEnd)
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
        println!("Reading embedded data at position: 0x{:x}", self.position);
        match *self.consume_current_byte()? {
            START => {
                // 0x84 indicates the start of embedded data
                println!(
                    "Found embedded data start at position: 0x{:x}",
                    self.position
                );
                self.read_type(true)
            }
            EMPTY => Ok(None),
            ptr => {
                let pointer = read_pointer(&ptr)?.map(|v| v as usize);
                if let Some(Archivable::Type(idx)) = self.object_table.get(pointer.value) {
                    Ok(Some(pointer.value))
                } else {
                    Err(TypedStreamError::InvalidPointer(pointer.value as u8))
                }
            }
        }
    }

    fn read_string(&mut self) -> Result<usize> {
        let current_byte = *self.consume_current_byte()?;
        println!("Reading string at position: 0x{:x}", self.position);
        match current_byte {
            START => {
                let string_data = read_string(&self.data[self.position..])?;
                self.position += string_data.bytes_consumed;
                self.types_table
                    .push(vec![Type::new_string(string_data.value)]);
                Ok(self.types_table.len() - 1)
            }
            EMPTY => {
                println!("Found empty string at position: 0x{:x}", self.position - 1);
                Err(TypedStreamError::EmptyString)
            }
            ptr => {
                let pointer = read_pointer(&ptr)?.map(|v| v as usize);
                if let Some(Type::String(_)) = self
                    .types_table
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
        println!("Reading class at position: 0x{:x}", self.position);

        // index of the first START we encounter (the bottom-most child)
        let mut first_new: Option<usize> = None;
        // index of the most recently pushed class (current “child”)
        let mut prev_new: Option<usize> = None;
        // parent for the outer-most new class (set by EMPTY or a pointer)
        let final_parent: Option<usize>;

        loop {
            match *self.consume_current_byte()? {
                START => {
                    println!("new at 0x{:x}", self.position);
                    let name_idx = self.read_string()?;
                    let version = self.read_unsigned_int()?;

                    // Append the new class with no parent yet
                    let idx = self.object_table.len();
                    self.object_table
                        .push(Archivable::Class(Class::new(name_idx, version, None)));

                    // The class we just appended (*idx*) is the **parent** of the
                    // class we appended in the previous iteration (*prev_new*)
                    if let Some(child_idx) = prev_new {
                        if let Archivable::Class(ref mut child_cls) = self.object_table[child_idx] {
                            child_cls.parent_index = Some(idx);
                        }
                    }

                    // remember the first class we ever pushed
                    first_new.get_or_insert(idx);
                    // and mark the current class as “last pushed”
                    prev_new = Some(idx);
                }
                EMPTY => {
                    println!("final class found!");
                    final_parent = None;
                    break;
                }
                ptr => {
                    println!("pointer");
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
            if let Archivable::Class(ref mut outer_cls) = self.object_table[outer_idx] {
                outer_cls.parent_index = final_parent;
            }
        }

        // Return the index of the bottom-most child we created first.
        Ok(Some(first_idx))
    }

    fn read_object(&mut self) -> Result<usize> {
        println!("Reading object type at position: 0x{:x}", self.position);

        match *read_byte_at(self.data, self.position)? {
            START => {
                let placeholder_index = self.object_table.len();
                // This placeholder will be replaced with the actual object data once we read the class
                self.object_table.push(Archivable::Placeholder);
                // Advance the position to the next byte, which should be the start of a class
                self.position += 1;

                if let Some(cls) = self.read_class()? {
                    self.object_table[placeholder_index] =
                        Archivable::Object(cls, Vec::with_capacity(8));
                    while self.position < self.data.len()
                        && *read_byte_at(self.data, self.position)? != END
                    {
                        // Read the next type, which should be an object
                        println!(
                            "inside object: 0x{:x} -> {:x}",
                            self.position,
                            read_byte_at(self.data, self.position)?
                        );
                        if let Some(next_index) = self.read_type(false)? {
                            // Recursively read the types for this object
                            if let Some(data) = self.read_types(next_index)? {
                                if let Some(Archivable::Object(_, data_vec)) =
                                    self.object_table.get_mut(placeholder_index)
                                {
                                    // Add the data to the object
                                    data_vec.push(data);
                                }
                            }
                        }
                    }
                }
                println!("End of object found at position: 0x{:x}", self.position);
                Ok(placeholder_index)
            }
            ptr => {
                println!("Reading object pointer at position: 0x{:x}", self.position);
                let pointer = read_pointer(&ptr)?;
                // self.position += pointer.bytes_consumed;
                Ok(pointer.value as usize)
            }
        }
    }

    fn read_types(&mut self, types_index: usize) -> Result<Option<Vec<OutputData<'a>>>> {
        // Get the types at the specified index
        let types = &self.types_table[types_index];

        let count = self.types_table[types_index].len();
        println!("Reading types at index {}: {:?}", types_index, types);

        let mut out_v = Vec::with_capacity(count);

        for i in 0..count {
            match &self.types_table[types_index][i] {
                Type::Utf8String => {
                    let str = &read_string(&self.data[self.position..])?;
                    self.position += str.bytes_consumed;
                    println!("Found string: {:?}", str.value);
                    out_v.push(OutputData::String(str.value));
                }
                Type::EmbeddedData => {
                    return match self.read_embedded_type()? {
                        Some(idx) => {
                            println!("Found embedded data at index: {:?}", idx);
                            self.position += 1; // Advance past the START byte
                            return self.read_types(idx);
                        }
                        None => {
                            println!("No embedded data found");
                            Ok(None)
                        }
                    };
                }
                Type::Object => {
                    let obj_idx = self.read_object()?;
                    println!(
                        "Found obj at {obj_idx:?}: {:?}\n{:?}\n{}",
                        self.object_table.get(obj_idx),
                        self.types_table,
                        self.object_table
                            .iter()
                            .enumerate()
                            .map(|(idx, item)| format!("  {idx}: {item:?}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                    self.position += 1; // Advance past the END byte
                    out_v.push(OutputData::Object(obj_idx));
                }
                Type::SignedInt => {
                    let signed_int = read_signed_int(&self.data[self.position..])?;
                    self.position += signed_int.bytes_consumed;
                    println!("Found signed int: {:?}", signed_int.value);
                    out_v.push(OutputData::SignedInteger(signed_int.value as i64));
                }
                Type::UnsignedInt => {
                    let unsigned_int = read_unsigned_int(&self.data[self.position..])?;
                    self.position += unsigned_int.bytes_consumed;
                    println!("Found unsigned int: {:?}", unsigned_int.value);
                    out_v.push(OutputData::UnsignedInteger(unsigned_int.value));
                }
                Type::Float => {
                    let float = read_float(&self.data[self.position..])?;
                    self.position += float.bytes_consumed;
                    println!("Found float: {:?}", float.value);
                    out_v.push(OutputData::Float(float.value as f32));
                }
                Type::Double => {
                    let double = read_double(&self.data[self.position..])?;
                    self.position += double.bytes_consumed;
                    println!("Found double: {:?}", double.value);
                    out_v.push(OutputData::Double(double.value as f64));
                }
                Type::String(s) => {
                    println!("Found string: {:?}", s);
                    // This means we should look up the associated index in the object table
                    println!(
                        "object at position: 0x{:x}: {:?}",
                        i,
                        self.object_table.get(types_index)
                    );
                    out_v.push(OutputData::Object(types_index));
                }
                Type::Array(length) => {
                    let array_length = *length;
                    let array_data = read_exact_bytes(&self.data[self.position..], array_length)?;
                    self.position += array_length;
                    println!("Found array of length {}: {:?}", array_length, array_data);
                    out_v.push(OutputData::Array(array_data));
                }
                Type::Unknown(_) => todo!(),
            }
        }
        println!("Finished reading types");

        Ok(Some(out_v))
    }

    /// Gets the current type from the stream, either by reading it from the stream or reading it from
    /// the specified index of [`Self::types_table`]. Returns an index into the types table
    /// to avoid cloning large type vectors.
    fn read_type(&mut self, is_embedded_type: bool) -> Result<Option<usize>> {
        let byte = *self.consume_current_byte()?;
        println!("Parsing byte: {:x} at {:x}", byte, self.position - 1);

        match byte {
            START => {
                println!("Start type at position {:x}", self.position);
                // Get the type of the object
                let new_types = Type::read_new_type(&self.data[self.position..])?;
                let new_type_index = self.types_table.len();
                println!(
                    "Parsed type of length {:?}: {:?}",
                    new_types.value.len(),
                    new_types.value
                );

                // Embedded data is stored as a C String in the objects table
                if is_embedded_type {
                    self.object_table.push(Archivable::Type(new_type_index));
                    // We only want to include the first embedded reference tag, not subsequent references to the same embed
                    self.seen_embedded_types
                        .insert(self.object_table.len().saturating_sub(1));
                }

                self.types_table.push(new_types.value);
                self.position += new_types.bytes_consumed;
                Ok(Some(self.types_table.len() - 1))
            }
            EMPTY => {
                println!("Empty type at position {:x}", self.position - 1);
                Ok(None)
            }
            END => {
                println!("End type at position {:x}", self.position - 1);
                Ok(None)
            }
            ptr => {
                println!("Pointer type at position {:x}", self.position - 1);
                let pointer = read_pointer(&ptr)?;
                let ref_tag = pointer.value as usize;

                if ref_tag as usize >= self.types_table.len() {
                    return Ok(None);
                }

                if is_embedded_type {
                    // We only want to include the first embedded reference tag, not subsequent references to the same embed
                    if !self.seen_embedded_types.contains(&ref_tag)
                        && self.types_table.get(ref_tag as usize).is_some()
                    {
                        self.object_table.push(Archivable::Type(ref_tag));
                        self.seen_embedded_types.insert(ref_tag);
                    }
                }

                Ok(Some(ref_tag as usize))
            }
        }
    }
}
