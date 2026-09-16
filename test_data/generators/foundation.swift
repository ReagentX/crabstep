// Generates the Foundation typedstream test fixtures in src/test_data/foundation/
// using the *classic* NSArchiver (the typedstream archiver). Run via generate.sh.
//
// One object is archived per process invocation: a class that refuses non-keyed
// archiving raises an NSException that aborts only this run (caught by the script
// as a nonzero exit), instead of taking down the whole batch.
//
// NSArchiver is deprecated but still emits `streamtyped` on current macOS. Only
// Tier-1 Foundation classes are generated here (see FOUNDATION_FEATURE_SCOPING.md);
// AppKit/Tier-2 classes are intentionally excluded.

import Foundation

func make(_ key: String) -> Any? {
    switch key {
    // string cluster
    case "NSString":            return NSString(string: "Hello, world")
    case "NSMutableString":     return NSMutableString(string: "Hello, world")
    case "NSAttributedString":  return NSAttributedString(string: "Hello, world")
    // data cluster
    case "NSData":              return NSData(bytes: [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x7F] as [UInt8], length: 6)
    case "NSMutableData":       return NSMutableData(bytes: [0xDE, 0xAD, 0xBE, 0xEF] as [UInt8], length: 4)
    // numbers (exactly-representable values so tests can assert equality)
    case "NumberBool":          return NSNumber(value: true)
    case "NumberInt":           return NSNumber(value: Int32(42))
    case "NumberFloat":         return NSNumber(value: Float(3.5))
    case "NumberDouble":        return NSNumber(value: Double(100.5))
    case "NumberInt64":         return NSNumber(value: Int64(-9_000_000_000)) // exercises the 8-byte form
    // arrays
    case "NSArray":             return NSArray(array: [NSString(string: "a"), NSNumber(value: 1), NSString(string: "b")])
    case "NSMutableArray":      return NSMutableArray(array: [NSString(string: "x"), NSNumber(value: 2)])
    case "NSArrayEmpty":        return NSArray(array: [])
    case "NSArrayWithNull":     return NSArray(array: [NSString(string: "x"), NSNull(), NSString(string: "y")])
    case "NSArrayNested":       return NSArray(array: [NSString(string: "top"), NSArray(array: [NSNumber(value: 1), NSNumber(value: 2)])])
    // dictionaries
    case "NSDictionary":        return NSDictionary(objects: [NSNumber(value: 1), NSString(string: "v")], forKeys: [NSString(string: "k1"), NSString(string: "k2")])
    case "NSMutableDictionary":
        let d = NSMutableDictionary()
        d.setObject(NSNumber(value: 9), forKey: NSString(string: "key"))
        return d
    case "NSDictionaryNested":
        return NSDictionary(
            objects: [
                NSArray(array: [NSNumber(value: 1), NSNumber(value: 2)]),
                NSData(bytes: [0x01, 0x02] as [UInt8], length: 2),
                NSNumber(value: 7),
            ],
            forKeys: [NSString(string: "arr"), NSString(string: "data"), NSString(string: "num")]
        )
    // sets
    case "NSSet":               return NSSet(array: [NSString(string: "s1"), NSString(string: "s2")])
    case "NSMutableSet":        return NSMutableSet(array: [NSString(string: "m1")])
    // dates (integral dodges the DECIMAL path; fractional exercises it)
    case "NSDate":              return NSDate(timeIntervalSinceReferenceDate: 21692800)        // unix 1_000_000_000
    case "NSDateFractional":    return NSDate(timeIntervalSinceReferenceDate: 700000000.523)
    // urls
    case "NSURL":               return NSURL(string: "https://example.com/path?q=1")
    case "NSURLRelative":       return NSURL(string: "page.html", relativeTo: URL(string: "https://example.com/dir/"))
    // nested fixtures: accessors operate on objects wrapped in a group, so these
    // place each cluster variant as an array element (a root object's groups are
    // its contents, not a wrapper around it).
    case "NestedStrings":       return NSArray(array: [NSString(string: "imm"), NSMutableString(string: "mut")])
    case "NestedData":          return NSArray(array: [NSData(bytes: [0x01, 0x02] as [UInt8], length: 2), NSMutableData(bytes: [0x03, 0x04, 0x05] as [UInt8], length: 3)])
    case "NestedAttributed":    return NSArray(array: [NSAttributedString(string: "styled")])
    case "NestedContainers":
        // Nested containers (both cluster variants + an empty array) so the
        // container accessors can be tested on objects wrapped in a group.
        let md = NSMutableDictionary()
        md.setObject(NSNumber(value: 8), forKey: NSString(string: "mk"))
        return NSArray(array: [
            NSArray(array: [NSNumber(value: 1), NSNumber(value: 2)]),
            NSMutableArray(array: [NSNumber(value: 3)]),
            NSArray(array: []),
            NSDictionary(objects: [NSNumber(value: 9)], forKeys: [NSString(string: "k")]),
            md,
            NSSet(array: [NSString(string: "s")]),
            NSMutableSet(array: [NSString(string: "ms")]),
        ])
    case "NestedScalars":
        // NSDate / NSURL (absolute + relative) / NSNull as array elements, so the
        // Phase 4 accessors can be tested on objects wrapped in a group.
        return NSArray(array: [
            NSDate(timeIntervalSinceReferenceDate: 21692800), // unix 1_000_000_000
            NSURL(string: "https://example.com/path?q=1")!,
            NSURL(string: "page.html", relativeTo: URL(string: "https://example.com/dir/"))!,
            NSNull(),
        ])
    default:                    return nil
    }
}

let args = CommandLine.arguments
guard args.count >= 3, let obj = make(args[1]) else {
    FileHandle.standardError.write("skip \(args.count >= 2 ? args[1] : "?")\n".data(using: .utf8)!)
    exit(2)
}
let data = NSArchiver.archivedData(withRootObject: obj)
try! data.write(to: URL(fileURLWithPath: args[2]))
print("ok \(args[1]) -> \(data.count) bytes")
