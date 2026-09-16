//! One entry in the shared-string table, and why an entry needs two views.

use crate::{
    error::Result,
    models::types::{Type, TypeEntry},
};

/// One entry in the deserializer's shared-string table.
///
/// `NSArchiver` writes each string into the stream once, as a literal, and
/// refers to it by index thereafter. Type descriptors, class names,
/// selectors, and `char *` values all go through this one table, so the index
/// space is shared among all of them. This matters because a single entry can
/// play more than one role: `NSValue` writes its `objCType` as a `char *`
/// value, then types the payload that follows with a reference to that same
/// string. The first use wants the text; the second wants it parsed as a
/// descriptor.
///
/// To accomplish this, every entry keeps its text, and the descriptor view is
/// parsed at most once. A descriptor literal is parsed at registration, since
/// the deserializer is about to read values with it. A name or value is not,
/// since a class name is never a valid descriptor and parsing it would fail;
/// its view is parsed only if a descriptor reference later resolves to it.
/// This means an entry never changes meaning: the text is always there, and a
/// parsed view, once present, is exactly what the text says.
///
/// Equality compares the text alone. The parsed view is a cache of the text,
/// so two entries with the same text are the same string whether or not
/// either has been used as a descriptor yet.
#[derive(Clone)]
pub struct SharedString<'a> {
    /// The text, borrowed from the stream.
    pub text: &'a str,
    /// The descriptor view, once something has needed it.
    parsed: Option<TypeEntry>,
}

impl<'a> SharedString<'a> {
    /// A freshly registered string, with no descriptor view yet.
    #[must_use]
    pub const fn new(text: &'a str) -> Self {
        Self { text, parsed: None }
    }

    /// The descriptor view, parsing the text the first time it is asked for.
    ///
    /// `max_slots` is the number of stream bytes that follow the descriptor.
    /// Since every slot reads at least one byte, an array that expands past
    /// that count cannot be satisfied by the stream, and the parser rejects it
    /// before allocating for it.
    pub fn descriptor(&mut self, max_slots: usize) -> Result<&TypeEntry> {
        if self.parsed.is_none() {
            self.parsed = Some(Type::parse_descriptor(self.text, max_slots)?);
        }
        Ok(self.parsed.as_ref().expect("set above"))
    }

    /// The descriptor view, if something has already parsed it.
    #[must_use]
    pub fn parsed(&self) -> Option<&TypeEntry> {
        self.parsed.as_ref()
    }
}

impl PartialEq for SharedString<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl PartialEq<str> for SharedString<'_> {
    fn eq(&self, other: &str) -> bool {
        self.text == other
    }
}

impl PartialEq<&str> for SharedString<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.text == *other
    }
}

impl core::fmt::Debug for SharedString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.text)?;
        if let Some(parsed) = &self.parsed {
            write!(f, " ({parsed:?})")?;
        }
        Ok(())
    }
}

impl core::fmt::Display for SharedString<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::SharedString;
    use crate::models::types::{Type, TypeEntry};

    /// The table is plain data: no interior mutability, so it stays `Sync`.
    #[test]
    fn shared_string_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SharedString<'static>>();
    }

    #[test]
    fn descriptor_parses_once_and_equality_ignores_it() {
        let mut entry = SharedString::new("iI");
        assert_eq!(entry.parsed(), None);
        assert_eq!(
            entry.descriptor(8).unwrap(),
            &TypeEntry::Many(alloc::vec![Type::SignedInt, Type::UnsignedInt])
        );
        assert!(entry.parsed().is_some());
        assert_eq!(entry, SharedString::new("iI"));
        assert_eq!(entry, "iI");
        assert_ne!(entry, SharedString::new("iC"));
        // A name is never a valid descriptor; the failure is not cached.
        let mut name = SharedString::new("NSString");
        assert!(name.descriptor(8).is_err());
        assert_eq!(name.parsed(), None);
    }
}
