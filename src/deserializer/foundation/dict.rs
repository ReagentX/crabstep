//! `as_dictionary` and the [`FoundationDict`] view.

use crate::deserializer::foundation::helpers::split_count;
use crate::deserializer::foundation::names::DICT_CLASSES;
use crate::deserializer::iter::{Property, PropertyIterator};

impl<'a, 'b: 'a> Property<'a, 'b> {
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

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

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
