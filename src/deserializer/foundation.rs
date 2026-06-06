/*!
Typed accessors for common Apple [Foundation](https://developer.apple.com/documentation/foundation) classes.

Enabled by the `foundation` cargo feature. These methods interpret the generic
[`Property`] tree as specific Foundation types (`NSString`, `NSDictionary`, …)
so consumers do not have to re-implement class-name matching (and re-discover
its footguns, e.g. that the data cluster archives as both `NSData` and
`NSMutableData`).

This feature is purely for convenience: the parser and the [`Property`]/
[`OutputData`] model are unchanged
whether or not it is enabled, and any class not modeled here stays reachable
through [`Property::Object`], so nothing is ever lost.

Accessors are called on a group-level [`Property`]: the value yielded while
iterating an object's properties.
*/

use crate::{
    deserializer::iter::{Property, PropertyIterator},
    models::output_data::OutputData,
};

// MARK: Names
// Cluster class-name sets. Foundation archives several types as both an immutable
// and a mutable variant (and the data cluster archives as both `NSData` and
// `NSMutableData`); matching all variants in one place keeps the footgun
// centralized.
/// Denotes string data
pub(crate) const STRING_CLASSES: &[&str] = &["NSString", "NSMutableString"];
/// Denotes string data with attributes (e.g. font, color, …)
pub(crate) const ATTRIBUTED_STRING_CLASSES: &[&str] =
    &["NSAttributedString", "NSMutableAttributedString"];
/// Denotes raw bytes
pub(crate) const DATA_CLASSES: &[&str] = &["NSData", "NSMutableData"];
/// Denotes ordered collections of arbitrary objects.
pub(crate) const ARRAY_CLASSES: &[&str] = &["NSArray", "NSMutableArray"];
/// Denotes unordered collections of arbitrary objects.
pub(crate) const DICT_CLASSES: &[&str] = &["NSDictionary", "NSMutableDictionary"];
/// Denotes unordered collections of unique arbitrary objects.
pub(crate) const SET_CLASSES: &[&str] = &["NSSet", "NSMutableSet"];

/// Extract the backing UTF-8 of a string-cluster object: the first `String`
/// primitive in the object's first data group.
fn backing_string<'a, 'b: 'a>(mut data: PropertyIterator<'a, 'b>) -> Option<&'a str> {
    if let Property::Group(group) = data.next()?
        && let Some(Property::Primitive(OutputData::String(s))) = group.first()
    {
        return Some(s);
    }
    None
}

// MARK: Property
/// Typed accessors for Foundation classes, implemented as methods on [`Property`] so
/// they are available while iterating any object's properties. Each accessor resolves
/// whether `self` is a [`Property::Group`] whose first element is an object of the
/// expected class(es), or (for scalar types) a group wrapping a bare primitive or
/// an `NSNumber` object wrapping one, and returns the interpreted value if successful.
impl<'a, 'b: 'a> Property<'a, 'b> {
    /// The Objective-C class name of the object this property refers to, if any.
    ///
    /// Resolves whether `self` is a [`Property::Object`] directly or a
    /// [`Property::Group`] whose first element is an object. This is the escape
    /// hatch for classes the `foundation` feature does not model: the class name
    /// plus the raw subtree (via [`Property::Object`]) are always available, so a
    /// consumer can handle the long tail of app-specific classes itself.
    #[must_use]
    pub fn class_name(&self) -> Option<&'a str> {
        match self {
            Property::Object { name, .. } => Some(*name),
            Property::Group(group) => match group.first()? {
                Property::Object { name, .. } => Some(name),
                _ => None,
            },
            Property::Primitive(_) => None,
        }
    }

    // MARK: String
    /// The backing string of an `NSString` / `NSMutableString`, or the plain text
    /// of an `NSAttributedString` / `NSMutableAttributedString` (its attributes
    /// remain reachable through the generic [`Property`] tree).
    #[must_use]
    pub fn as_string(&self) -> Option<&'a str> {
        if let Some(data) = self.object_in_classes(STRING_CLASSES) {
            return backing_string(data);
        }
        // An attributed string stores its backing store as a nested
        // `NSString`/`NSMutableString`; its position among the groups varies by
        // producer (the attributes dictionary often comes first), so scan.
        if let Some(data) = self.object_in_classes(ATTRIBUTED_STRING_CLASSES) {
            for prop in data {
                if let Property::Group(group) = prop
                    && let Some(Property::Object {
                        name, data: inner, ..
                    }) = group.first()
                    && STRING_CLASSES.contains(&name)
                {
                    return backing_string(inner);
                }
            }
        }
        None
    }

    // MARK: Bytes
    /// The raw bytes of an `NSData` / `NSMutableData`.
    ///
    /// crabstep does not interpret the bytes — they may be a `bplist00`, a
    /// compressed blob, an image, etc. The caller decides what they are.
    #[must_use]
    pub fn as_data(&self) -> Option<&'a [u8]> {
        let data = self.object_in_classes(DATA_CLASSES)?;
        for prop in data {
            if let Property::Group(group) = prop {
                for child in group {
                    if let Property::Primitive(OutputData::Array(bytes)) = child {
                        return Some(bytes);
                    }
                }
            }
        }
        None
    }

    // MARK: Boolean
    /// An `NSNumber` (or bare primitive) interpreted as a boolean. Returns `None`
    /// for integer values other than `0` and `1`.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self.as_i64()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    // MARK: i64
    /// An `NSNumber` (or bare primitive) interpreted as a signed integer. Accepts
    /// either integer encoding; an unsigned value that does not fit `i64` yields
    /// `None`. Float/double values are *not* coerced.
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        match self.scalar()? {
            OutputData::SignedInteger(v) => Some(*v),
            OutputData::UnsignedInteger(v) => i64::try_from(*v).ok(),
            _ => None,
        }
    }

    // MARK: u64
    /// An `NSNumber` (or bare primitive) interpreted as an unsigned integer.
    /// Accepts either integer encoding; a negative signed value yields `None`.
    /// Float/double values are *not* coerced.
    #[must_use]
    pub fn as_u64(&self) -> Option<u64> {
        match self.scalar()? {
            OutputData::UnsignedInteger(v) => Some(*v),
            OutputData::SignedInteger(v) => u64::try_from(*v).ok(),
            _ => None,
        }
    }

    // MARK: f64
    /// An `NSNumber` (or bare primitive) interpreted as a double. Accepts `Float`
    /// and `Double`; integer values are *not* coerced.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self.scalar()? {
            OutputData::Double(v) => Some(*v),
            OutputData::Float(v) => Some(f64::from(*v)),
            _ => None,
        }
    }

    // MARK: Array
    /// The elements of an `NSArray` / `NSMutableArray` as a lazy [`FoundationArray`]
    /// view (the leading element-count group is skipped). Supports `len` /
    /// `get(index)` / `iter`; each element is a group-level [`Property`] on which
    /// the other accessors (`as_string`, `as_i64`, a nested `as_array`, …) apply.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let bytes: &[u8] = &[]; // a typedstream payload
    /// let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// let root = typedstream.oxidize().unwrap();
    ///
    /// for property in typedstream.resolve_properties(root).unwrap() {
    ///     if let Some(array) = property.as_array() {
    ///         println!("{} elements", array.len());
    ///         for element in &array {
    ///             println!("{:?}", element.as_string());
    ///         }
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn as_array(&self) -> Option<FoundationArray<'a, 'b>> {
        let (elements, len) = split_count(self.object_in_classes(ARRAY_CLASSES)?)?;
        Some(FoundationArray { elements, len })
    }

    // MARK: Set
    /// The members of an `NSSet` / `NSMutableSet` as a lazy [`FoundationArray`]
    /// view (unordered). Shares the type with [`as_array`](Self::as_array): the
    /// count group is skipped and each member is a group-level [`Property`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let bytes: &[u8] = &[];
    /// let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// let root = typedstream.oxidize().unwrap();
    ///
    /// for property in typedstream.resolve_properties(root).unwrap() {
    ///     if let Some(set) = property.as_set() {
    ///         for member in &set {
    ///             println!("{:?}", member.as_string());
    ///         }
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn as_set(&self) -> Option<FoundationArray<'a, 'b>> {
        let (elements, len) = split_count(self.object_in_classes(SET_CLASSES)?)?;
        Some(FoundationArray { elements, len })
    }

    // MARK: Dictionary
    /// The entries of an `NSDictionary` / `NSMutableDictionary` as a lazy
    /// [`FoundationDict`] view (the leading count group is skipped). Look up a
    /// string key with [`get`](FoundationDict::get), or iterate the `(key, value)`
    /// pairs; each key and value is a group-level [`Property`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crabstep::TypedStreamDeserializer;
    ///
    /// let bytes: &[u8] = &[];
    /// let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// let root = typedstream.oxidize().unwrap();
    ///
    /// for property in typedstream.resolve_properties(root).unwrap() {
    ///     if let Some(dict) = property.as_dictionary() {
    ///         // Look up a value by its string key.
    ///         if let Some(part) = dict.get("__kIMMessagePartAttributeName") {
    ///             println!("part index = {:?}", part.as_i64());
    ///         }
    ///         // Or iterate every entry.
    ///         for (key, value) in &dict {
    ///             println!("{:?} => {:?}", key.as_string(), value.as_i64());
    ///         }
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn as_dictionary(&self) -> Option<FoundationDict<'a, 'b>> {
        let (entries, len) = split_count(self.object_in_classes(DICT_CLASSES)?)?;
        Some(FoundationDict { entries, len })
    }

    // MARK: Date
    /// An `NSDate` as seconds since the Cocoa reference epoch (2001-01-01 00:00:00
    /// UTC). Use [`as_unix_time`](Self::as_unix_time) for seconds since the Unix
    /// epoch.
    #[must_use]
    pub fn as_date(&self) -> Option<f64> {
        let mut data = self.object_in_classes(&["NSDate"])?;
        match data.next()? {
            Property::Group(group) => match group.first()? {
                Property::Primitive(OutputData::Double(seconds)) => Some(*seconds),
                _ => None,
            },
            _ => None,
        }
    }

    /// An `NSDate` as seconds since the Unix epoch (1970-01-01 00:00:00 UTC),
    /// i.e. [`as_date`](Self::as_date)` + 978_307_200.0` (the offset between the
    /// Unix and Cocoa reference epochs).
    #[must_use]
    pub fn as_unix_time(&self) -> Option<f64> {
        self.as_date().map(|seconds| seconds + 978_307_200.0)
    }

    // MARK: URL
    /// The string of an `NSURL`. For an absolute URL this is the full URL; for a
    /// URL created relative to a base, it is the relative component (the base
    /// `NSURL` remains reachable through the generic [`Property`] tree).
    #[must_use]
    pub fn as_url(&self) -> Option<&'a str> {
        let data = self.object_in_classes(&["NSURL"])?;
        for group in data {
            if let Some(url) = group.as_string() {
                return Some(url);
            }
        }
        None
    }

    // MARK: Null
    /// Whether this property is an `NSNull` instance or a nil object reference
    /// ([`OutputData::Null`]).
    #[must_use]
    pub fn is_null(&self) -> bool {
        match self {
            Property::Object { name: "NSNull", .. } | Property::Primitive(OutputData::Null) => true,
            Property::Group(group) => matches!(
                group.first(),
                Some(
                    Property::Object { name: "NSNull", .. } | Property::Primitive(OutputData::Null)
                )
            ),
            _ => false,
        }
    }

    // MARK: Helpers
    /// If `self` is a group whose first item is an object whose class is in
    /// `classes`, return that object's data iterator.
    fn object_in_classes(&self, classes: &[&str]) -> Option<PropertyIterator<'a, 'b>> {
        let Property::Group(group) = self else {
            return None;
        };
        match group.first()? {
            Property::Object { name, data, .. } if classes.contains(&name) => Some(data),
            _ => None,
        }
    }

    /// The underlying scalar value of a group that is either a bare primitive or
    /// an `NSNumber` wrapping one.
    fn scalar(&self) -> Option<&'b OutputData<'a>> {
        let Property::Group(group) = self else {
            return None;
        };
        match group.first()? {
            Property::Primitive(value) => Some(value),
            Property::Object {
                name: "NSNumber",
                mut data,
                ..
            } => match data.next()? {
                Property::Group(inner) => match inner.first()? {
                    Property::Primitive(value) => Some(value),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}

// MARK: Iterators
/// Read the leading count group of a container's data, returning the remaining
/// iterator (positioned at the first element/entry) and the declared count.
fn split_count<'a, 'b: 'a>(
    mut data: PropertyIterator<'a, 'b>,
) -> Option<(PropertyIterator<'a, 'b>, usize)> {
    let count = data.next()?;
    let len = usize::try_from(count.as_i64().unwrap_or(0)).unwrap_or(0);
    Some((data, len))
}

/// A lazy view over the elements of an `NSArray` / `NSMutableArray` (or the
/// members of an `NSSet` / `NSMutableSet`), produced by [`Property::as_array`] /
/// [`Property::as_set`]. Cheap to clone and queryable any number of times; each
/// element is a group-level [`Property`], so the other accessors apply directly.
#[derive(Debug, Clone)]
pub struct FoundationArray<'a, 'b> {
    elements: PropertyIterator<'a, 'b>,
    len: usize,
}

impl<'a, 'b: 'a> FoundationArray<'a, 'b> {
    /// The number of elements (from the archived count).
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the collection has no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// A fresh iterator over the elements.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crabstep::TypedStreamDeserializer;
    /// # let bytes: &[u8] = &[];
    /// # let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// # let root = typedstream.oxidize().unwrap();
    /// # let property = typedstream.resolve_properties(root).unwrap().next().unwrap();
    /// # let array = property.as_array().unwrap();
    /// for element in array.iter() {
    ///     println!("{:?}", element.as_string());
    /// }
    /// ```
    #[must_use]
    pub fn iter(&self) -> FoundationArrayIter<'a, 'b> {
        FoundationArrayIter {
            inner: self.elements.clone(),
        }
    }

    /// The element at `index` (a linear `O(index)` walk).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crabstep::TypedStreamDeserializer;
    /// # let bytes: &[u8] = &[];
    /// # let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// # let root = typedstream.oxidize().unwrap();
    /// # let property = typedstream.resolve_properties(root).unwrap().next().unwrap();
    /// # let array = property.as_array().unwrap();
    /// println!("{:?}", array.get(2).and_then(|element| element.as_i64()));
    /// ```
    #[must_use]
    pub fn get(&self, index: usize) -> Option<Property<'a, 'b>> {
        self.iter().nth(index)
    }

    /// The first element.
    #[must_use]
    pub fn first(&self) -> Option<Property<'a, 'b>> {
        self.iter().next()
    }
}

impl<'a, 'b: 'a> IntoIterator for FoundationArray<'a, 'b> {
    type Item = Property<'a, 'b>;
    type IntoIter = FoundationArrayIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        FoundationArrayIter {
            inner: self.elements,
        }
    }
}

impl<'a, 'b: 'a> IntoIterator for &FoundationArray<'a, 'b> {
    type Item = Property<'a, 'b>;
    type IntoIter = FoundationArrayIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// The iterator yielded by [`FoundationArray::iter`] and its [`IntoIterator`] impl.
#[derive(Debug, Clone)]
pub struct FoundationArrayIter<'a, 'b> {
    inner: PropertyIterator<'a, 'b>,
}

impl<'a, 'b: 'a> Iterator for FoundationArrayIter<'a, 'b> {
    type Item = Property<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// A lazy view over the `(key, value)` pairs of an `NSDictionary` /
/// `NSMutableDictionary`, produced by [`Property::as_dictionary`]. Cheap to clone
/// and queryable any number of times; each key and value is a group-level
/// [`Property`].
#[derive(Debug, Clone)]
pub struct FoundationDict<'a, 'b> {
    entries: PropertyIterator<'a, 'b>,
    len: usize,
}

impl<'a, 'b: 'a> FoundationDict<'a, 'b> {
    /// The number of entries (from the archived count).
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the dictionary has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// A fresh iterator over the `(key, value)` pairs.
    #[must_use]
    pub fn iter(&self) -> FoundationDictIter<'a, 'b> {
        FoundationDictIter {
            inner: self.entries.clone(),
        }
    }

    /// The value for a string `key`, or `None` if absent.
    ///
    /// This is a linear scan (`O(n)`); dictionaries archived in a `typedstream`
    /// are typically small. Only string keys are matched — for other key types,
    /// or to build an index for many lookups, use [`iter`](Self::iter).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crabstep::TypedStreamDeserializer;
    /// # let bytes: &[u8] = &[];
    /// # let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// # let root = typedstream.oxidize().unwrap();
    /// # let property = typedstream.resolve_properties(root).unwrap().next().unwrap();
    /// # let dict = property.as_dictionary().unwrap();
    /// if let Some(value) = dict.get("__kIMMessagePartAttributeName") {
    ///     println!("{:?}", value.as_i64());
    /// }
    /// ```
    #[must_use]
    pub fn get(&self, key: &str) -> Option<Property<'a, 'b>> {
        self.iter()
            .find_map(|(k, v)| (k.as_string() == Some(key)).then_some(v))
    }

    /// Whether the dictionary contains the given string `key`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crabstep::TypedStreamDeserializer;
    /// # let bytes: &[u8] = &[];
    /// # let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// # let root = typedstream.oxidize().unwrap();
    /// # let property = typedstream.resolve_properties(root).unwrap().next().unwrap();
    /// # let dict = property.as_dictionary().unwrap();
    /// if dict.contains_key("__kIMMessagePartAttributeName") {
    ///     // the message-part attribute is present
    /// }
    /// ```
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// A fresh iterator over the keys.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crabstep::TypedStreamDeserializer;
    /// # let bytes: &[u8] = &[];
    /// # let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// # let root = typedstream.oxidize().unwrap();
    /// # let property = typedstream.resolve_properties(root).unwrap().next().unwrap();
    /// # let dict = property.as_dictionary().unwrap();
    /// for key in dict.keys() {
    ///     println!("{:?}", key.as_string());
    /// }
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = Property<'a, 'b>> {
        self.iter().map(|(k, _)| k)
    }

    /// A fresh iterator over the values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crabstep::TypedStreamDeserializer;
    /// # let bytes: &[u8] = &[];
    /// # let mut typedstream = TypedStreamDeserializer::new(bytes);
    /// # let root = typedstream.oxidize().unwrap();
    /// # let property = typedstream.resolve_properties(root).unwrap().next().unwrap();
    /// # let dict = property.as_dictionary().unwrap();
    /// for value in dict.values() {
    ///     println!("{:?}", value.as_i64());
    /// }
    /// ```
    pub fn values(&self) -> impl Iterator<Item = Property<'a, 'b>> {
        self.iter().map(|(_, v)| v)
    }
}

impl<'a, 'b: 'a> IntoIterator for FoundationDict<'a, 'b> {
    type Item = (Property<'a, 'b>, Property<'a, 'b>);
    type IntoIter = FoundationDictIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        FoundationDictIter {
            inner: self.entries,
        }
    }
}

impl<'a, 'b: 'a> IntoIterator for &FoundationDict<'a, 'b> {
    type Item = (Property<'a, 'b>, Property<'a, 'b>);
    type IntoIter = FoundationDictIter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// The iterator yielded by [`FoundationDict::iter`] and its [`IntoIterator`] impl.
#[derive(Debug, Clone)]
pub struct FoundationDictIter<'a, 'b> {
    inner: PropertyIterator<'a, 'b>,
}

impl<'a, 'b: 'a> Iterator for FoundationDictIter<'a, 'b> {
    type Item = (Property<'a, 'b>, Property<'a, 'b>);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.next()?;
        // A missing value means an unpaired trailing key (malformed data); drop it.
        let value = self.inner.next()?;
        Some((key, value))
    }
}

// MARK: Tests
#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::{vec, vec::Vec};
    use std::{env::current_dir, fs::File, io::Read};

    use crate::deserializer::iter::Property;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    /// Load a fixture by path relative to `src/test_data`.
    fn load(rel: &str) -> Vec<u8> {
        let path = current_dir().unwrap().join("src/test_data").join(rel);
        let mut file =
            File::open(&path).unwrap_or_else(|e| panic!("opening fixture {path:?}: {e}"));
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();
        bytes
    }

    // MARK: class_name

    #[test]
    fn class_name_resolves_element_classes() {
        // Iterating an `NSArray` yields one group per element; each element group's
        // `class_name()` is the element's class, and the bare count group is None.
        let bytes = load("foundation/NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let names: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.class_name())
            .collect();

        assert!(names.contains(&"NSString"), "names: {names:?}");
        assert!(names.contains(&"NSNumber"), "names: {names:?}");
    }

    #[test]
    fn class_name_on_direct_object() {
        // `class_name()` also works when called on a [`Property::Object`] directly
        // (e.g. after stepping into a group with `iter().next()`).
        let bytes = load("foundation/NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let object_names: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| match group {
                Property::Group(g) => g.first(),
                _ => None,
            })
            .filter_map(|inner| inner.class_name())
            .collect();

        assert!(
            object_names.contains(&"NSString"),
            "names: {object_names:?}"
        );
    }

    #[test]
    fn class_name_is_none_for_primitive() {
        // The `NSArray`'s count group is a bare primitive, no class.
        let bytes = load("foundation/NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let has_primitive_group = ts
            .resolve_properties(root)
            .unwrap()
            .any(|group| group.class_name().is_none());

        assert!(has_primitive_group);
    }

    // MARK: as_string

    #[test]
    fn as_string_reads_both_string_variants() {
        // NSArray([NSString "imm", NSMutableString "mut"])
        let bytes = load("foundation/NestedStrings");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let strings: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();

        assert_eq!(strings.len(), 2, "{strings:?}");
        assert!(strings.contains(&"imm"), "{strings:?}");
        assert!(strings.contains(&"mut"), "{strings:?}");
    }

    #[test]
    fn as_string_reads_attributed_backing_text() {
        // NSArray([NSAttributedString "styled"]) — exercises the scan path.
        let bytes = load("foundation/NestedAttributed");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let strings: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();

        assert_eq!(strings, vec!["styled"]);
    }

    #[test]
    fn as_string_reads_real_mutable_attributed_body() {
        // Real Messages NSMutableAttributedString: backing store is an
        // NSMutableString reached as one of the root's groups (plain path).
        let bytes = load("AttributedBodyTextOnly");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let strings: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();

        assert!(strings.contains(&"Noter test"), "{strings:?}");
    }

    // MARK: as_data

    #[test]
    fn as_data_reads_both_data_variants() {
        // NSArray([NSData [1,2], NSMutableData [3,4,5]])
        let bytes = load("foundation/NestedData");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let datas: Vec<&[u8]> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_data())
            .collect();

        assert_eq!(datas.len(), 2, "{datas:?}");
        assert!(datas.contains(&&[0x01, 0x02][..]), "{datas:?}");
        assert!(datas.contains(&&[0x03, 0x04, 0x05][..]), "{datas:?}");
    }

    // MARK: numbers

    #[test]
    fn as_f64_reads_decimal_double() {
        // NumberDouble root: bare-primitive path, exercises the B1 DECIMAL fix.
        let bytes = load("foundation/NumberDouble");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_f64(), Some(100.5));
    }

    #[test]
    fn as_f64_reads_float() {
        let bytes = load("foundation/NumberFloat");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_f64(), Some(3.5));
    }

    #[test]
    fn as_i64_reads_large_negative() {
        // NumberInt64 = -9_000_000_000: bare-primitive path, exercises the B2 fix.
        let bytes = load("foundation/NumberInt64");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_i64(), Some(-9_000_000_000));
    }

    #[test]
    fn integer_coercion_and_strictness() {
        // NumberInt = SignedInteger(42).
        let bytes = load("foundation/NumberInt");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_i64(), Some(42));
        assert_eq!(group.as_u64(), Some(42)); // non-negative signed coerces to u64
        assert_eq!(group.as_f64(), None); // no silent int -> float
        assert_eq!(group.as_string(), None); // a number is not a string
        assert_eq!(group.as_data(), None);
    }

    #[test]
    fn unsigned_coercion_rejects_negative() {
        let bytes = load("foundation/NumberInt64"); // -9_000_000_000
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_u64(), None);
    }

    // MARK: as_bool
    #[test]
    fn as_bool_reads_boolean() {
        let bytes = load("foundation/NumberBool"); // NSNumber(true) -> SignedInteger(1)
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert_eq!(group.as_bool(), Some(true));
        assert_eq!(group.as_i64(), Some(1));
    }

    // MARK: wrappers

    #[test]
    fn unwraps_nsnumber_object_and_reads_dict_entries() {
        // NSDictionaryNested = { "arr": [..], "data": <bytes>, "num": NSNumber(7) }.
        // Keys are NSString objects; the NSNumber value exercises the unwrap path;
        // the NSData value exercises as_data on a dictionary value.
        let bytes = load("foundation/NSDictionaryNested");

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let keys: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_string())
            .collect();
        assert!(
            keys.contains(&"arr") && keys.contains(&"data") && keys.contains(&"num"),
            "{keys:?}"
        );

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let ints: Vec<i64> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_i64())
            .collect();
        assert!(ints.contains(&7), "{ints:?}"); // NSNumber(7) value, unwrapped

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let datas: Vec<&[u8]> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_data())
            .collect();
        assert!(datas.contains(&&[0x01, 0x02][..]), "{datas:?}"); // NSData value
    }

    // MARK: as_array / as_set

    #[test]
    fn as_array_yields_elements_both_variants_and_empty() {
        // NestedContainers root holds NSArray[1,2], NSMutableArray[3], an empty
        // NSArray, then non-array elements (dicts/sets) which as_array ignores.
        let bytes = load("foundation/NestedContainers");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let arrays: Vec<Vec<i64>> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_array())
            .map(|array| array.into_iter().filter_map(|el| el.as_i64()).collect())
            .collect();

        assert_eq!(arrays, vec![vec![1, 2], vec![3], vec![]]);
    }

    #[test]
    fn as_set_yields_members_both_variants() {
        let bytes = load("foundation/NestedContainers");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let sets: Vec<Vec<&str>> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_set())
            .map(|set| {
                set.into_iter()
                    .filter_map(|member| member.as_string())
                    .collect()
            })
            .collect();

        assert_eq!(sets, vec![vec!["s"], vec!["ms"]]);
    }

    // MARK: as_dictionary

    #[test]
    fn as_dictionary_yields_pairs_both_variants() {
        // NestedContainers holds NSDictionary{k:9} and NSMutableDictionary{mk:8}.
        let bytes = load("foundation/NestedContainers");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let entries: Vec<(&str, i64)> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_dictionary())
            .flat_map(|dict| {
                dict.into_iter()
                    .filter_map(|(key, value)| Some((key.as_string()?, value.as_i64()?)))
                    .collect::<Vec<_>>()
            })
            .collect();

        assert_eq!(entries.len(), 2, "{entries:?}");
        assert!(entries.contains(&("k", 9)), "{entries:?}");
        assert!(entries.contains(&("mk", 8)), "{entries:?}");
    }

    #[test]
    fn nested_array_inside_array() {
        // NSArrayNested = [NSString "top", NSArray[1, 2]] — as_array on the nested
        // element resolves the inner array.
        let bytes = load("foundation/NSArrayNested");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let inner: Vec<Vec<i64>> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_array())
            .map(|array| array.into_iter().filter_map(|el| el.as_i64()).collect())
            .collect();

        assert_eq!(inner, vec![vec![1, 2]]);
    }

    // MARK: container negatives

    #[test]
    fn container_accessors_reject_non_containers() {
        let bytes = load("foundation/NumberInt");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert!(group.as_array().is_none());
        assert!(group.as_set().is_none());
        assert!(group.as_dictionary().is_none());
    }

    // MARK: as_date / as_unix_time

    #[test]
    fn as_date_and_unix_time() {
        // NestedScalars holds NSDate(timeIntervalSinceReferenceDate: 21692800).
        let bytes = load("foundation/NestedScalars");

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let dates: Vec<f64> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_date())
            .collect();
        assert_eq!(dates, vec![21692800.0]);

        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let unix: Vec<f64> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_unix_time())
            .collect();
        assert_eq!(unix, vec![1_000_000_000.0]);
    }

    // MARK: as_url

    #[test]
    fn as_url_absolute_and_relative() {
        // NestedScalars holds an absolute NSURL then a relative one; the relative
        // one yields its relative component.
        let bytes = load("foundation/NestedScalars");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let urls: Vec<&str> = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_url())
            .collect();

        assert_eq!(urls, vec!["https://example.com/path?q=1", "page.html"]);
    }

    // MARK: is_null

    #[test]
    fn is_null_detects_nsnull() {
        let bytes = load("foundation/NestedScalars");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let null_count = ts
            .resolve_properties(root)
            .unwrap()
            .filter(|group| group.is_null())
            .count();

        assert_eq!(null_count, 1);
    }

    #[test]
    fn scalar_accessors_reject_wrong_types() {
        let bytes = load("foundation/NumberInt");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let group = ts.resolve_properties(root).unwrap().next().unwrap();

        assert!(!group.is_null());
        assert_eq!(group.as_date(), None);
        assert_eq!(group.as_unix_time(), None);
        assert_eq!(group.as_url(), None);
    }

    // MARK: container views

    #[test]
    fn array_view_len_get_first() {
        // First array element of NestedContainers is NSArray[1, 2].
        let bytes = load("foundation/NestedContainers");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let array = ts
            .resolve_properties(root)
            .unwrap()
            .find_map(|group| group.as_array())
            .unwrap();

        assert_eq!(array.len(), 2);
        assert!(!array.is_empty());
        assert_eq!(array.first().and_then(|e| e.as_i64()), Some(1));
        assert_eq!(array.get(0).and_then(|e| e.as_i64()), Some(1));
        assert_eq!(array.get(1).and_then(|e| e.as_i64()), Some(2));
        assert!(array.get(2).is_none());
    }

    #[test]
    fn array_view_empty() {
        // NestedContainers also holds an empty NSArray.
        let bytes = load("foundation/NestedContainers");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let empty = ts
            .resolve_properties(root)
            .unwrap()
            .filter_map(|group| group.as_array())
            .find(|array| array.is_empty())
            .unwrap();

        assert_eq!(empty.len(), 0);
        assert!(empty.first().is_none());
        assert_eq!(empty.iter().count(), 0);
    }

    #[test]
    fn dict_view_get_contains_keys_values() {
        // First dictionary of NestedContainers is NSDictionary { "k": 9 }.
        let bytes = load("foundation/NestedContainers");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.oxidize().unwrap();
        let dict = ts
            .resolve_properties(root)
            .unwrap()
            .find_map(|group| group.as_dictionary())
            .unwrap();

        assert_eq!(dict.len(), 1);
        assert!(!dict.is_empty());
        assert_eq!(dict.get("k").and_then(|v| v.as_i64()), Some(9));
        assert!(dict.get("missing").is_none());
        assert!(dict.contains_key("k"));
        assert!(!dict.contains_key("missing"));

        let keys: Vec<&str> = dict.keys().filter_map(|k| k.as_string()).collect();
        assert_eq!(keys, vec!["k"]);
        let values: Vec<i64> = dict.values().filter_map(|v| v.as_i64()).collect();
        assert_eq!(values, vec![9]);
    }
}
