//! `as_url`: `NSURL`.

use crate::deserializer::iter::Property;

impl<'a, 'b: 'a> Property<'a, 'b> {
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
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use crate::deserializer::foundation::test_support::load;
    use crate::deserializer::typedstream::TypedStreamDeserializer;

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
}
