/*!
Typed accessors for common Apple [Foundation](https://developer.apple.com/documentation/foundation) classes.

Enabled by the `foundation` cargo feature. These methods interpret the generic
[`Property`](crate::deserializer::iter::Property) tree as specific Foundation
types (`NSString`, `NSDictionary`, …) so consumers do not have to re-implement
class-name matching (and re-discover its footguns, e.g. that the data cluster
archives as both `NSData` and `NSMutableData`).

This feature is purely for convenience: the parser and the
[`Property`](crate::deserializer::iter::Property) /
[`OutputData`](crate::models::output_data::OutputData) model are unchanged whether
or not it is enabled, and any class not modeled here stays reachable through
[`Property::Object`](crate::deserializer::iter::Property::Object), so nothing is
ever lost.

Accessors are methods on a group-level
[`Property`](crate::deserializer::iter::Property): the value yielded while
iterating an object's properties. Each Foundation type lives in its own module.
*/

mod array;
mod boolean;
mod bytes;
mod class;
mod date;
mod dict;
mod helpers;
mod names;
mod null;
mod number;
mod string;
mod url;

#[cfg(test)]
mod test_support;

pub use crate::deserializer::foundation::array::{FoundationArray, FoundationArrayIter};
pub use crate::deserializer::foundation::dict::{FoundationDict, FoundationDictIter};
