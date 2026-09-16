//! Type tags that denote the type of data stored in a `typedstream`
use crate::{
    deserializer::{consumed::Consumed, number::read_unsigned_int, read::read_exact_bytes},
    error::{Result, TypedStreamError},
};
use alloc::vec::Vec;

/// Represents primitive types of data that can be stored in a `typedstream`
///
/// These type encodings are partially documented [here](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html#//apple_ref/doc/uid/TP40008048-CH100-SW1) by Apple.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type<'a> {
    /// Encoded string data, usually embedded in an object. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x2B` | [`+`](https://www.compart.com/en/unicode/U+002B) |
    Utf8String,
    /// A C string (`char *`) encoded as a nullable shared pointer:
    /// [`EMPTY`](crate::deserializer::constants::EMPTY) for `NULL`; otherwise,
    /// a new object-table slot with a shared-string index or a reference to an
    /// earlier pointer's slot. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x2A` | [`*`](https://www.compart.com/en/unicode/U+002A) |
    CString,
    /// A method selector (`SEL`), written as a shared string: the first
    /// occurrence is a literal, later ones are references to it, and a `NULL`
    /// selector is [`EMPTY`](crate::deserializer::constants::EMPTY). Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x3A` | [`:`](https://www.compart.com/en/unicode/U+003A) |
    Selector,
    /// An instance of a class, usually with data. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x40` | [`@`](https://www.compart.com/en/unicode/U+0040) |
    Object,
    /// A class reference (`Class`), written as a class chain: name, version,
    /// and superclass, exactly as an object's class header. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x23` | [`#`](https://www.compart.com/en/unicode/U+0023) |
    Class,
    /// An [`i8`], [`i16`], or [`i32`]. Denoted by:
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x63` | [`c`](https://www.compart.com/en/unicode/U+0063) |
    /// | `0x69` | [`i`](https://www.compart.com/en/unicode/U+0069) |
    /// | `0x6c` | [`l`](https://www.compart.com/en/unicode/U+006c) |
    /// | `0x71` | [`q`](https://www.compart.com/en/unicode/U+0071) |
    /// | `0x73` | [`s`](https://www.compart.com/en/unicode/U+0073) |
    ///
    /// The width is determined by the prefix: [`i8`] has none, [`i16`] has `0x81`, and [`i32`] has `0x82`.
    SignedInt,
    /// A [`u8`], [`u16`], or [`u32`]. Denoted by:
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x43` | [`C`](https://www.compart.com/en/unicode/U+0043) |
    /// | `0x49` | [`I`](https://www.compart.com/en/unicode/U+0049) |
    /// | `0x4c` | [`L`](https://www.compart.com/en/unicode/U+004c) |
    /// | `0x51` | [`Q`](https://www.compart.com/en/unicode/U+0051) |
    /// | `0x53` | [`S`](https://www.compart.com/en/unicode/U+0053) |
    /// | `0x42` | [`B`](https://www.compart.com/en/unicode/U+0042) |
    ///
    /// The width is determined by the prefix: [`u8`] has none, [`u16`] has `0x81`, and [`u32`] has `0x82`.
    /// `B` is C's `_Bool` (`BOOL` on arm64): one byte holding 0 or 1, read like `C`.
    UnsignedInt,
    /// An [`f32`]. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x66` | [`f`](https://www.compart.com/en/unicode/U+0066) |
    Float,
    /// An [`f64`]. Denoted by:
    ///
    /// | Hex    | UTF-8 |
    /// |--------|-------|
    /// | `0x64` | [`d`](https://www.compart.com/en/unicode/U+0064) |
    Double,
    /// Some text we can reuse later, i.e. a class name.
    String(&'a str),
    /// `length` raw bytes: a `char` array, `[Nc]` or `[NC]`. An array of any
    /// other element type is expanded to `N` copies of the element's types,
    /// because `NSArchiver` writes each element with its own encoding.
    Array(usize),
}

impl<'a> Type<'a> {
    #[inline]
    pub(crate) fn new_string(str: &'a str) -> Self {
        Self::String(str)
    }

    /// Reads a length-prefixed type descriptor literal and returns its text with
    /// its slot types. Preserve the text for the shared-string table; use this
    /// entry by index when decoding a later `char *` value.
    pub(crate) fn read_new_type(data: &'_ [u8]) -> Result<Consumed<(&'_ str, TypeEntry<'_>)>> {
        let type_length = read_unsigned_int(data)?;
        let type_bytes = read_exact_bytes(
            &data[type_length.bytes_consumed..],
            type_length.value as usize,
        )?;
        let bytes_consumed = type_length.bytes_consumed + type_bytes.len();
        let text = core::str::from_utf8(type_bytes)?;
        let entry = Type::parse_descriptor(text, data.len() - bytes_consumed)?;
        Ok(Consumed::new((text, entry), bytes_consumed))
    }

    /// Parses a type descriptor into its slot types.
    ///
    /// Input: an Objective-C type-encoding string.
    /// Leaf-value order: struct members flat; array elements one after another.
    /// Thus `{_NSRange=QQ}` is two unsigned ints, `[2i]` is two signed ints, and
    /// `i{?=ii}i` is four signed ints. Exception: keep a `char` array, `[Nc]`,
    /// as `N` raw bytes.
    ///
    /// Use `max_slots` to bound the bytes available after the descriptor. At
    /// least one byte belongs to each decoded slot; an array that expands past
    /// this bound is malformed.
    pub(crate) fn parse_descriptor(text: &str, max_slots: usize) -> Result<TypeEntry<'a>> {
        let bytes = text.as_bytes();

        // The overwhelming majority of descriptors are a single scalar letter,
        // so keep that case off the heap.
        if let [byte] = bytes
            && let Some(ty) = Type::scalar(*byte)
        {
            return Ok(TypeEntry::One(ty));
        }

        let mut types = Vec::new();
        let mut pos = 0;
        while pos < bytes.len() {
            Type::parse_one(bytes, &mut pos, &mut types, max_slots)?;
        }
        Ok(if types.len() == 1 {
            TypeEntry::One(types.pop().unwrap())
        } else {
            TypeEntry::Many(types)
        })
    }

    /// The type a single non-aggregate encoding character denotes, if any.
    #[inline]
    fn scalar(byte: u8) -> Option<Self> {
        Some(match byte {
            b'@' => Self::Object,
            b'#' => Self::Class,
            b':' => Self::Selector,
            b'*' => Self::CString,
            b'+' => Self::Utf8String,
            b'f' => Self::Float,
            b'd' => Self::Double,
            b'c' | b'i' | b'l' | b'q' | b's' => Self::SignedInt,
            b'C' | b'I' | b'L' | b'Q' | b'S' | b'B' => Self::UnsignedInt,
            _ => return None,
        })
    }

    /// Parses one type at `pos`, appending its leaf types to `out` and leaving
    /// `pos` on the byte after it. `max_slots` caps any single array expansion.
    fn parse_one(
        bytes: &[u8],
        pos: &mut usize,
        out: &mut Vec<Type<'a>>,
        max_slots: usize,
    ) -> Result<()> {
        // Skip method qualifiers (`const`, `in`, `out`, ...) ahead of the type.
        // `NSArchiver` strips them, so this is defensive.
        while matches!(
            bytes.get(*pos),
            Some(b'r' | b'n' | b'N' | b'o' | b'O' | b'R' | b'V')
        ) {
            *pos += 1;
        }
        let Some(&byte) = bytes.get(*pos) else {
            return Err(TypedStreamError::InvalidType(0));
        };
        *pos += 1;

        match byte {
            b'[' => {
                // `[` count element `]`
                let mut count = 0usize;
                let mut saw_digit = false;
                while let Some(digit) = bytes.get(*pos).filter(|b| b.is_ascii_digit()) {
                    count = count
                        .checked_mul(10)
                        .and_then(|c| c.checked_add(usize::from(digit - b'0')))
                        .ok_or(TypedStreamError::InvalidArray(*pos))?;
                    saw_digit = true;
                    *pos += 1;
                }
                if !saw_digit {
                    return Err(TypedStreamError::InvalidArray(*pos));
                }
                // A `char` array is the one aggregate kept whole: `N` raw bytes.
                if let Some(b'c' | b'C') = bytes.get(*pos)
                    && bytes.get(*pos + 1) == Some(&b']')
                {
                    *pos += 2;
                    out.push(Type::Array(count));
                    return Ok(());
                }
                // Any other element: its leaves, `count` times over.
                let mut element = Vec::new();
                Type::parse_one(bytes, pos, &mut element, max_slots)?;
                if bytes.get(*pos) != Some(&b']') {
                    return Err(TypedStreamError::InvalidType(b'['));
                }
                *pos += 1;
                if count
                    .checked_mul(element.len())
                    .is_none_or(|slots| slots > max_slots)
                {
                    return Err(TypedStreamError::InvalidArray(*pos));
                }
                for _ in 0..count {
                    out.extend_from_slice(&element);
                }
                Ok(())
            }
            b'{' => {
                // `{` name [`=` member*] `}`. Skip the name rather than scanning
                // it: `C`, `S`, and `i` in `CGSize` are not slots.
                while let Some(&b) = bytes.get(*pos) {
                    *pos += 1;
                    match b {
                        b'=' => break,
                        b'}' => return Ok(()),
                        _ => {}
                    }
                }
                loop {
                    match bytes.get(*pos) {
                        Some(b'}') => {
                            *pos += 1;
                            return Ok(());
                        }
                        Some(_) => Type::parse_one(bytes, pos, out, max_slots)?,
                        None => return Err(TypedStreamError::InvalidType(b'{')),
                    }
                }
            }
            // Encodings NSArchiver refuses to write (unions, bitfields, pointers)
            // or with no value at all (`void`, unknown `?`).
            b'(' | b'b' | b'^' | b'v' | b'?' => Err(TypedStreamError::InvalidType(byte)),
            other => {
                out.push(Type::scalar(other).ok_or(TypedStreamError::InvalidType(other))?);
                Ok(())
            }
        }
    }
}

/// One entry in the deserializer's type table.
///
/// A type descriptor usually resolves to a single [`Type`]; storing that inline
/// avoids a heap allocation per entry (the table was previously a
/// `Vec<Vec<Type>>`). Multi-type descriptors fall back to a [`Vec`].
#[derive(Debug, Clone, PartialEq)]
pub enum TypeEntry<'a> {
    /// A single type, stored inline.
    One(Type<'a>),
    /// Two or more types.
    Many(Vec<Type<'a>>),
}

impl<'a> TypeEntry<'a> {
    /// The number of types in this entry.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            TypeEntry::One(_) => 1,
            TypeEntry::Many(types) => types.len(),
        }
    }

    /// Whether the entry has no types.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, TypeEntry::Many(types) if types.is_empty())
    }

    /// The first type in the entry, if any.
    #[must_use]
    pub fn first(&self) -> Option<&Type<'a>> {
        match self {
            TypeEntry::One(ty) => Some(ty),
            TypeEntry::Many(types) => types.first(),
        }
    }

    /// Build a [`TypeEntry`] from a list of types, normalizing the single-type
    /// case to [`TypeEntry::One`]. Used by tests to express expected type tables
    /// in the pre-existing nested style.
    #[cfg(test)]
    pub(crate) fn from_types(mut types: Vec<Type<'a>>) -> Self {
        if types.len() == 1 {
            TypeEntry::One(types.pop().unwrap())
        } else {
            TypeEntry::Many(types)
        }
    }
}

impl<'a> core::ops::Index<usize> for TypeEntry<'a> {
    type Output = Type<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            TypeEntry::One(ty) => {
                assert_eq!(index, 0, "index out of bounds for single-type entry");
                ty
            }
            TypeEntry::Many(types) => &types[index],
        }
    }
}

#[cfg(test)]
mod type_encoding_tests {
    use alloc::vec;

    use super::{Type, TypeEntry};
    use crate::error::TypedStreamError;

    /// Parses a bare encoding string as a length-prefixed descriptor followed by
    /// `remaining` bytes of stream, which bound array expansion.
    fn parse_with(
        encoding: &str,
        remaining: usize,
    ) -> Result<TypeEntry<'static>, TypedStreamError> {
        let mut bytes = vec![u8::try_from(encoding.len()).unwrap()];
        bytes.extend_from_slice(encoding.as_bytes());
        bytes.resize(bytes.len() + remaining, 0);
        // The parser borrows nothing from the descriptor, so the lifetime is free.
        Type::read_new_type(bytes.leak()).map(|c| {
            assert_eq!(c.bytes_consumed, encoding.len() + 1);
            assert_eq!(c.value.0, encoding);
            c.value.1
        })
    }

    fn parse(encoding: &str) -> Result<TypeEntry<'static>, TypedStreamError> {
        parse_with(encoding, 64)
    }

    #[test]
    fn scalars() {
        for (encoding, expected) in [
            ("@", Type::Object),
            ("#", Type::Class),
            (":", Type::Selector),
            ("*", Type::CString),
            ("+", Type::Utf8String),
            ("f", Type::Float),
            ("d", Type::Double),
            ("c", Type::SignedInt),
            ("q", Type::SignedInt),
            ("C", Type::UnsignedInt),
            ("Q", Type::UnsignedInt),
            ("B", Type::UnsignedInt),
        ] {
            assert_eq!(
                parse(encoding).unwrap(),
                TypeEntry::One(expected),
                "{encoding}"
            );
        }
        assert_eq!(
            parse("iI").unwrap(),
            TypeEntry::Many(vec![Type::SignedInt, Type::UnsignedInt])
        );
        // NSArchiver never writes an empty descriptor; parse it as no slots anyway.
        assert_eq!(parse("").unwrap(), TypeEntry::Many(vec![]));
    }

    #[test]
    fn qualifiers_are_skipped() {
        assert_eq!(parse("ri").unwrap(), TypeEntry::One(Type::SignedInt));
        assert_eq!(parse("oNi").unwrap(), TypeEntry::One(Type::SignedInt));
    }

    #[test]
    fn structs_flatten_and_names_are_not_scanned() {
        use Type::{Double as D, SignedInt as I, UnsignedInt as U};
        assert_eq!(parse("{_NSRange=QQ}").unwrap(), TypeEntry::Many(vec![U, U]));
        // `C`, `S`, `i` in the name are letters, not slots.
        assert_eq!(parse("{CGSize=dd}").unwrap(), TypeEntry::Many(vec![D, D]));
        assert_eq!(
            parse("{CGRect={CGPoint=dd}{CGSize=dd}}").unwrap(),
            TypeEntry::Many(vec![D, D, D, D])
        );
        assert_eq!(
            parse("{?={?=ii}q}").unwrap(),
            TypeEntry::Many(vec![I, I, I])
        );
        assert_eq!(
            parse("i{?=ii}i").unwrap(),
            TypeEntry::Many(vec![I, I, I, I])
        );
        // Opaque and empty structs have no slots.
        assert_eq!(parse("{opaque}").unwrap(), TypeEntry::Many(vec![]));
        assert_eq!(parse("{empty=}").unwrap(), TypeEntry::Many(vec![]));
        assert_eq!(parse("i{empty=}i").unwrap(), TypeEntry::Many(vec![I, I]));
    }

    #[test]
    fn arrays() {
        use Type::{Array as A, SignedInt as I};
        // `char` arrays stay whole as raw bytes; everything else is expanded.
        assert_eq!(parse("[4c]").unwrap(), TypeEntry::One(A(4)));
        assert_eq!(parse("[648C]").unwrap(), TypeEntry::One(A(648)));
        assert_eq!(parse("[2i]").unwrap(), TypeEntry::Many(vec![I, I]));
        assert_eq!(parse("[3s]").unwrap(), TypeEntry::Many(vec![I, I, I]));
        assert_eq!(parse("[2[2c]]").unwrap(), TypeEntry::Many(vec![A(2), A(2)]));
        assert_eq!(
            parse("[2{?=ic}]").unwrap(),
            TypeEntry::Many(vec![I, I, I, I])
        );
        assert_eq!(parse("[0i]").unwrap(), TypeEntry::Many(vec![]));
        assert_eq!(parse("[1i]").unwrap(), TypeEntry::One(I));
        assert_eq!(parse("i[2c]i").unwrap(), TypeEntry::Many(vec![I, A(2), I]));
    }

    #[test]
    fn unencodable_and_malformed_are_errors() {
        for (encoding, byte) in [
            ("(u=if)", b'('),
            ("b3", b'b'),
            ("^i", b'^'),
            ("v", b'v'),
            ("?", b'?'),
            ("x", b'x'),
            ("{?=ii", b'{'),
            ("[2i", b'['),
            ("i(u=if)", b'('),
        ] {
            assert!(
                matches!(parse(encoding), Err(TypedStreamError::InvalidType(b)) if b == byte),
                "{encoding}"
            );
        }
        assert!(matches!(
            parse("[i]"),
            Err(TypedStreamError::InvalidArray(_))
        ));
        assert!(matches!(
            parse("[]"),
            Err(TypedStreamError::InvalidArray(_))
        ));
    }

    #[test]
    fn array_expansion_is_bounded_by_the_stream() {
        use Type::{Array as A, SignedInt as I};
        // Exactly enough bytes for the slots is fine; one short is not.
        assert_eq!(parse_with("[4i]", 4).unwrap(), TypeEntry::Many(vec![I; 4]));
        assert!(matches!(
            parse_with("[4i]", 3),
            Err(TypedStreamError::InvalidArray(_))
        ));
        assert!(matches!(
            parse_with("[2{?=ii}]", 3),
            Err(TypedStreamError::InvalidArray(_))
        ));
        // Rejected before any allocation, including on multiplication overflow.
        assert!(matches!(
            parse_with("[4000000000i]", 8),
            Err(TypedStreamError::InvalidArray(_))
        ));
        assert!(matches!(
            parse_with("[18446744073709551615{?=ii}]", 8),
            Err(TypedStreamError::InvalidArray(_))
        ));
        // A byte array is one slot however long; its bound is checked when read.
        assert_eq!(
            parse_with("[4000000000c]", 0).unwrap(),
            TypeEntry::One(A(4_000_000_000))
        );
    }
}
