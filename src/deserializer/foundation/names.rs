//! Cluster class-name sets.
//!
//! Foundation archives several types as both an immutable and a mutable variant
//! (and the data cluster archives as both `NSData` and `NSMutableData`); matching
//! all variants in one place keeps the footgun centralized.

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
