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
    /// The shared-string table, with descriptor literals stored as parsed types.
    ///
    /// Shared-string layout: literal once, index thereafter. Entry kinds:
    /// type descriptors, class names, selectors, and `char *` values. Register
    /// names and values as [`Type::String`]; parse an entry in place before
    /// using it as a descriptor. Keep every entry's original text available
    /// through [`shared_string`](Self::shared_string).
    pub type_table: Vec<TypeEntry<'a>>,
    /// Text for each [`type_table`](Self::type_table) entry, by index. Keep both
    /// vectors aligned by routing every registration through
    /// [`register_string`](Self::register_string).
    pub(crate) strings: Vec<&'a str>,
    /// As we parse the `typedstream`, build a table of seen [`Archived`] data to reference in the future
    pub object_table: Vec<Archived<'a>>,
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
            strings: Vec::new(),
            object_table: Vec::new(),
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
        let strings = (estimated_size / 64).clamp(16, 256);
        self.type_table.reserve(strings);
        self.strings.reserve(strings);
        self.object_table
            .reserve((estimated_size / 16).clamp(32, 8192));

        // Advance by the number of bytes consumed by the header validation
        self.position += validation.bytes_consumed;

        // The root must be an object: a stream with no root descriptor, or one
        // whose first value is not an object reference, has nothing to walk.
        let Some(type_index) = self.read_type()? else {
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

    /// Reads a shared string: a literal, registered as a new entry, or a
    /// reference to an existing one. Returns the entry's index.
    fn read_string(&mut self) -> Result<usize> {
        match *self.consume_current_byte()? {
            START => {
                let string_data = read_string(&self.data[self.position..])?;
                self.position += string_data.bytes_consumed;
                Ok(self.register_string(string_data.value, None))
            }
            EMPTY => Err(TypedStreamError::EmptyString),
            ptr => {
                let index = read_pointer(&ptr)?.value as usize;
                if index < self.strings.len() {
                    Ok(index)
                } else {
                    Err(TypedStreamError::InvalidPointer(index as u8))
                }
            }
        }
    }

    /// Returns the text of shared-string entry `index`: a type descriptor, class
    /// name, selector, or `char *` value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let mut ts = TypedStreamDeserializer::new(&[]);
    /// ts.oxidize().unwrap();
    /// assert_eq!(ts.shared_string(0), Some("@"));
    /// ```
    #[must_use]
    pub fn shared_string(&self, index: usize) -> Option<&'a str> {
        debug_assert_eq!(self.strings.len(), self.type_table.len());
        self.strings.get(index).copied()
    }

    /// Appends a shared string to both tables and returns its index. Route every
    /// registration through this function so the tables remain aligned.
    ///
    /// `parsed`: the descriptor view for a descriptor literal. For a name or
    /// value, omit parsed types; parse the entry only for descriptor use.
    fn register_string(&mut self, text: &'a str, parsed: Option<TypeEntry<'a>>) -> usize {
        self.type_table
            .push(parsed.unwrap_or(TypeEntry::One(Type::new_string(text))));
        self.strings.push(text);
        debug_assert_eq!(self.strings.len(), self.type_table.len());
        self.strings.len() - 1
    }

    /// Reads a class from the stream, handling nested class definitions.
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

    /// Reads an object from the stream, handling its class and associated data.
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
                        if let Some(next_index) = self.read_type()? {
                            // Recursively read the types for this object
                            data.push(self.read_types(next_index)?);
                        }
                    }
                    self.object_table[placeholder_index] = Archived::Object { class: cls, data };
                }
                Ok(Some(placeholder_index))
            }
            // A nil reference and a pointer are both one byte; like the `END`
            // of an inline object, that byte is left for the caller to consume.
            EMPTY => Ok(None),
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

    /// Decodes one slot of a descriptor into one [`OutputData`] value.
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
            // Parse every string-table entry in `read_type` before passing it to `read_types`
            Type::String(_) => Err(TypedStreamError::InvalidObject),
            Type::Array(length) => {
                let array_data = read_exact_bytes(&self.data[self.position..], length)?;
                self.position += length;
                Ok(OutputData::Array(array_data))
            }
            // Selector encoding: one literal, then shared-string references.
            Type::Selector => {
                if *read_byte_at(self.data, self.position)? == EMPTY {
                    self.position += 1;
                    return Ok(OutputData::Null);
                }
                let index = self.read_string()?;
                Ok(OutputData::String(self.strings[index]))
            }
            // Track C strings by pointer identity through the object table:
            // allocate a slot on `START`, store the shared-string index, and
            // reuse an earlier slot for a pointer.
            Type::CString => match *self.consume_current_byte()? {
                EMPTY => Ok(OutputData::Null),
                START => {
                    let index = self.read_string()?;
                    self.object_table.push(Archived::CString(index));
                    Ok(OutputData::String(self.strings[index]))
                }
                ptr => {
                    let slot = read_pointer(&ptr)?.value as usize;
                    match self.object_table.get(slot) {
                        Some(Archived::CString(index)) => {
                            Ok(OutputData::String(self.strings[*index]))
                        }
                        _ => Err(TypedStreamError::InvalidPointer(slot as u8)),
                    }
                }
            },
            // A class chain, encoded exactly like an object's class header;
            // `idx` is the `Archived::Class` entry, not an object.
            Type::Class => Ok(match self.read_class()? {
                Some(idx) => OutputData::Object(idx),
                None => OutputData::Null,
            }),
            // Handle all numeric types
            Type::SignedInt | Type::UnsignedInt | Type::Float | Type::Double => {
                self.read_number(ty)
            }
        }
    }

    /// Reads one value per slot of the descriptor at `types_index` into a
    /// single data group.
    fn read_types(&mut self, types_index: usize) -> Result<DataGroup<'a>> {
        let len = self.type_table[types_index].len();

        // Common case: a single descriptor decodes to a single value with no Vec.
        if len == 1 {
            let ty = self.type_table[types_index][0];
            return Ok(DataGroup::One(self.read_value(ty)?));
        }

        let mut out_v = Vec::with_capacity(len);
        for i in 0..len {
            let ty = self.type_table[types_index][i];
            out_v.push(self.read_value(ty)?);
        }
        Ok(DataGroup::Values(out_v))
    }

    /// Reads a type descriptor: a literal, registered as a new shared string, or
    /// a reference to an existing one. Returns its index in
    /// [`type_table`](Self::type_table), or `None` at an [`END`]/[`EMPTY`] marker
    /// where a descriptor was optional.
    ///
    /// Descriptor-reference case: a name or `char *` value already in the
    /// referenced entry. For `NSValue`'s `objCType`, parse that entry in place,
    /// then use its text to decode the following value. Keep the text in
    /// [`strings`](Self::strings).
    fn read_type(&mut self) -> Result<Option<usize>> {
        match *self.consume_current_byte()? {
            START => {
                let literal = Type::read_new_type(&self.data[self.position..])?;
                self.position += literal.bytes_consumed;
                let (text, entry) = literal.value;
                Ok(Some(self.register_string(text, Some(entry))))
            }
            END | EMPTY => Ok(None),
            ptr => {
                let index = read_pointer(&ptr)?.value as usize;
                if index >= self.type_table.len() {
                    return Err(TypedStreamError::InvalidPointer(index as u8));
                }
                if let Some(Type::String(text)) = self.type_table[index].first() {
                    let remaining = self.data.len() - self.position;
                    self.type_table[index] = Type::parse_descriptor(text, remaining)?;
                }
                Ok(Some(index))
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
        models::{
            archived::Archived,
            types::{Type, TypeEntry},
        },
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
            // A nil `char *` value: one `NULL` byte and one group holding `Null`.
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
    fn nil_object_reference_consumes_one_byte() {
        // `@` + EMPTY is a one-byte nil. The slot after it must still line up:
        // consuming a second byte would swallow the `7`.
        let bytes = stream(&[START, 2, b'@', b'C', EMPTY, 7]);
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let Archived::Object { data, .. } = &ts.object_table[root] else {
            panic!("expected an object");
        };
        assert_eq!(
            data,
            &ObjectData::Groups(vec![DataGroup::Values(vec![
                OutputData::Null,
                OutputData::UnsignedInteger(7),
            ])])
        );
        assert_eq!(ts.position, bytes.len());
    }

    #[test]
    fn selectors_are_shared_strings() {
        // Descriptor `::` is string-table entry 2, the literal `quit:` entry 3,
        // so the second selector references it with tag 0x92 + 3.
        let bytes = stream(&[
            START, 2, b':', b':', START, 5, b'q', b'u', b'i', b't', b':', 0x95, START, 1, b':',
            EMPTY,
        ]);
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let Archived::Object { data, .. } = &ts.object_table[root] else {
            panic!("expected an object");
        };
        assert_eq!(
            data,
            &ObjectData::Groups(vec![
                DataGroup::Values(vec![
                    OutputData::String("quit:"),
                    OutputData::String("quit:"),
                ]),
                DataGroup::One(OutputData::Null),
            ])
        );
        assert_eq!(ts.position, bytes.len());
    }

    #[test]
    fn class_references_read_a_class_chain() {
        // `#` then a class header: name, version, superclass terminator.
        let bytes = stream(&[
            START, 2, b'#', b'#', START, START, 3, b'F', b'o', b'o', 2, EMPTY, EMPTY,
        ]);
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let Archived::Object {
            data: ObjectData::Groups(groups),
            ..
        } = &ts.object_table[root]
        else {
            panic!("expected grouped data");
        };
        let [DataGroup::Values(values)] = groups.as_slice() else {
            panic!("expected one two-slot group");
        };
        let [OutputData::Object(class_idx), OutputData::Null] = values.as_slice() else {
            panic!("expected a class reference and a nil class, got {values:?}");
        };
        let Archived::Class(class) = &ts.object_table[*class_idx] else {
            panic!("`#` must reference a class entry");
        };
        assert_eq!(
            ts.type_table[class.name_index].first(),
            Some(&Type::String("Foo"))
        );
        assert_eq!(class.version, 2);
        assert_eq!(class.parent_index, None);
        assert_eq!(ts.position, bytes.len());
    }

    #[test]
    fn aggregates_decode_flat_in_stream() {
        use OutputData::{Array as A, Double as D, SignedInteger as I, UnsignedInteger as U};
        // Integral doubles are written as one-byte ints, as NSArchiver does.
        for (descriptor, values, expected) in [
            (&b"B"[..], &[1u8][..], vec![U(1)]),
            (b"{_NSRange=QQ}", &[3, 4], vec![U(3), U(4)]),
            (b"i{CGSize=dd}", &[7, 1, 2], vec![I(7), D(1.0), D(2.0)]),
            (b"[2i]", &[5, 6], vec![I(5), I(6)]),
            (b"[2[2c]]", b"abcd", vec![A(b"ab"), A(b"cd")]),
            (b"i[2c]i", &[1, b'x', b'y', 2], vec![I(1), A(b"xy"), I(2)]),
        ] {
            let mut group = vec![START, u8::try_from(descriptor.len()).unwrap()];
            group.extend_from_slice(descriptor);
            group.extend_from_slice(values);
            let bytes = stream(&group);
            let mut ts = TypedStreamDeserializer::new(&bytes);
            let root = ts.oxidize().unwrap();
            let Archived::Object { data, .. } = &ts.object_table[root] else {
                panic!("expected an object");
            };
            let got = match data {
                ObjectData::Inline(value) => core::slice::from_ref(value),
                ObjectData::Groups(groups) => groups[0].as_slice(),
                ObjectData::Empty => panic!("no data"),
            };
            assert_eq!(
                got,
                expected.as_slice(),
                "{}",
                core::str::from_utf8(descriptor).unwrap()
            );
            assert_eq!(ts.position, bytes.len());
        }
    }

    #[test]
    fn unencodable_descriptor_is_an_error_not_a_desync() {
        let bytes = stream(&[START, 2, b'^', b'i', 0]);
        let mut ts = TypedStreamDeserializer::new(&bytes);
        assert!(matches!(
            ts.oxidize(),
            Err(crate::error::TypedStreamError::InvalidType(b'^'))
        ));
    }

    #[test]
    fn c_strings_share_by_pointer_through_the_object_table() {
        // `stream()` prefix: object-table slots [0] (root), [1] (class), [2]
        // (first C string, tag `0x94`). Shared-string entries: [0] `@`, [1] `X`,
        // [2] first group descriptor, [3] C-string literal (`0x95`).
        use DataGroup::{One, Values};
        use OutputData::{Null, SignedInteger as I, String as S, UnsignedInteger as U};
        for (groups, expected, c_string_slots) in [
            // Literal C string: allocate a slot, then read the shared string.
            (
                vec![START, 2, b'C', b'*', 1, START, START, 2, b'h', b'i'],
                vec![Values(vec![U(1), S("hi")])],
                1,
            ),
            (
                vec![START, 2, b'*', b'C', START, START, 2, b'h', b'i', 2],
                vec![Values(vec![S("hi"), U(2)])],
                1,
            ),
            // `NULL`: one byte; no object-table slot.
            (
                vec![START, 3, b'C', b'*', b'C', 1, EMPTY, 3],
                vec![Values(vec![U(1), Null, U(3)])],
                0,
            ),
            // Repeated pointer: a bare reference to the existing slot.
            (
                vec![START, 1, b'*', START, START, 2, b'h', b'i', 0x94, 0x94],
                vec![One(S("hi")), One(S("hi"))],
                1,
            ),
            // Distinct pointer, same text: a new object-table slot and the same
            // string-table entry.
            (
                vec![
                    START, 1, b'*', START, START, 2, b'h', b'i', 0x94, START, 0x95,
                ],
                vec![One(S("hi")), One(S("hi"))],
                2,
            ),
            // Existing descriptor text (`i`): reuse its string-table entry for
            // the C-string value.
            (
                vec![START, 1, b'i', 7, START, 1, b'*', START, 0x94],
                vec![One(I(7)), One(S("i"))],
                1,
            ),
        ] {
            let bytes = stream(&groups);
            let mut ts = TypedStreamDeserializer::new(&bytes);
            let root = ts.oxidize().unwrap();
            let Archived::Object {
                data: ObjectData::Groups(got),
                ..
            } = &ts.object_table[root]
            else {
                panic!("expected grouped data for {groups:?}");
            };
            assert_eq!(got, &expected, "{groups:?}");
            assert_eq!(
                ts.object_table
                    .iter()
                    .filter(|o| matches!(o, Archived::CString(_)))
                    .count(),
                c_string_slots,
                "{groups:?}"
            );
            assert_eq!(ts.position, bytes.len(), "{groups:?}");
        }
    }

    #[test]
    fn c_string_text_types_the_value_that_follows() {
        // `NSValue` layout: record `objCType` through `*`, then reuse that
        // string to type the following value.
        let bytes = stream(&[START, 1, b'*', START, START, 1, b'q', 0x95, 5]);
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let Archived::Object { data, .. } = &ts.object_table[root] else {
            panic!("expected an object");
        };
        assert_eq!(
            data,
            &ObjectData::Groups(vec![
                DataGroup::One(OutputData::String("q")),
                DataGroup::One(OutputData::SignedInteger(5)),
            ])
        );
        // Entry [3]: register the value string first; parse it in place when the
        // descriptor references it, while preserving the original text for
        // `shared_string`.
        assert_eq!(ts.shared_string(3), Some("q"));
        assert_eq!(ts.type_table[3], TypeEntry::One(Type::SignedInt));
        assert_eq!(ts.object_table[2], Archived::CString(3));
        assert_eq!(ts.position, bytes.len());
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
