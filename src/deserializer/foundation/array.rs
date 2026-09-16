//! `as_array` / `as_set` and the [`FoundationArray`] view.

use crate::deserializer::foundation::helpers::split_count;
use crate::deserializer::foundation::names::{ARRAY_CLASSES, SET_CLASSES};
use crate::deserializer::iter::{Property, PropertyIterator};

impl<'a, 'b: 'a> Property<'a, 'b> {
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

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn root_object_resolves_as_array() {
        // Root NSArray([NSString "a", NSNumber 1, NSString "b"]) via `root()`.
        let bytes = load("foundation/NSArray");
        let mut ts = TypedStreamDeserializer::new(&bytes);
        let root = ts.root().unwrap();
        let array = root.as_array().unwrap();
        assert_eq!(array.len(), 3);
        let strings: Vec<&str> = array.iter().filter_map(|e| e.as_string()).collect();
        assert_eq!(strings, vec!["a", "b"]);
    }

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

    #[test]
    fn nested_array_inside_array() {
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
}
