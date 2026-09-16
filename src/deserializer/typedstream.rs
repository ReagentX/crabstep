/*!
 Logic used to deserialize data from a `typedstream`.

 A writeup about the reverse engineering of `typedstream` can be found [here](https://chrissardegna.com/blog/reverse-engineering-apples-typedstream-format/).
*/

use alloc::vec::Vec;

use crate::{
    deserializer::{
        constants::{EMPTY, END, START},
        header::validate_header,
        iter::{Property, PropertyIterator, object_property},
        number::{read_double, read_float, read_signed_int, read_unsigned_int},
        read::{read_byte_at, read_exact_bytes, read_pointer},
        string::read_string,
    },
    error::{Result, TypedStreamError},
    models::{
        archived::{Archived, DataGroup, ObjectData},
        class::Class,
        output_data::OutputData,
        types::{Type, TypeEntry},
    },
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
    pub type_table: Vec<TypeEntry<'a>>,
    /// As we parse the `typedstream`, build a table of seen [`Archived`] data to reference in the future
    pub object_table: Vec<Archived<'a>>,
    /// We want to copy embedded types the first time they are seen, even if the types were resolved through references
    pub(crate) seen_embedded_types: Vec<usize>,
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
        // Table capacities are reserved in `oxidize`, once the header has
        // validated. Constructing a deserializer over a non-`typedstream`
        // buffer therefore allocates nothing.
        Self {
            data,
            position: 0,
            type_table: Vec::new(),
            object_table: Vec::new(),
            seen_embedded_types: Vec::new(),
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

    /// Parse the `typedstream`, consuming header and objects, returning the index of the top-level archived object.
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
        let validation = validate_header(self.data)?;

        // Reserve table capacity now that the input is known to be a valid
        // `typedstream`, so malformed/non-`typedstream` buffers that fail the
        // header check never trigger a large reservation. The divisors reflect
        // the measured worst-case density (~1 object / 16 bytes on
        // distinct-object-heavy streams); the object table is the only one that
        // grows large, so the others stay tight.
        let estimated_size = self.data.len();
        self.type_table
            .reserve((estimated_size / 64).clamp(16, 256));
        self.object_table
            .reserve((estimated_size / 16).clamp(32, 8192));
        self.seen_embedded_types
            .reserve((estimated_size / 128).clamp(8, 64));

        // Advance by the number of bytes consumed by the header validation
        self.position += validation.bytes_consumed;

        // The root must be an object: a stream with no root descriptor, or one
        // whose first value is not an object reference, has nothing to walk.
        let Some(type_index) = self.read_type(false)? else {
            return Err(TypedStreamError::InvalidObject);
        };
        match self.read_types(type_index)?.as_slice().first() {
            Some(OutputData::Object(idx)) => Ok(*idx),
            _ => Err(TypedStreamError::InvalidObject),
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
    /// Returns [`TypedStreamError::OutOfBounds`] if the index is past the object
    /// table, or [`TypedStreamError::InvalidObject`] if it is not an object.
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
        if root_object_index >= self.object_table.len() {
            return Err(TypedStreamError::OutOfBounds(
                root_object_index,
                self.object_table.len(),
            ));
        }
        PropertyIterator::new(&self.object_table, &self.type_table, root_object_index)
            .ok_or(TypedStreamError::InvalidObject)
    }

    /// Resolve the object at `object_index` into a group-level [`Property`].
    ///
    /// Unlike [`resolve_properties`](Self::resolve_properties) (which iterates an
    /// object's *contents*), this returns the object itself, in the form the
    /// `foundation` accessors expect. Use it when the value you want is an object
    /// (for example the stream's root): `ts.resolve_object(root)?.as_dictionary()`.
    ///
    /// # Errors
    ///
    /// Returns [`TypedStreamError::OutOfBounds`] if the index is past the object
    /// table, or [`TypedStreamError::InvalidObject`] if it is not an object.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let mut ts = TypedStreamDeserializer::new(&[]);
    /// let root = ts.oxidize().unwrap();
    ///
    /// // The object as a `Property`, ready for the `foundation` accessors.
    /// let object = ts.resolve_object(root).unwrap();
    /// ```
    pub fn resolve_object(&self, object_index: usize) -> Result<Property<'_, '_>> {
        if object_index >= self.object_table.len() {
            return Err(TypedStreamError::OutOfBounds(
                object_index,
                self.object_table.len(),
            ));
        }
        object_property(&self.object_table, &self.type_table, object_index)
            .ok_or(TypedStreamError::InvalidObject)
    }

    /// Oxidize the stream and resolve its root object into a [`Property`].
    ///
    /// The object-level counterpart of [`iter_root`](Self::iter_root): use it when
    /// the stream's root *is* the value you want (e.g. a root `NSDictionary`),
    /// rather than a container whose contents you iterate.
    ///
    /// # Errors
    ///
    /// Returns a [`TypedStreamError`] if parsing fails or the root is invalid.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let mut ts = TypedStreamDeserializer::new(&[]);
    ///
    /// // Resolve the root object directly, without a separate `oxidize` call.
    /// let root = ts.root().unwrap();
    /// ```
    pub fn root(&mut self) -> Result<Property<'_, '_>> {
        let root = self.oxidize()?;
        self.resolve_object(root)
    }

    /// Reads the next byte from the stream, advancing the position.
    #[inline(always)]
    fn consume_current_byte(&mut self) -> Result<&u8> {
        let byte = read_byte_at(self.data, self.position)?;
        self.position += 1;
        Ok(byte)
    }

    /// Reads an unsigned integer from the stream, advancing the position.
    #[inline(always)]
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
                    .push(TypeEntry::One(Type::new_string(string_data.value)));
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
        // Index of the first START we encounter (the bottom-most child)
        let mut first_new: Option<usize> = None;
        // Index of the most recently pushed class (current “child”)
        let mut prev_new: Option<usize> = None;
        // Parent for the outer-most new class (set by EMPTY or a pointer)
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
                    if let Some(child_idx) = prev_new
                        && let Archived::Class(ref mut child_cls) = self.object_table[child_idx]
                    {
                        child_cls.parent_index = Some(idx);
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
        let Some(first_idx) = first_new else {
            return Ok(final_parent);
        };

        // Patch the outer-most newly created class so that it points to the
        // already-existing parent (or to `None` if EMPTY terminated the list).
        if let Some(outer_idx) = prev_new
            && let Archived::Class(ref mut outer_cls) = self.object_table[outer_idx]
        {
            outer_cls.parent_index = final_parent;
        }

        // Return the index of the bottom-most child we created first.
        Ok(Some(first_idx))
    }

    fn read_object(&mut self) -> Result<Option<usize>> {
        match *read_byte_at(self.data, self.position)? {
            START => {
                let placeholder_index = self.object_table.len();
                // This placeholder will be replaced with the actual object data once we read the class
                self.object_table.push(Archived::Placeholder);
                // Advance the position to the next byte, which should be the start of a class
                self.position += 1;

                if let Some(cls) = self.read_class()? {
                    // Collect the object's groups locally. The overwhelming
                    // majority of objects hold a single single-value group (an
                    // NSString's text, an NSNumber's value, a reference to
                    // another object), which `ObjectData` stores inline with no
                    // heap allocation at all.
                    let mut data = ObjectData::Empty;
                    while self.position < self.data.len()
                        && *read_byte_at(self.data, self.position)? != END
                    {
                        // Read the next type, which should be an object
                        if let Some(next_index) = self.read_type(false)? {
                            // Recursively read the types for this object
                            data.push(self.read_types(next_index)?);
                        }
                    }
                    self.object_table[placeholder_index] = Archived::Object { class: cls, data };
                }
                Ok(Some(placeholder_index))
            }
            EMPTY => {
                self.position += 1;
                Ok(None)
            }
            ptr => {
                let pointer = read_pointer(&ptr)?;
                Ok(Some(pointer.value as usize))
            }
        }
    }

    /// Reads numeric types (signed, unsigned, float, double) and returns the corresponding `OutputData`
    fn read_number(&mut self, ty: Type<'a>) -> Result<OutputData<'a>> {
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

    /// Decodes a single, already-resolved non-embedded type descriptor into one
    /// [`OutputData`] value.
    ///
    /// [`Type::EmbeddedData`] is handled by the caller ([`Self::read_types`])
    /// because it redirects to another type entry rather than producing a value.
    #[inline]
    fn read_value(&mut self, ty: Type<'a>) -> Result<OutputData<'a>> {
        match ty {
            Type::Utf8String => {
                let str_data = read_string(&self.data[self.position..])?;
                self.position += str_data.bytes_consumed;
                Ok(OutputData::String(str_data.value))
            }
            Type::Object => {
                let obj_idx = self.read_object()?;
                self.position += 1;
                Ok(match obj_idx {
                    Some(idx) => OutputData::Object(idx),
                    None => OutputData::Null,
                })
            }
            Type::String(s) => Ok(OutputData::String(s)),
            Type::Array(length) => {
                let array_data = read_exact_bytes(&self.data[self.position..], length)?;
                self.position += length;
                Ok(OutputData::Array(array_data))
            }
            Type::Unknown(byte) => Ok(OutputData::Byte(byte)),
            // Handle all numeric types
            Type::SignedInt | Type::UnsignedInt | Type::Float | Type::Double => {
                self.read_number(ty)
            }
            // `EmbeddedData` is intercepted by `read_types` before reaching here.
            Type::EmbeddedData => Err(TypedStreamError::InvalidObject),
        }
    }

    /// Reads an `EmbeddedData` descriptor, redirecting to the embedded type
    /// entry. Returns the group decoded from that entry.
    ///
    /// A nil embedded type is a `NULL` pointer written into the slot, the same
    /// thing a nil object reference is, so it decodes to
    /// [`OutputData::Null`] and keeps its position in the object's groups.
    fn read_embedded(&mut self) -> Result<DataGroup<'a>> {
        if let Some(idx) = self.read_embedded_type()? {
            self.position += 1;
            self.read_types(idx)
        } else {
            Ok(DataGroup::One(OutputData::Null))
        }
    }

    /// Reads all type descriptors at `types_index` into a single data group.
    fn read_types(&mut self, types_index: usize) -> Result<DataGroup<'a>> {
        let len = self.type_table[types_index].len();

        // Common case: a single descriptor decodes to a single value with no Vec.
        if len == 1 {
            let ty = self.type_table[types_index][0];
            return if matches!(ty, Type::EmbeddedData) {
                self.read_embedded()
            } else {
                Ok(DataGroup::One(self.read_value(ty)?))
            };
        }

        let mut out_v = Vec::with_capacity(len);
        for i in 0..len {
            let ty = self.type_table[types_index][i];
            if matches!(ty, Type::EmbeddedData) {
                // Read the embedded group and merge its contents into the current output vector.
                match self.read_embedded()? {
                    DataGroup::One(value) => out_v.push(value),
                    DataGroup::Values(values) => out_v.extend(values),
                }
                continue;
            }
            out_v.push(self.read_value(ty)?);
        }

        // An embed with no types can leave a multi-slot descriptor holding one
        // value; keep `One` exact.
        Ok(if out_v.len() == 1 {
            DataGroup::One(out_v.pop().unwrap())
        } else {
            DataGroup::Values(out_v)
        })
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
                // Embedded data is stored as a Type in the objects table
                if is_embedded_type {
                    self.object_table.push(Archived::Type(new_type_index));
                    // We only want to include the first embedded reference tag, not subsequent references to the same embed
                    self.seen_embedded_types
                        .push(self.object_table.len().saturating_sub(1));
                }

                self.type_table.push(new_types.value);
                self.position += new_types.bytes_consumed;
                Ok(Some(self.type_table.len() - 1))
            }
            END | EMPTY => Ok(None),
            ptr => {
                let pointer = read_pointer(&ptr)?;
                let ref_tag = pointer.value as usize;

                // Optimize bounds checking
                if ref_tag >= self.type_table.len() {
                    return Ok(None);
                }

                if is_embedded_type {
                    // We only want to include the first embedded reference tag, not subsequent references to the same embed
                    if !self.seen_embedded_types.contains(&ref_tag) {
                        self.object_table.push(Archived::Type(ref_tag));
                        self.seen_embedded_types.push(ref_tag);
                    }
                }

                Ok(Some(ref_tag))
            }
        }
    }
}

#[cfg(test)]
mod group_tests {
    use alloc::{vec, vec::Vec};

    use super::TypedStreamDeserializer;
    use crate::{
        DataGroup, ObjectData, OutputData, Property,
        deserializer::constants::{EMPTY, END, START},
        models::archived::Archived,
    };

    fn stream(groups: &[u8]) -> Vec<u8> {
        let mut bytes = vec![
            4, 11, b's', b't', b'r', b'e', b'a', b'm', b't', b'y', b'p', b'e', b'd', 0x81, 0xe8, 3,
            START, 1, b'@', START, START, START, 1, b'X', 0, EMPTY,
        ];
        bytes.extend_from_slice(groups);
        bytes.push(END);
        bytes
    }

    fn unsigned(property: Property<'_, '_>) -> u64 {
        let Property::Primitive(value) = property else {
            panic!("expected a primitive");
        };
        value.as_u64().unwrap()
    }

    #[test]
    fn preserves_empty_groups_null_slots_and_item_order() {
        let bytes = stream(&[
            // A zero-length descriptor is a group with no values; it keeps its position.
            START, 1, b'C', 1, START, 0, START, 2, b'C', b'C', 2, 3, START, 1, b'C', 4,
            // A nil `EmbeddedData` is a slot written as `NULL`: one group holding `Null`.
            START, 1, b'*', EMPTY,
        ]);
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let Archived::Object { data, .. } = &ts.object_table[root] else {
            panic!("expected an object");
        };
        assert_eq!(data.group_count(), 5);
        assert_eq!(
            data,
            &ObjectData::Groups(vec![
                DataGroup::One(OutputData::UnsignedInteger(1)),
                DataGroup::Values(vec![]),
                DataGroup::Values(vec![
                    OutputData::UnsignedInteger(2),
                    OutputData::UnsignedInteger(3),
                ]),
                DataGroup::One(OutputData::UnsignedInteger(4)),
                DataGroup::One(OutputData::Null),
            ])
        );

        let mut properties = ts.resolve_properties(root).unwrap();
        let mut lengths = Vec::new();
        let mut values = Vec::new();
        for property in properties.by_ref().take(4) {
            let Property::Group(group) = property else {
                panic!("expected a group");
            };
            lengths.push(group.len());
            let forward: Vec<_> = group.iter().map(unsigned).collect();
            let reverse: Vec<_> = group.iter().rev().map(unsigned).collect();
            assert_eq!(reverse, forward.iter().rev().copied().collect::<Vec<_>>());
            values.extend_from_slice(&forward);

            let mut iter = group.iter();
            assert_eq!(iter.len(), forward.len());
            if !forward.is_empty() {
                assert_eq!(unsigned(iter.next().unwrap()), forward[0]);
                assert_eq!(iter.len(), forward.len() - 1);
            }
            if forward.len() == 2 {
                assert_eq!(unsigned(iter.next_back().unwrap()), forward[1]);
            }
            assert_eq!(iter.len(), 0);
            assert!(iter.next().is_none());
            assert!(iter.next_back().is_none());
        }
        assert_eq!(lengths, [1, 0, 2, 1]);
        assert_eq!(values, [1, 2, 3, 4]);

        let Some(Property::Group(group)) = properties.next() else {
            panic!("expected the nil embed's group");
        };
        assert_eq!(group.len(), 1);
        assert!(matches!(
            group.iter().next(),
            Some(Property::Primitive(OutputData::Null))
        ));
        assert!(properties.next().is_none());
    }

    #[test]
    fn preserves_groups_when_promoting_inline_data() {
        for (descriptors, expected) in [
            (
                vec![START, 1, b'C', 1, START, 1, b'C', 2],
                vec![vec![1], vec![2]],
            ),
            (
                vec![START, 1, b'C', 1, START, 2, b'C', b'C', 2, 3],
                vec![vec![1], vec![2, 3]],
            ),
            (
                vec![START, 2, b'C', b'C', 1, 2, START, 1, b'C', 3],
                vec![vec![1, 2], vec![3]],
            ),
        ] {
            let bytes = stream(&descriptors);
            let mut ts = TypedStreamDeserializer::new(&bytes);
            let root = ts.oxidize().unwrap();
            let Archived::Object {
                data: ObjectData::Groups(groups),
                ..
            } = &ts.object_table[root]
            else {
                panic!("expected grouped data");
            };
            let values: Vec<Vec<_>> = groups
                .iter()
                .map(|group| {
                    group
                        .as_slice()
                        .iter()
                        .map(|value| value.as_u64().unwrap())
                        .collect()
                })
                .collect();
            assert_eq!(values, expected);
            assert!(groups.iter().all(|group| !group.is_empty()));
            for group in groups {
                assert_eq!(matches!(group, DataGroup::One(_)), group.len() == 1);
            }
        }
    }

    #[test]
    fn splices_embedded_values_into_multi_slot_groups() {
        // A descriptor with several slots fills one group; an `EmbeddedData`
        // slot contributes the embedded values in place.
        use OutputData::UnsignedInteger as U;
        for (descriptors, expected) in [
            // Embed last: the values decoded before it must survive.
            (
                vec![START, 2, b'C', b'*', 1, START, START, 1, b'C', 0x94, 2],
                vec![U(1), U(2)],
            ),
            // Embed first: the slots after it must still be read.
            (
                vec![START, 2, b'*', b'C', START, START, 1, b'C', 0x94, 1, 2],
                vec![U(1), U(2)],
            ),
            // Embed in the middle.
            (
                vec![
                    START, 3, b'C', b'*', b'C', 1, START, START, 1, b'C', 0x94, 2, 3,
                ],
                vec![U(1), U(2), U(3)],
            ),
            // A multi-value embedded type flattens into the enclosing group.
            (
                vec![
                    START, 2, b'C', b'*', 1, START, START, 2, b'C', b'C', 0x94, 2, 3,
                ],
                vec![U(1), U(2), U(3)],
            ),
            // A nil embed is a `NULL` slot; the slot after it is still read.
            (
                vec![START, 3, b'C', b'*', b'C', 1, EMPTY, 3],
                vec![U(1), OutputData::Null, U(3)],
            ),
            // An embedded type with no types contributes nothing, which can
            // leave a multi-slot descriptor with one value.
            (
                vec![START, 2, b'C', b'*', 1, START, START, 0, 0x94],
                vec![U(1)],
            ),
        ] {
            let bytes = stream(&descriptors);
            let mut ts = TypedStreamDeserializer::new(&bytes);
            let root = ts.oxidize().unwrap();
            let Archived::Object { data, .. } = &ts.object_table[root] else {
                panic!("expected an object");
            };
            assert_eq!(data.group_count(), 1, "{descriptors:?}");
            let values = match data {
                ObjectData::Inline(value) => core::slice::from_ref(value),
                ObjectData::Groups(groups) => groups[0].as_slice(),
                ObjectData::Empty => panic!("expected data for {descriptors:?}"),
            };
            assert_eq!(values, expected.as_slice(), "{descriptors:?}");
            // A single value never lands in `Values`, whichever path produced it.
            assert_eq!(
                matches!(data, ObjectData::Inline(_)),
                expected.len() == 1,
                "{descriptors:?}"
            );
        }
    }

    #[test]
    fn preserves_shared_and_self_references() {
        let bytes = stream(&[
            // The child reuses class 1 and occupies object slot 2.
            START, 1, b'@', START, 0x93, START, 1, b'C', 7, END, START, 1, b'@', 0x94, START, 2,
            b'@', b'@', 0x92, 0x94,
        ]);
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        assert_eq!(root, 0);
        assert_eq!(ts.object_table.len(), 3);
        assert_eq!(
            ts.object_table[root],
            Archived::Object {
                class: 1,
                data: ObjectData::Groups(vec![
                    DataGroup::One(OutputData::Object(2)),
                    DataGroup::One(OutputData::Object(2)),
                    DataGroup::Values(vec![OutputData::Object(0), OutputData::Object(2)]),
                ]),
            }
        );
        assert_eq!(
            ts.object_table[2],
            Archived::Object {
                class: 1,
                data: ObjectData::Inline(OutputData::UnsignedInteger(7)),
            }
        );
        let primitives = ts
            .resolve_properties(root)
            .unwrap()
            .primitives_with_limits(10, 100);
        assert!(!primitives.is_empty());
        assert!(primitives.len() <= 100);
        assert!(primitives.iter().all(|value| value.as_u64() == Some(7)));
    }
}
