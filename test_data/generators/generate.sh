#!/bin/sh
# Regenerate the Foundation typedstream test fixtures with the classic `NSArchiver`.
#
# macOS only (requires Swift + Foundation). The emitted fixtures live in
# src/test_data/foundation/ and are committed, so the Rust tests run on any
# platform / in CI; only regeneration needs a Mac. `NSArchiver` is deprecated, so
# `swiftc` prints a deprecation warning.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="$repo/src/test_data/foundation"
mkdir -p "$out"

bin="$(mktemp -t foundation_gen)"
trap 'rm -f "$bin"' EXIT
swiftc -o "$bin" "$here/foundation.swift"

keys="NSString NSMutableString NSAttributedString \
NSData NSMutableData \
NumberBool NumberInt NumberFloat NumberDouble NumberInt64 \
NSArray NSMutableArray NSArrayEmpty NSArrayWithNull NSArrayNested \
NSDictionary NSMutableDictionary NSDictionaryNested \
NSSet NSMutableSet \
NSDate NSDateFractional \
NSURL NSURLRelative \
NestedStrings NestedData NestedAttributed NestedContainers"

for k in $keys; do
    "$bin" "$k" "$out/$k" || echo "FAILED: $k"
done

echo "Fixtures written to $out"
