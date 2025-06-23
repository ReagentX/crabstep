#![forbid(unsafe_code)]
// TODO: Enable this once we have documentation for all public items
// #![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod constants;
pub mod deserializer;
pub mod error;
pub mod models;

#[cfg(test)]
mod tests {
    use std::{env::current_dir, fs::File, io::Read};

    use crate::{
        deserializer::{iter::print_resolved, typedstream::TypedStreamDeserializer},
        models::{archivable::Archived, class::Class, output_data::OutputData, types::Type},
    };

    #[test]
    fn test_parse_text_basic() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AttributedBodyTextOnly");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        // Skip the header for now
        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let root = typedstream.oxidize().unwrap();
        println!("\nResults:");
        println!("Root object: {:x?}", typedstream.object_table[root]);
        print_resolved(typedstream.resolve_properties(root).unwrap(), 2);

        println!("\nFound {:?} types:", typedstream.type_table.len());
        typedstream
            .type_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        println!("\nFound {:?} objects:", typedstream.type_table.len());
        typedstream
            .object_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        let expected_types = vec![
            vec![Type::Object],
            vec![Type::String("NSMutableAttributedString")],
            vec![Type::String("NSAttributedString")],
            vec![Type::String("NSObject")],
            vec![Type::String("NSMutableString")],
            vec![Type::String("NSString")],
            vec![Type::Utf8String],
            vec![Type::SignedInt, Type::UnsignedInt],
            vec![Type::String("NSDictionary")],
            vec![Type::SignedInt],
            vec![Type::String("NSNumber")],
            vec![Type::String("NSValue")],
            vec![Type::EmbeddedData],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(10),
                    ],
                    vec![OutputData::Object(7)],
                ],
            },
            Archived::Class(Class {
                name_index: 1,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Class(Class {
                name_index: 2,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Class(Class {
                name_index: 3,
                version: 0,
                parent_index: None,
            }),
            Archived::Object {
                class: 5,
                data: vec![vec![OutputData::String("Noter test")]],
            },
            Archived::Class(Class {
                name_index: 4,
                version: 1,
                parent_index: Some(6),
            }),
            Archived::Class(Class {
                name_index: 5,
                version: 1,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(12),
            }),
            Archived::Class(Class {
                name_index: 11,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(9),
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_basic_2() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AttributedBodyTextOnly2");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        // Skip the header for now
        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let root = typedstream.oxidize().unwrap();
        println!("\nResults:");
        println!("Root object: {:x?}", typedstream.object_table[root]);
        print_resolved(typedstream.resolve_properties(root).unwrap(), 2);

        println!("\nFound {:?} types:", typedstream.type_table.len());
        typedstream
            .type_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        println!("\nFound {:?} objects:", typedstream.type_table.len());
        typedstream
            .object_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        let expected_types = vec![
            vec![Type::Object],
            vec![Type::String("NSAttributedString")],
            vec![Type::String("NSObject")],
            vec![Type::String("NSString")],
            vec![Type::Utf8String],
            vec![Type::SignedInt, Type::UnsignedInt],
            vec![Type::String("NSDictionary")],
            vec![Type::SignedInt],
            vec![Type::String("NSNumber")],
            vec![Type::String("NSValue")],
            vec![Type::EmbeddedData],
            vec![Type::SignedInt],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(3)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(5)],
                ],
            },
            Archived::Class(Class {
                name_index: 1,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Class(Class {
                name_index: 2,
                version: 0,
                parent_index: None,
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("Test 3")]],
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Class(Class {
                name_index: 6,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(10),
            }),
            Archived::Class(Class {
                name_index: 9,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Type(11),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_overlapping_format_url() {
        let typedstream_path = current_dir().unwrap().as_path().join("src/test_data/35123");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        // Skip the header for now
        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let root = typedstream.oxidize().unwrap();
        println!("\nResults:");
        println!("Root object: {:x?}", typedstream.object_table[root]);
        print_resolved(typedstream.resolve_properties(root).unwrap(), 2);

        println!("\nFound {:?} types:", typedstream.type_table.len());
        typedstream
            .type_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        println!("\nFound {:?} objects:", typedstream.type_table.len());
        typedstream
            .object_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        let expected_types = vec![
            vec![Type::Object],
            vec![Type::String("NSMutableAttributedString")],
            vec![Type::String("NSAttributedString")],
            vec![Type::String("NSObject")],
            vec![Type::String("NSMutableString")],
            vec![Type::String("NSString")],
            vec![Type::Utf8String],
            vec![Type::SignedInt, Type::UnsignedInt],
            vec![Type::String("NSDictionary")],
            vec![Type::SignedInt],
            vec![Type::String("NSNumber")],
            vec![Type::String("NSValue")],
            vec![Type::EmbeddedData],
            vec![Type::SignedInt],
            vec![Type::String("NSURL")],
            vec![Type::SignedInt],
            vec![Type::String("NSData")],
            vec![Type::Array(649)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(5)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(2)],
                    vec![OutputData::Object(26)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(3)],
                    vec![OutputData::Object(28)],
                    vec![OutputData::SignedInteger(4), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(31)],
                    vec![OutputData::SignedInteger(5), OutputData::UnsignedInteger(8)],
                    vec![OutputData::Object(33)],
                    vec![
                        OutputData::SignedInteger(6),
                        OutputData::UnsignedInteger(10),
                    ],
                    vec![OutputData::Object(35)],
                    vec![
                        OutputData::SignedInteger(7),
                        OutputData::UnsignedInteger(13),
                    ],
                    vec![OutputData::Object(36)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(2)],
                    vec![OutputData::Object(37)],
                    vec![OutputData::SignedInteger(9), OutputData::UnsignedInteger(4)],
                    vec![OutputData::Object(38)],
                    vec![
                        OutputData::SignedInteger(10),
                        OutputData::UnsignedInteger(5),
                    ],
                    vec![OutputData::Object(41)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(1)],
                    vec![
                        OutputData::SignedInteger(11),
                        OutputData::UnsignedInteger(5),
                    ],
                    vec![OutputData::Object(43)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(1)],
                    vec![
                        OutputData::SignedInteger(12),
                        OutputData::UnsignedInteger(3),
                    ],
                    vec![OutputData::Object(45)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(1)],
                    vec![
                        OutputData::SignedInteger(13),
                        OutputData::UnsignedInteger(7),
                    ],
                    vec![OutputData::Object(47)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(1)],
                    vec![
                        OutputData::SignedInteger(14),
                        OutputData::UnsignedInteger(6),
                    ],
                    vec![OutputData::Object(49)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(1)],
                    vec![
                        OutputData::SignedInteger(15),
                        OutputData::UnsignedInteger(5),
                    ],
                    vec![OutputData::Object(51)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(1)],
                    vec![
                        OutputData::SignedInteger(16),
                        OutputData::UnsignedInteger(6),
                    ],
                    vec![OutputData::Object(53)],
                ],
            },
            Archived::Class(Class {
                name_index: 1,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Class(Class {
                name_index: 2,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Class(Class {
                name_index: 3,
                version: 0,
                parent_index: None,
            }),
            Archived::Object {
                class: 5,
                data: vec![vec![OutputData::String(
                    "0123456789\nBold Italics Underline Strikethrough\u{a0}\u{a0}Big Small Shake Nod Explode Ripple Bloom Jitter",
                )]],
            },
            Archived::Class(Class {
                name_index: 4,
                version: 1,
                parent_index: Some(6),
            }),
            Archived::Class(Class {
                name_index: 5,
                version: 1,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(5)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(24)],
                ],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextUnderlineAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(12),
            }),
            Archived::Class(Class {
                name_index: 11,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(9),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Type(13),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMLinkAttributeName")]],
            },
            Archived::Object {
                class: 19,
                data: vec![
                    vec![OutputData::SignedInteger(0)],
                    vec![OutputData::Object(20)],
                ],
            },
            Archived::Class(Class {
                name_index: 14,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("tel:0123456789")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMPhoneNumberAttributeName")]],
            },
            Archived::Object {
                class: 25,
                data: vec![
                    vec![OutputData::SignedInteger(649)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 10, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 16, 128, 1, 175,
                        16, 17, 13, 14, 28, 36, 37, 38, 44, 45, 46, 51, 57, 61, 62, 63, 66, 69, 73,
                        85, 36, 110, 117, 108, 108, 215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 26, 82, 77, 83, 86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81,
                        84, 81, 80, 82, 83, 82, 82, 86, 78, 128, 6, 128, 15, 128, 2, 128, 7, 16, 1,
                        128, 8, 212, 29, 30, 31, 16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114,
                        97, 110, 103, 101, 118, 97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16,
                        20, 78, 83, 46, 114, 97, 110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97,
                        116, 105, 111, 110, 90, 78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128,
                        3, 128, 4, 16, 4, 128, 5, 16, 10, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99,
                        108, 97, 115, 115, 110, 97, 109, 101, 88, 36, 99, 108, 97, 115, 115, 101,
                        115, 87, 78, 83, 86, 97, 108, 117, 101, 162, 41, 43, 88, 78, 83, 79, 98,
                        106, 101, 99, 116, 90, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 91, 80, 104,
                        111, 110, 101, 78, 117, 109, 98, 101, 114, 210, 47, 16, 48, 50, 90, 78, 83,
                        46, 111, 98, 106, 101, 99, 116, 115, 161, 49, 128, 9, 128, 14, 215, 15, 16,
                        17, 18, 19, 20, 21, 52, 23, 54, 55, 26, 56, 26, 128, 11, 128, 15, 128, 10,
                        128, 12, 128, 13, 212, 29, 30, 31, 16, 32, 33, 34, 35, 128, 3, 128, 4, 128,
                        5, 90, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 85, 86, 97, 108, 117, 101,
                        210, 47, 16, 64, 50, 160, 128, 14, 210, 39, 40, 67, 68, 87, 78, 83, 65,
                        114, 114, 97, 121, 162, 67, 43, 210, 39, 40, 70, 71, 95, 16, 15, 68, 68,
                        83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108, 116, 162, 72, 43,
                        95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108,
                        116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41, 0, 50, 0, 55, 0, 73, 0, 78,
                        0, 86, 0, 96, 0, 98, 0, 100, 0, 120, 0, 126, 0, 141, 0, 144, 0, 151, 0,
                        154, 0, 156, 0, 158, 0, 161, 0, 164, 0, 166, 0, 168, 0, 170, 0, 172, 0,
                        174, 0, 176, 0, 185, 0, 206, 0, 229, 0, 240, 0, 242, 0, 244, 0, 246, 0,
                        248, 0, 250, 0, 252, 1, 1, 1, 12, 1, 21, 1, 29, 1, 32, 1, 41, 1, 52, 1, 64,
                        1, 69, 1, 80, 1, 82, 1, 84, 1, 86, 1, 101, 1, 103, 1, 105, 1, 107, 1, 109,
                        1, 111, 1, 120, 1, 122, 1, 124, 1, 126, 1, 137, 1, 143, 1, 148, 1, 149, 1,
                        151, 1, 156, 1, 164, 1, 167, 1, 172, 1, 190, 1, 193, 1, 211, 0, 0, 0, 0, 0,
                        0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        1, 213,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 16,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(6)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(24)],
                    vec![OutputData::Object(27)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMTextStrikethroughAttributeName",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(5)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(27)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(29)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(24)],
                ],
            },
            Archived::Object {
                class: 19,
                data: vec![
                    vec![OutputData::SignedInteger(0)],
                    vec![OutputData::Object(30)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("tel:0123456789")]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(32)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextBoldAttributeName")]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(32)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(34)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextItalicAttributeName")]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(5)],
                    vec![OutputData::Object(32)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(34)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(6)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(34)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(32)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(27)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                ],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(40)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextEffectAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(5)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(42)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(11)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(44)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(9)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(46)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(8)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(48)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(12)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(50)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(4)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(52)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(6)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(54)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(10)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(
            typedstream.object_table[..expected_objects.len()],
            expected_objects
        );
    }

    #[test]
    fn test_parse_text_overlapping_url_short() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/0123456789");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        // Skip the header for now
        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let root = typedstream.oxidize().unwrap();
        println!("\nResults:");
        println!("Root object: {:x?}", typedstream.object_table[root]);
        print_resolved(typedstream.resolve_properties(root).unwrap(), 2);

        println!("\nFound {:?} types:", typedstream.type_table.len());
        typedstream
            .type_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        println!("\nFound {:?} objects:", typedstream.type_table.len());
        typedstream
            .object_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        let expected_types = vec![
            vec![Type::Object],
            vec![Type::String("NSAttributedString")],
            vec![Type::String("NSObject")],
            vec![Type::String("NSString")],
            vec![Type::Utf8String],
            vec![Type::SignedInt, Type::UnsignedInt],
            vec![Type::String("NSDictionary")],
            vec![Type::SignedInt],
            vec![Type::String("NSMutableData")],
            vec![Type::String("NSData")],
            vec![Type::Array(635)],
            vec![Type::String("NSNumber")],
            vec![Type::String("NSValue")],
            vec![Type::EmbeddedData],
            vec![Type::String("NSURL")],
            vec![Type::SignedInt],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(3)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(5)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(4)],
                    vec![OutputData::Object(22)],
                ],
            },
            Archived::Class(Class {
                name_index: 1,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Class(Class {
                name_index: 2,
                version: 0,
                parent_index: None,
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("0123456789")]],
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(21)],
                ],
            },
            Archived::Class(Class {
                name_index: 6,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMPhoneNumberAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![
                    vec![OutputData::SignedInteger(635)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 10, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 15, 128, 1, 175,
                        16, 16, 13, 14, 28, 36, 37, 38, 44, 45, 46, 51, 57, 61, 62, 65, 68, 72, 85,
                        36, 110, 117, 108, 108, 215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                        26, 27, 26, 82, 77, 83, 86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81, 84,
                        81, 80, 82, 83, 82, 82, 86, 78, 128, 6, 128, 14, 128, 2, 128, 7, 16, 1,
                        128, 8, 212, 29, 30, 31, 16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114,
                        97, 110, 103, 101, 118, 97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16,
                        20, 78, 83, 46, 114, 97, 110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97,
                        116, 105, 111, 110, 90, 78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128,
                        3, 128, 4, 16, 4, 128, 5, 16, 10, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99,
                        108, 97, 115, 115, 110, 97, 109, 101, 88, 36, 99, 108, 97, 115, 115, 101,
                        115, 87, 78, 83, 86, 97, 108, 117, 101, 162, 41, 43, 88, 78, 83, 79, 98,
                        106, 101, 99, 116, 90, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 91, 80, 104,
                        111, 110, 101, 78, 117, 109, 98, 101, 114, 210, 47, 16, 48, 50, 90, 78, 83,
                        46, 111, 98, 106, 101, 99, 116, 115, 161, 49, 128, 9, 128, 13, 215, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 54, 55, 26, 56, 26, 128, 6, 128, 14, 128, 10,
                        128, 11, 128, 12, 212, 29, 30, 31, 16, 32, 33, 34, 35, 128, 3, 128, 4, 128,
                        5, 85, 86, 97, 108, 117, 101, 210, 47, 16, 63, 50, 160, 128, 13, 210, 39,
                        40, 66, 67, 87, 78, 83, 65, 114, 114, 97, 121, 162, 66, 43, 210, 39, 40,
                        69, 70, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115,
                        117, 108, 116, 162, 71, 43, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101,
                        114, 82, 101, 115, 117, 108, 116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41,
                        0, 50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100, 0, 119, 0, 125, 0,
                        140, 0, 143, 0, 150, 0, 153, 0, 155, 0, 157, 0, 160, 0, 163, 0, 165, 0,
                        167, 0, 169, 0, 171, 0, 173, 0, 175, 0, 184, 0, 205, 0, 228, 0, 239, 0,
                        241, 0, 243, 0, 245, 0, 247, 0, 249, 0, 251, 1, 0, 1, 11, 1, 20, 1, 28, 1,
                        31, 1, 40, 1, 51, 1, 63, 1, 68, 1, 79, 1, 81, 1, 83, 1, 85, 1, 100, 1, 102,
                        1, 104, 1, 106, 1, 108, 1, 110, 1, 119, 1, 121, 1, 123, 1, 125, 1, 131, 1,
                        136, 1, 137, 1, 139, 1, 144, 1, 152, 1, 155, 1, 160, 1, 178, 1, 181, 1,
                        199, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 1, 201,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(10),
            }),
            Archived::Class(Class {
                name_index: 9,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Class(Class {
                name_index: 11,
                version: 0,
                parent_index: Some(14),
            }),
            Archived::Class(Class {
                name_index: 12,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Type(7),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMLinkAttributeName")]],
            },
            Archived::Object {
                class: 18,
                data: vec![
                    vec![OutputData::SignedInteger(0)],
                    vec![OutputData::Object(19)],
                ],
            },
            Archived::Class(Class {
                name_index: 14,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("tel:0123456789")]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "__kIMTextStrikethroughAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(21)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextItalicAttributeName")]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_apple_music_lyrics() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AppleMusicLyrics");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        // Skip the header for now
        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let root = typedstream.oxidize().unwrap();
        println!("\nResults:");
        println!("Root object: {:x?}", typedstream.object_table[root]);
        print_resolved(typedstream.resolve_properties(root).unwrap(), 2);

        println!("\nFound {:?} types:", typedstream.type_table.len());
        typedstream
            .type_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        println!("\nFound {:?} objects:", typedstream.type_table.len());
        typedstream
            .object_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:?}"));

        let expected_types = vec![
            vec![Type::Object],
            vec![Type::String("NSMutableAttributedString")],
            vec![Type::String("NSAttributedString")],
            vec![Type::String("NSObject")],
            vec![Type::String("NSMutableString")],
            vec![Type::String("NSString")],
            vec![Type::Utf8String],
            vec![Type::SignedInt, Type::UnsignedInt],
            vec![Type::String("NSDictionary")],
            vec![Type::SignedInt],
            vec![Type::String("NSNumber")],
            vec![Type::String("NSValue")],
            vec![Type::EmbeddedData],
            vec![Type::SignedInt],
            vec![Type::SignedInt],
            vec![Type::String("NSURL")],
            vec![Type::String("NSData")],
            vec![Type::Array(675)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(145),
                    ],
                    vec![OutputData::Object(7)],
                ],
            },
            Archived::Class(Class {
                name_index: 1,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Class(Class {
                name_index: 2,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Class(Class {
                name_index: 3,
                version: 0,
                parent_index: None,
            }),
            Archived::Object {
                class: 5,
                data: vec![vec![OutputData::String(
                    "https://music.apple.com/us/lyrics/1329891623?ts=11.108&te=16.031&l=en&tk=2.v1.VsuX9f%2BaT1PyrgMgIT7ANQ%3D%3D&itsct=sharing_msg_lyrics&itscg=50401",
                )]],
            },
            Archived::Class(Class {
                name_index: 4,
                version: 1,
                parent_index: Some(6),
            }),
            Archived::Class(Class {
                name_index: 5,
                version: 1,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(5)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(24)],
                ],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(12),
            }),
            Archived::Class(Class {
                name_index: 11,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(13),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMLinkIsRichLinkAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Type(14),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMLinkAttributeName")]],
            },
            Archived::Object {
                class: 19,
                data: vec![
                    vec![OutputData::SignedInteger(0)],
                    vec![OutputData::Object(20)],
                ],
            },
            Archived::Class(Class {
                name_index: 15,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "https://music.apple.com/us/lyrics/1329891623?ts=11.108&te=16.031&l=en&tk=2.v1.VsuX9f%2BaT1PyrgMgIT7ANQ%3D%3D&itsct=sharing_msg_lyrics&itscg=50401",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMDataDetectedAttributeName")]],
            },
            Archived::Object {
                class: 25,
                data: vec![
                    vec![OutputData::SignedInteger(675)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 10, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 11, 128, 1, 172,
                        13, 14, 28, 36, 37, 38, 44, 45, 46, 50, 53, 57, 85, 36, 110, 117, 108, 108,
                        215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 26, 82, 77, 83,
                        86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81, 84, 81, 80, 82, 83, 82, 82,
                        86, 78, 128, 6, 128, 10, 128, 2, 128, 7, 16, 1, 128, 8, 212, 29, 30, 31,
                        16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114, 97, 110, 103, 101, 118,
                        97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16, 20, 78, 83, 46, 114, 97,
                        110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97, 116, 105, 111, 110, 90,
                        78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128, 3, 128, 4, 16, 4, 128, 5,
                        16, 145, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99, 108, 97, 115, 115, 110,
                        97, 109, 101, 88, 36, 99, 108, 97, 115, 115, 101, 115, 87, 78, 83, 86, 97,
                        108, 117, 101, 162, 41, 43, 88, 78, 83, 79, 98, 106, 101, 99, 116, 95, 16,
                        145, 104, 116, 116, 112, 115, 58, 47, 47, 109, 117, 115, 105, 99, 46, 97,
                        112, 112, 108, 101, 46, 99, 111, 109, 47, 117, 115, 47, 108, 121, 114, 105,
                        99, 115, 47, 49, 51, 50, 57, 56, 57, 49, 54, 50, 51, 63, 116, 115, 61, 49,
                        49, 46, 49, 48, 56, 38, 116, 101, 61, 49, 54, 46, 48, 51, 49, 38, 108, 61,
                        101, 110, 38, 116, 107, 61, 50, 46, 118, 49, 46, 86, 115, 117, 88, 57, 102,
                        37, 50, 66, 97, 84, 49, 80, 121, 114, 103, 77, 103, 73, 84, 55, 65, 78, 81,
                        37, 51, 68, 37, 51, 68, 38, 105, 116, 115, 99, 116, 61, 115, 104, 97, 114,
                        105, 110, 103, 95, 109, 115, 103, 95, 108, 121, 114, 105, 99, 115, 38, 105,
                        116, 115, 99, 103, 61, 53, 48, 52, 48, 49, 87, 72, 116, 116, 112, 85, 82,
                        76, 210, 47, 16, 48, 49, 90, 78, 83, 46, 111, 98, 106, 101, 99, 116, 115,
                        160, 128, 9, 210, 39, 40, 51, 52, 87, 78, 83, 65, 114, 114, 97, 121, 162,
                        51, 43, 210, 39, 40, 54, 55, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101,
                        114, 82, 101, 115, 117, 108, 116, 162, 56, 43, 95, 16, 15, 68, 68, 83, 99,
                        97, 110, 110, 101, 114, 82, 101, 115, 117, 108, 116, 16, 1, 0, 8, 0, 17, 0,
                        26, 0, 36, 0, 41, 0, 50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100,
                        0, 113, 0, 119, 0, 134, 0, 137, 0, 144, 0, 147, 0, 149, 0, 151, 0, 154, 0,
                        157, 0, 159, 0, 161, 0, 163, 0, 165, 0, 167, 0, 169, 0, 178, 0, 199, 0,
                        222, 0, 233, 0, 235, 0, 237, 0, 239, 0, 241, 0, 243, 0, 245, 0, 250, 1, 5,
                        1, 14, 1, 22, 1, 25, 1, 34, 1, 182, 1, 190, 1, 195, 1, 206, 1, 207, 1, 209,
                        1, 214, 1, 222, 1, 225, 1, 230, 1, 248, 1, 251, 2, 13, 0, 0, 0, 0, 0, 0, 2,
                        1, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                        15,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 16,
                version: 0,
                parent_index: Some(3),
            }),
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }
}
