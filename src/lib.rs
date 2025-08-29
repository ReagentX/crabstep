#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![no_std]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod deserializer;
pub mod error;
pub mod models;

pub use deserializer::{iter::PropertyIterator, typedstream::TypedStreamDeserializer};
pub use models::{archived::Archived, output_data::OutputData};

#[cfg(test)]
mod test_typedstream_deserializer {
    extern crate std;
    use alloc::vec;
    use std::{env::current_dir, fs::File, io::Read, println};

    use crate::{
        deserializer::{iter::print_resolved, typedstream::TypedStreamDeserializer},
        models::{archived::Archived, class::Class, output_data::OutputData, types::Type},
    };

    #[test]
    fn test_parse_text_iter() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AttributedBodyTextOnly");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let root = typedstream.oxidize().unwrap();
        let root_obj = typedstream.resolve_properties(root).unwrap();
        let primitives = root_obj.primitives();

        assert_eq!(
            primitives,
            vec![
                &OutputData::String("Noter test"),
                &OutputData::UnsignedInteger(10),
                &OutputData::SignedInteger(1),
                &OutputData::SignedInteger(0),
                &OutputData::String("__kIMMessagePartAttributeName"),
                &OutputData::SignedInteger(1)
            ]
        );
    }

    #[test]
    fn test_parse_text_basic() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AttributedBodyTextOnly");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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

    #[test]
    fn test_parse_text_app_message() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AppMessage");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
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
                data: vec![vec![OutputData::String("￼")]], // U+FFFC "OBJECT REPLACEMENT CHARACTER"
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
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
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "F0B18A15-E9A5-4B18-A38F-685B7B3FF037",
                )]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(12),
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
                class: 11,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_array() {
        let typedstream_path = current_dir().unwrap().as_path().join("src/test_data/Array");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::String("NSData")],
            vec![Type::Array(904)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(32),
                    ],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(16),
                    ],
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
                    "A single ChatGPT instance takes 5MW of power to run",
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
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMDataDetectedAttributeName")]],
            },
            Archived::Object {
                class: 17,
                data: vec![
                    vec![OutputData::SignedInteger(904)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 10, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 31, 128, 1, 175,
                        16, 32, 13, 14, 28, 36, 37, 38, 44, 45, 46, 53, 59, 26, 63, 64, 65, 68, 72,
                        76, 84, 88, 89, 90, 91, 94, 95, 102, 106, 107, 108, 109, 112, 113, 85, 36,
                        110, 117, 108, 108, 215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                        27, 26, 82, 77, 83, 86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81, 84, 81,
                        80, 82, 83, 82, 82, 86, 78, 128, 6, 128, 16, 128, 2, 128, 7, 16, 1, 128, 8,
                        212, 29, 30, 31, 16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114, 97, 110,
                        103, 101, 118, 97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16, 20, 78,
                        83, 46, 114, 97, 110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97, 116,
                        105, 111, 110, 90, 78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128, 3,
                        128, 4, 16, 4, 128, 5, 16, 3, 16, 32, 210, 39, 40, 41, 42, 90, 36, 99, 108,
                        97, 115, 115, 110, 97, 109, 101, 88, 36, 99, 108, 97, 115, 115, 101, 115,
                        87, 78, 83, 86, 97, 108, 117, 101, 162, 41, 43, 88, 78, 83, 79, 98, 106,
                        101, 99, 116, 83, 53, 77, 87, 94, 80, 104, 121, 115, 105, 99, 97, 108, 65,
                        109, 111, 117, 110, 116, 210, 47, 16, 48, 52, 90, 78, 83, 46, 111, 98, 106,
                        101, 99, 116, 115, 163, 49, 50, 51, 128, 9, 128, 17, 128, 24, 128, 15, 215,
                        15, 16, 17, 18, 19, 20, 21, 54, 23, 56, 57, 26, 58, 26, 128, 12, 128, 16,
                        128, 10, 128, 13, 128, 14, 212, 29, 30, 31, 16, 60, 33, 34, 35, 128, 11,
                        128, 4, 128, 5, 81, 53, 93, 73, 110, 116, 101, 103, 114, 97, 108, 86, 97,
                        108, 117, 101, 210, 47, 16, 66, 52, 160, 128, 15, 210, 39, 40, 69, 70, 94,
                        78, 83, 77, 117, 116, 97, 98, 108, 101, 65, 114, 114, 97, 121, 163, 69, 71,
                        43, 87, 78, 83, 65, 114, 114, 97, 121, 210, 39, 40, 73, 74, 95, 16, 15, 68,
                        68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108, 116, 162, 75,
                        43, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117,
                        108, 116, 216, 15, 16, 77, 17, 18, 19, 20, 21, 78, 23, 80, 81, 82, 26, 83,
                        26, 81, 86, 128, 20, 128, 16, 128, 23, 128, 18, 128, 21, 128, 22, 212, 29,
                        30, 31, 16, 60, 86, 34, 35, 128, 11, 128, 19, 128, 5, 16, 33, 81, 77, 90,
                        77, 117, 108, 116, 105, 112, 108, 105, 101, 114, 210, 47, 16, 92, 52, 160,
                        128, 15, 87, 49, 48, 48, 48, 48, 48, 48, 216, 15, 16, 77, 17, 18, 19, 20,
                        21, 96, 23, 98, 99, 100, 26, 101, 26, 128, 27, 128, 16, 128, 30, 128, 25,
                        128, 28, 128, 29, 212, 29, 30, 31, 16, 60, 104, 34, 35, 128, 11, 128, 26,
                        128, 5, 16, 34, 81, 87, 84, 85, 110, 105, 116, 210, 47, 16, 110, 52, 160,
                        128, 15, 84, 119, 97, 116, 116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41, 0,
                        50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100, 0, 135, 0, 141, 0,
                        156, 0, 159, 0, 166, 0, 169, 0, 171, 0, 173, 0, 176, 0, 179, 0, 181, 0,
                        183, 0, 185, 0, 187, 0, 189, 0, 191, 0, 200, 0, 221, 0, 244, 0, 255, 1, 1,
                        1, 3, 1, 5, 1, 7, 1, 9, 1, 11, 1, 16, 1, 27, 1, 36, 1, 44, 1, 47, 1, 56, 1,
                        60, 1, 75, 1, 80, 1, 91, 1, 95, 1, 97, 1, 99, 1, 101, 1, 103, 1, 118, 1,
                        120, 1, 122, 1, 124, 1, 126, 1, 128, 1, 137, 1, 139, 1, 141, 1, 143, 1,
                        145, 1, 159, 1, 164, 1, 165, 1, 167, 1, 172, 1, 187, 1, 191, 1, 199, 1,
                        204, 1, 222, 1, 225, 1, 243, 2, 4, 2, 6, 2, 8, 2, 10, 2, 12, 2, 14, 2, 16,
                        2, 18, 2, 27, 2, 29, 2, 31, 2, 33, 2, 35, 2, 37, 2, 48, 2, 53, 2, 54, 2,
                        56, 2, 64, 2, 81, 2, 83, 2, 85, 2, 87, 2, 89, 2, 91, 2, 93, 2, 102, 2, 104,
                        2, 106, 2, 108, 2, 110, 2, 112, 2, 117, 2, 122, 2, 123, 2, 125, 2, 130, 0,
                        0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 2, 132,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 13,
                version: 0,
                parent_index: Some(3),
            }),
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_attachment() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/Attachment");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::Double],
            vec![Type::SignedInt],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(7)],
                    vec![
                        OutputData::SignedInteger(2),
                        OutputData::UnsignedInteger(77),
                    ],
                    vec![OutputData::Object(26)],
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
                    "￼This is how the notes look to me fyi, in case it helps make sense of anything",
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
                    vec![OutputData::SignedInteger(6)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(24)],
                    vec![OutputData::Object(25)],
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
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_0_2E5F12C3-E649-48AA-954D-3EA67C016BCC",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMInlineMediaHeightAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::Double(1139.0)]],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(14),
            }),
            Archived::Class(Class {
                name_index: 11,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(13),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Type(14),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Type(9),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMFilenameAttributeName")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("Messages Image(785748029).png")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMInlineMediaWidthAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::Double(952.0)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(27)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(28)],
                    vec![OutputData::Object(29)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_attachment_i16() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/AttachmentI16");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
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
                data: vec![vec![OutputData::String("￼")]],
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
                    vec![OutputData::SignedInteger(6)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(23)],
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
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_0_BE588799-C4BC-47DF-A56D-7EE90C74911D",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMInlineMediaHeightAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(600)]],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(14),
            }),
            Archived::Class(Class {
                name_index: 11,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(13),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMFilenameAttributeName")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "brilliant-kids-test-answers-32-93042.jpeg",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMInlineMediaWidthAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(660)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_blank() {
        let typedstream_path = current_dir().unwrap().as_path().join("src/test_data/Blank");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![vec![OutputData::Object(3)]],
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
                data: vec![vec![OutputData::String("")]],
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_code() {
        let typedstream_path = current_dir().unwrap().as_path().join("src/test_data/Code");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::String("NSData")],
            vec![Type::Array(535)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(7)],
                    vec![
                        OutputData::SignedInteger(2),
                        OutputData::UnsignedInteger(46),
                    ],
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
                    "000123 is your security code. Don't share your code.",
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
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
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
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMOneTimeCodeAttributeName")]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(17)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("displayCode")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("000123")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("code")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMDataDetectedAttributeName")]],
            },
            Archived::Object {
                class: 21,
                data: vec![
                    vec![OutputData::SignedInteger(535)],
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
                        16, 6, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99, 108, 97, 115, 115, 110, 97,
                        109, 101, 88, 36, 99, 108, 97, 115, 115, 101, 115, 87, 78, 83, 86, 97, 108,
                        117, 101, 162, 41, 43, 88, 78, 83, 79, 98, 106, 101, 99, 116, 86, 48, 48,
                        48, 49, 50, 51, 88, 65, 117, 116, 104, 67, 111, 100, 101, 210, 47, 16, 48,
                        49, 90, 78, 83, 46, 111, 98, 106, 101, 99, 116, 115, 160, 128, 9, 210, 39,
                        40, 51, 52, 87, 78, 83, 65, 114, 114, 97, 121, 162, 51, 43, 210, 39, 40,
                        54, 55, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115,
                        117, 108, 116, 162, 56, 43, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101,
                        114, 82, 101, 115, 117, 108, 116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41,
                        0, 50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100, 0, 113, 0, 119, 0,
                        134, 0, 137, 0, 144, 0, 147, 0, 149, 0, 151, 0, 154, 0, 157, 0, 159, 0,
                        161, 0, 163, 0, 165, 0, 167, 0, 169, 0, 178, 0, 199, 0, 222, 0, 233, 0,
                        235, 0, 237, 0, 239, 0, 241, 0, 243, 0, 245, 0, 250, 1, 5, 1, 14, 1, 22, 1,
                        25, 1, 34, 1, 41, 1, 50, 1, 55, 1, 66, 1, 67, 1, 69, 1, 74, 1, 82, 1, 85,
                        1, 90, 1, 108, 1, 111, 1, 129, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0,
                        58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 131,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 13,
                version: 0,
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
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_custom_tapback() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/CustomTapback");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(75),
                    ],
                    vec![OutputData::Object(5)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(14)],
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
                data: vec![vec![OutputData::String(
                    "Reacted with a sticker to “Like I wonder if the stickers can be reactions ”￼",
                )]],
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
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(17)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "41C4376E-397E-4C42-84E2-B16F7801F638",
                )]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_date() {
        let typedstream_path = current_dir().unwrap().as_path().join("src/test_data/Date");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::String("NSMutableData")],
            vec![Type::String("NSData")],
            vec![Type::Array(669)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(17),
                    ],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(8)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
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
                data: vec![vec![OutputData::String("Hi. Right now or tomorrow?")]],
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
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMCalendarEventAttributeName")]],
            },
            Archived::Object {
                class: 17,
                data: vec![
                    vec![OutputData::SignedInteger(669)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 10, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 16, 128, 1, 175,
                        16, 17, 13, 14, 29, 37, 38, 39, 45, 46, 47, 52, 60, 64, 65, 68, 72, 73, 77,
                        85, 36, 110, 117, 108, 108, 215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 28, 82, 77, 83, 86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81,
                        84, 81, 80, 82, 83, 82, 82, 86, 78, 128, 6, 128, 15, 128, 2, 128, 7, 16, 0,
                        128, 8, 16, 1, 212, 30, 31, 32, 16, 33, 34, 35, 36, 95, 16, 18, 78, 83, 46,
                        114, 97, 110, 103, 101, 118, 97, 108, 46, 108, 101, 110, 103, 116, 104, 95,
                        16, 20, 78, 83, 46, 114, 97, 110, 103, 101, 118, 97, 108, 46, 108, 111, 99,
                        97, 116, 105, 111, 110, 90, 78, 83, 46, 115, 112, 101, 99, 105, 97, 108,
                        128, 3, 128, 4, 16, 4, 128, 5, 16, 8, 16, 17, 210, 40, 41, 42, 43, 90, 36,
                        99, 108, 97, 115, 115, 110, 97, 109, 101, 88, 36, 99, 108, 97, 115, 115,
                        101, 115, 87, 78, 83, 86, 97, 108, 117, 101, 162, 42, 44, 88, 78, 83, 79,
                        98, 106, 101, 99, 116, 88, 116, 111, 109, 111, 114, 114, 111, 119, 84, 68,
                        97, 116, 101, 210, 48, 16, 49, 51, 90, 78, 83, 46, 111, 98, 106, 101, 99,
                        116, 115, 161, 50, 128, 9, 128, 13, 216, 15, 16, 53, 17, 18, 19, 20, 21,
                        22, 23, 56, 57, 58, 28, 59, 28, 81, 86, 128, 6, 128, 15, 128, 14, 128, 10,
                        128, 11, 128, 12, 212, 30, 31, 32, 16, 33, 34, 35, 36, 128, 3, 128, 4, 128,
                        5, 91, 82, 101, 108, 97, 116, 105, 118, 101, 68, 97, 121, 210, 48, 16, 66,
                        51, 160, 128, 13, 210, 40, 41, 69, 70, 94, 78, 83, 77, 117, 116, 97, 98,
                        108, 101, 65, 114, 114, 97, 121, 163, 69, 71, 44, 87, 78, 83, 65, 114, 114,
                        97, 121, 81, 49, 210, 40, 41, 74, 75, 95, 16, 15, 68, 68, 83, 99, 97, 110,
                        110, 101, 114, 82, 101, 115, 117, 108, 116, 162, 76, 44, 95, 16, 15, 68,
                        68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108, 116, 16, 1, 0,
                        8, 0, 17, 0, 26, 0, 36, 0, 41, 0, 50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0,
                        98, 0, 100, 0, 120, 0, 126, 0, 141, 0, 144, 0, 151, 0, 154, 0, 156, 0, 158,
                        0, 161, 0, 164, 0, 166, 0, 168, 0, 170, 0, 172, 0, 174, 0, 176, 0, 178, 0,
                        187, 0, 208, 0, 231, 0, 242, 0, 244, 0, 246, 0, 248, 0, 250, 0, 252, 0,
                        254, 1, 3, 1, 14, 1, 23, 1, 31, 1, 34, 1, 43, 1, 52, 1, 57, 1, 62, 1, 73,
                        1, 75, 1, 77, 1, 79, 1, 96, 1, 98, 1, 100, 1, 102, 1, 104, 1, 106, 1, 108,
                        1, 110, 1, 119, 1, 121, 1, 123, 1, 125, 1, 137, 1, 142, 1, 143, 1, 145, 1,
                        150, 1, 165, 1, 169, 1, 177, 1, 179, 1, 184, 1, 202, 1, 205, 1, 223, 0, 0,
                        0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 1, 225,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 13,
                version: 0,
                parent_index: Some(18),
            }),
            Archived::Class(Class {
                name_index: 14,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_edited_with_formatting() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/EditedWithFormatting");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(3)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(4)],
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
                data: vec![vec![OutputData::String("Test")]],
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
                    "__kIMTextStrikethroughAttributeName",
                )]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(1)]],
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
            Archived::Type(7),
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
    fn test_parse_text_email() {
        let typedstream_path = current_dir().unwrap().as_path().join("src/test_data/Email");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::Array(667)],
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
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(20),
                    ],
                    vec![OutputData::Object(5)],
                    vec![
                        OutputData::SignedInteger(2),
                        OutputData::UnsignedInteger(11),
                    ],
                    vec![OutputData::Object(20)],
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
                data: vec![vec![OutputData::String("asdfghjklq@gmail.com might work")]],
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                ],
            },
            Archived::Class(Class {
                name_index: 6,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMDataDetectedAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![
                    vec![OutputData::SignedInteger(667)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 13, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 16, 128, 1, 175,
                        16, 17, 13, 14, 28, 36, 37, 38, 44, 45, 46, 51, 57, 61, 62, 63, 66, 69, 73,
                        85, 36, 110, 117, 108, 108, 215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 26, 82, 77, 83, 86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81,
                        84, 81, 80, 82, 83, 82, 82, 86, 78, 128, 6, 128, 15, 128, 2, 128, 7, 16, 1,
                        128, 8, 212, 29, 30, 31, 16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114,
                        97, 110, 103, 101, 118, 97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16,
                        20, 78, 83, 46, 114, 97, 110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97,
                        116, 105, 111, 110, 90, 78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128,
                        3, 128, 4, 16, 4, 128, 5, 16, 20, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99,
                        108, 97, 115, 115, 110, 97, 109, 101, 88, 36, 99, 108, 97, 115, 115, 101,
                        115, 87, 78, 83, 86, 97, 108, 117, 101, 162, 41, 43, 88, 78, 83, 79, 98,
                        106, 101, 99, 116, 95, 16, 20, 97, 115, 100, 102, 103, 104, 106, 107, 108,
                        113, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 85, 69, 109, 97, 105,
                        108, 210, 47, 16, 48, 50, 90, 78, 83, 46, 111, 98, 106, 101, 99, 116, 115,
                        161, 49, 128, 9, 128, 14, 215, 15, 16, 17, 18, 19, 20, 21, 52, 23, 54, 55,
                        26, 56, 26, 128, 11, 128, 15, 128, 13, 128, 12, 128, 13, 212, 29, 30, 31,
                        16, 32, 33, 34, 35, 128, 3, 128, 4, 128, 5, 95, 16, 20, 97, 115, 100, 102,
                        103, 104, 106, 107, 108, 113, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109,
                        85, 86, 97, 108, 117, 101, 210, 47, 16, 64, 50, 160, 128, 14, 210, 39, 40,
                        67, 68, 87, 78, 83, 65, 114, 114, 97, 121, 162, 67, 43, 210, 39, 40, 70,
                        71, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117,
                        108, 116, 162, 72, 43, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114,
                        82, 101, 115, 117, 108, 116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41, 0,
                        50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100, 0, 120, 0, 126, 0,
                        141, 0, 144, 0, 151, 0, 154, 0, 156, 0, 158, 0, 161, 0, 164, 0, 166, 0,
                        168, 0, 170, 0, 172, 0, 174, 0, 176, 0, 185, 0, 206, 0, 229, 0, 240, 0,
                        242, 0, 244, 0, 246, 0, 248, 0, 250, 0, 252, 1, 1, 1, 12, 1, 21, 1, 29, 1,
                        32, 1, 41, 1, 64, 1, 70, 1, 75, 1, 86, 1, 88, 1, 90, 1, 92, 1, 107, 1, 109,
                        1, 111, 1, 113, 1, 115, 1, 117, 1, 126, 1, 128, 1, 130, 1, 132, 1, 155, 1,
                        161, 1, 166, 1, 167, 1, 169, 1, 174, 1, 182, 1, 185, 1, 190, 1, 208, 1,
                        211, 1, 229, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 231,
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
                data: vec![vec![OutputData::String("mailto:asdfghjklq@gmail.com")]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(12)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_emoji_bold_underline() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/EmojiBoldUnderline");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(3)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(3)],
                    vec![OutputData::Object(5)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(4)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(9)],
                    vec![OutputData::Object(16)],
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
                data: vec![vec![OutputData::String("🅱\u{fe0f}Bold_Underline")]],
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                ],
            },
            Archived::Class(Class {
                name_index: 6,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(0)]],
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
            Archived::Type(7),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(13)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(8)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextBoldAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(8)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextUnderlineAttributeName")]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_extra_data() {
        // This test file is missing a block of text so the pointers become misaligned during parsing
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/ExtraData");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let root = typedstream.oxidize();
        assert!(root.is_err());
    }

    #[test]
    fn test_parse_text_formatted() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/Formatted");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(9)], // Changed 6 to 9 here
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
                data: vec![vec![OutputData::String("Big small")]], // Changed "Test 3" to "Big small"
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
    fn test_parse_text_long_message() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/LongMessage");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                        OutputData::UnsignedInteger(2359),
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
                    "Sed nibh velit, sodales et facilisis ut, sodales id libero. Mauris nec venenatis lorem, ac vulputate lorem. Maecenas in faucibus dui. In hac habitasse platea dictumst. Integer commodo erat eu elit tincidunt malesuada. Ut tellus mi, eleifend a ligula vitae, eleifend malesuada nibh. Duis leo magna, porttitor eu viverra varius, laoreet eget urna. Duis faucibus eleifend pretium. Nulla nec orci rhoncus, tincidunt lorem non, viverra velit. Aliquam id tincidunt lacus, vel accumsan enim.\nProin id ultrices nunc. Integer id posuere tellus. Donec vitae lacinia elit. In diam est, scelerisque non lacus ultrices, consequat hendrerit ex. Proin ut felis mi. Fusce id ultrices mi. Duis sagittis justo quis sapien tincidunt faucibus. Suspendisse id feugiat risus, ac vestibulum neque. Sed sit amet mauris mauris.\nAenean pharetra, nisl eu maximus commodo, est leo volutpat tortor, sodales semper risus eros non lorem. Phasellus auctor erat quis ante tristique ultrices. Nam at eleifend ligula. Donec posuere lobortis ante quis pulvinar. Maecenas cursus nibh sit amet finibus ultrices. Etiam ut volutpat risus, in molestie velit. Curabitur ornare justo lacus, vitae consequat augue commodo eget. Praesent tincidunt, urna et posuere mattis, orci diam efficitur lorem, eu tincidunt tortor mi id velit. Morbi justo felis, placerat accumsan blandit vel, accumsan eu mauris. Aliquam mattis nisl sed pulvinar hendrerit. Fusce hendrerit fermentum tellus, sit amet lacinia ante viverra ac. Vivamus ultricies tristique congue. Aliquam varius, odio ut porta consectetur, justo est dignissim massa, id dapibus orci risus dignissim dolor. Aliquam viverra tincidunt neque vel euismod. Integer semper ultricies libero vel cursus. Vestibulum sapien eros, dictum id ultricies in, accumsan vel est.\nCurabitur et lacus quis mauris viverra accumsan a non dui. Donec efficitur ex vitae maximus facilisis. Suspendisse molestie lectus quis bibendum porta. Donec a tellus vehicula, iaculis libero non, tincidunt dolor. Curabitur sit amet felis quis magna euismod feugiat vel vitae magna. Proin tellus nunc, mollis quis ipsum ac, blandit tristique libero. Etiam sit amet hendrerit nisl. Mauris dapibus tortor vel enim interdum faucibus. Nulla facilisi. Nulla ut nulla sit amet leo accumsan convallis eget in tortor. Donec sit amet ullamcorper urna. Curabitur consectetur cursus sem nec accumsan.",
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
    fn test_parse_text_mention() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/Mention");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(5)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(3)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
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
                data: vec![vec![OutputData::String("Test Dad ")]],
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
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMentionConfirmedMention")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("+15558675309")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_multi_attachment() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/MultiAttachment");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::SignedInteger(4), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(26)],
                    vec![OutputData::SignedInteger(5), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(29)],
                    vec![OutputData::SignedInteger(6), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(32)],
                    vec![OutputData::SignedInteger(7), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(35)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(38)],
                    vec![OutputData::SignedInteger(9), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(41)],
                    vec![
                        OutputData::SignedInteger(10),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(44)],
                    vec![
                        OutputData::SignedInteger(11),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(47)],
                    vec![
                        OutputData::SignedInteger(12),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(50)],
                    vec![
                        OutputData::SignedInteger(13),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(53)],
                    vec![
                        OutputData::SignedInteger(14),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(56)],
                    vec![
                        OutputData::SignedInteger(15),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(59)],
                    vec![
                        OutputData::SignedInteger(16),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(62)],
                    vec![
                        OutputData::SignedInteger(17),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(65)],
                    vec![
                        OutputData::SignedInteger(18),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(68)],
                    vec![
                        OutputData::SignedInteger(19),
                        OutputData::UnsignedInteger(1),
                    ],
                    vec![OutputData::Object(71)],
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
                data: vec![vec![OutputData::String("￼￼￼￼￼￼￼￼￼￼￼￼￼￼￼￼￼￼￼")]],
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
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
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
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_0_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(20)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_1_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(24)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(25)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(2)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_2_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(27)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(28)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(3)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_3_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(30)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(31)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(4)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_4_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(33)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(34)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(5)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_5_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(36)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(37)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(6)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_6_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(39)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(40)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(7)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_7_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(42)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(43)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(8)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_8_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(45)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(46)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(9)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_9_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(48)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(49)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(10)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_10_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(51)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(52)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(11)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_11_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(54)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(55)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(12)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_12_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(57)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(58)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(13)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_13_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(60)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(61)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(14)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_14_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(63)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(64)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(15)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_15_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(66)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(67)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(16)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_16_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(69)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(70)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(17)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_17_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(72)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(73)],
                ],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(18)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_18_48B9C973-3466-438C-BE72-E5B498D30772",
                )]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_multi_part() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/MultiPart");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::SignedInteger(4), OutputData::UnsignedInteger(7)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::SignedInteger(5), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(24)],
                    vec![OutputData::SignedInteger(6), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(27)],
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
                data: vec![vec![OutputData::String("￼test 1￼test 2 ￼test 3")]],
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
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(12)],
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
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_0_F0668F79-20C2-49C9-A87F-1B007ABB0CED",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(14),
            }),
            Archived::Class(Class {
                name_index: 11,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(9),
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(21)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_2_F0668F79-20C2-49C9-A87F-1B007ABB0CED",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(2)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(23)],
                ],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(3)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(25)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(26)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "at_4_F0668F79-20C2-49C9-A87F-1B007ABB0CED",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(4)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(28)],
                ],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(5)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_multi_part_with_deleted() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/MultiPartWithDeleted");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                        OutputData::UnsignedInteger(28),
                    ],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(14)],
                    vec![
                        OutputData::SignedInteger(3),
                        OutputData::UnsignedInteger(32),
                    ],
                    vec![OutputData::Object(19)],
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
                    "From arbitrary byte stream:\r￼To native Rust data structures:\r",
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
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "D0551D89-4E11-43D0-9A0E-06F19704E97B",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(21)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(2)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_overlapping_format() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/OverlappingFormat");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::String("NSData")],
            vec![Type::Array(820)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(2)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::SignedInteger(4), OutputData::UnsignedInteger(2)],
                    vec![OutputData::Object(25)],
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
                data: vec![vec![OutputData::String("8:00 pm")]],
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
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                ],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextBoldAttributeName")]],
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
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Type(13),
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
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMCalendarEventAttributeName")]],
            },
            Archived::Object {
                class: 21,
                data: vec![
                    vec![OutputData::SignedInteger(820)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 10, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 27, 128, 1, 175,
                        16, 28, 13, 14, 28, 36, 37, 38, 44, 45, 46, 53, 59, 26, 63, 64, 65, 68, 71,
                        75, 81, 85, 86, 87, 88, 96, 100, 101, 102, 103, 85, 36, 110, 117, 108, 108,
                        215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 26, 82, 77, 83,
                        86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81, 84, 81, 80, 82, 83, 82, 82,
                        86, 78, 128, 6, 128, 16, 128, 2, 128, 7, 16, 1, 128, 8, 212, 29, 30, 31,
                        16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114, 97, 110, 103, 101, 118,
                        97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16, 20, 78, 83, 46, 114, 97,
                        110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97, 116, 105, 111, 110, 90,
                        78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128, 3, 128, 4, 16, 4, 128, 5,
                        16, 7, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99, 108, 97, 115, 115, 110, 97,
                        109, 101, 88, 36, 99, 108, 97, 115, 115, 101, 115, 87, 78, 83, 86, 97, 108,
                        117, 101, 162, 41, 43, 88, 78, 83, 79, 98, 106, 101, 99, 116, 87, 56, 58,
                        48, 48, 32, 112, 109, 84, 84, 105, 109, 101, 210, 47, 16, 48, 52, 90, 78,
                        83, 46, 111, 98, 106, 101, 99, 116, 115, 163, 49, 50, 51, 128, 9, 128, 17,
                        128, 22, 128, 15, 215, 15, 16, 17, 18, 19, 20, 21, 54, 23, 56, 57, 26, 58,
                        26, 128, 12, 128, 16, 128, 10, 128, 13, 128, 14, 212, 29, 30, 31, 16, 60,
                        33, 34, 35, 128, 11, 128, 4, 128, 5, 81, 56, 85, 72, 111, 117, 114, 115,
                        210, 47, 16, 66, 52, 160, 128, 15, 210, 39, 40, 69, 70, 87, 78, 83, 65,
                        114, 114, 97, 121, 162, 69, 43, 210, 39, 40, 72, 73, 95, 16, 15, 68, 68,
                        83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108, 116, 162, 74, 43,
                        95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108,
                        116, 215, 15, 16, 17, 18, 19, 20, 21, 76, 23, 78, 79, 26, 58, 26, 128, 20,
                        128, 16, 128, 18, 128, 21, 128, 14, 212, 29, 30, 31, 16, 82, 82, 34, 35,
                        128, 19, 128, 19, 128, 5, 16, 2, 82, 48, 48, 87, 77, 105, 110, 117, 116,
                        101, 115, 216, 15, 16, 89, 17, 18, 19, 20, 21, 90, 23, 90, 93, 94, 26, 58,
                        26, 81, 86, 128, 25, 128, 16, 128, 25, 128, 23, 128, 26, 128, 14, 212, 29,
                        30, 31, 16, 82, 98, 34, 35, 128, 19, 128, 24, 128, 5, 16, 5, 82, 112, 109,
                        88, 77, 101, 114, 105, 100, 105, 97, 110, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36,
                        0, 41, 0, 50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100, 0, 131, 0,
                        137, 0, 152, 0, 155, 0, 162, 0, 165, 0, 167, 0, 169, 0, 172, 0, 175, 0,
                        177, 0, 179, 0, 181, 0, 183, 0, 185, 0, 187, 0, 196, 0, 217, 0, 240, 0,
                        251, 0, 253, 0, 255, 1, 1, 1, 3, 1, 5, 1, 7, 1, 12, 1, 23, 1, 32, 1, 40, 1,
                        43, 1, 52, 1, 60, 1, 65, 1, 70, 1, 81, 1, 85, 1, 87, 1, 89, 1, 91, 1, 93,
                        1, 108, 1, 110, 1, 112, 1, 114, 1, 116, 1, 118, 1, 127, 1, 129, 1, 131, 1,
                        133, 1, 135, 1, 141, 1, 146, 1, 147, 1, 149, 1, 154, 1, 162, 1, 165, 1,
                        170, 1, 188, 1, 191, 1, 209, 1, 224, 1, 226, 1, 228, 1, 230, 1, 232, 1,
                        234, 1, 243, 1, 245, 1, 247, 1, 249, 1, 251, 1, 254, 2, 6, 2, 23, 2, 25, 2,
                        27, 2, 29, 2, 31, 2, 33, 2, 35, 2, 37, 2, 46, 2, 48, 2, 50, 2, 52, 2, 54,
                        2, 57, 2, 66, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 104, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 68,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 14,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                ],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(24)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextUnderlineAttributeName")]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(26)],
                    vec![OutputData::Object(10)],
                ],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextItalicAttributeName")]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_phone_number() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/PhoneNumber");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(11),
                    ],
                    vec![OutputData::Object(7)],
                    vec![
                        OutputData::SignedInteger(2),
                        OutputData::UnsignedInteger(10),
                    ],
                    vec![OutputData::Object(16)],
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
                data: vec![vec![OutputData::String("What about 0000000000")]],
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
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
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
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Object {
                class: 8,
                data: vec![
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                ],
            },
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
                data: vec![vec![OutputData::String("tel:0000000000")]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMPhoneNumberAttributeName")]],
            },
            Archived::Object {
                class: 23,
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
                        3, 128, 4, 16, 4, 128, 5, 16, 10, 16, 11, 210, 39, 40, 41, 42, 90, 36, 99,
                        108, 97, 115, 115, 110, 97, 109, 101, 88, 36, 99, 108, 97, 115, 115, 101,
                        115, 87, 78, 83, 86, 97, 108, 117, 101, 162, 41, 43, 88, 78, 83, 79, 98,
                        106, 101, 99, 116, 90, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 91, 80, 104,
                        111, 110, 101, 78, 117, 109, 98, 101, 114, 210, 47, 16, 48, 50, 90, 78, 83,
                        46, 111, 98, 106, 101, 99, 116, 115, 161, 49, 128, 9, 128, 14, 215, 15, 16,
                        17, 18, 19, 20, 21, 52, 23, 54, 55, 26, 56, 26, 128, 11, 128, 15, 128, 10,
                        128, 12, 128, 13, 212, 29, 30, 31, 16, 32, 33, 34, 35, 128, 3, 128, 4, 128,
                        5, 90, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 85, 86, 97, 108, 117, 101,
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
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_styled_link() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/StyledLink");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::String("NSData")],
            vec![Type::Array(591)],
            vec![Type::String("NSURL")],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(61),
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
                    "https://github.com/ReagentX/imessage-exporter/discussions/553",
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
                    vec![OutputData::SignedInteger(6)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::Object(24)],
                    vec![OutputData::Object(25)],
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
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Type(14),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMDataDetectedAttributeName")]],
            },
            Archived::Object {
                class: 19,
                data: vec![
                    vec![OutputData::SignedInteger(591)],
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
                        16, 61, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99, 108, 97, 115, 115, 110, 97,
                        109, 101, 88, 36, 99, 108, 97, 115, 115, 101, 115, 87, 78, 83, 86, 97, 108,
                        117, 101, 162, 41, 43, 88, 78, 83, 79, 98, 106, 101, 99, 116, 95, 16, 61,
                        104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99,
                        111, 109, 47, 82, 101, 97, 103, 101, 110, 116, 88, 47, 105, 109, 101, 115,
                        115, 97, 103, 101, 45, 101, 120, 112, 111, 114, 116, 101, 114, 47, 100,
                        105, 115, 99, 117, 115, 115, 105, 111, 110, 115, 47, 53, 53, 51, 87, 72,
                        116, 116, 112, 85, 82, 76, 210, 47, 16, 48, 49, 90, 78, 83, 46, 111, 98,
                        106, 101, 99, 116, 115, 160, 128, 9, 210, 39, 40, 51, 52, 87, 78, 83, 65,
                        114, 114, 97, 121, 162, 51, 43, 210, 39, 40, 54, 55, 95, 16, 15, 68, 68,
                        83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108, 116, 162, 56, 43,
                        95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108,
                        116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41, 0, 50, 0, 55, 0, 73, 0, 78,
                        0, 86, 0, 96, 0, 98, 0, 100, 0, 113, 0, 119, 0, 134, 0, 137, 0, 144, 0,
                        147, 0, 149, 0, 151, 0, 154, 0, 157, 0, 159, 0, 161, 0, 163, 0, 165, 0,
                        167, 0, 169, 0, 178, 0, 199, 0, 222, 0, 233, 0, 235, 0, 237, 0, 239, 0,
                        241, 0, 243, 0, 245, 0, 250, 1, 5, 1, 14, 1, 22, 1, 25, 1, 34, 1, 98, 1,
                        106, 1, 111, 1, 122, 1, 123, 1, 125, 1, 130, 1, 138, 1, 141, 1, 146, 1,
                        164, 1, 167, 1, 185, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 187,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 15,
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
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMTextEffectAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![vec![OutputData::SignedInteger(5)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMLinkAttributeName")]],
            },
            Archived::Object {
                class: 26,
                data: vec![
                    vec![OutputData::SignedInteger(0)],
                    vec![OutputData::Object(27)],
                ],
            },
            Archived::Class(Class {
                name_index: 17,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "https://github.com/ReagentX/imessage-exporter/discussions/553",
                )]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_effects() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/TextEffects");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(3)],
                    vec![OutputData::Object(5)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::SignedInteger(4), OutputData::UnsignedInteger(5)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(1)],
                    vec![OutputData::SignedInteger(5), OutputData::UnsignedInteger(3)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(1)],
                    vec![OutputData::SignedInteger(6), OutputData::UnsignedInteger(8)],
                    vec![OutputData::Object(23)],
                    vec![OutputData::SignedInteger(7), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(25)],
                    vec![OutputData::SignedInteger(6), OutputData::UnsignedInteger(1)],
                    vec![OutputData::SignedInteger(8), OutputData::UnsignedInteger(5)],
                    vec![OutputData::Object(27)],
                    vec![OutputData::SignedInteger(6), OutputData::UnsignedInteger(1)],
                    vec![OutputData::SignedInteger(9), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(29)],
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
                data: vec![vec![OutputData::String(
                    "Big small shake nod explode ripple bloom jitter",
                )]],
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
                data: vec![vec![OutputData::String("__kIMTextEffectAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(5)]],
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
            Archived::Type(7),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(11)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(9)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(8)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(24)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(12)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(26)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(4)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(28)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(6)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(30)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(10)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_styles() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/TextStyles");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(3)],
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(4)],
                    vec![OutputData::Object(5)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(9)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![OutputData::SignedInteger(4), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(1)],
                    vec![
                        OutputData::SignedInteger(5),
                        OutputData::UnsignedInteger(13),
                    ],
                    vec![OutputData::Object(20)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(5)],
                    vec![OutputData::SignedInteger(6), OutputData::UnsignedInteger(4)],
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
                data: vec![vec![OutputData::String(
                    "Bold underline italic strikethrough all four",
                )]],
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
                data: vec![vec![OutputData::String("__kIMTextBoldAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(1)]],
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
            Archived::Type(7),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(1)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextUnderlineAttributeName")]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextItalicAttributeName")]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(2)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "__kIMTextStrikethroughAttributeName",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(5)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(21)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(8)],
                ],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_styles_mixed() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/TextStylesMixed");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(9)],
                    vec![OutputData::Object(5)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(8)],
                    vec![OutputData::Object(17)],
                    vec![OutputData::SignedInteger(3), OutputData::UnsignedInteger(6)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::SignedInteger(2), OutputData::UnsignedInteger(7)],
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
                data: vec![vec![OutputData::String("Underline normal jitter normal")]],
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
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
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextUnderlineAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(1)]],
            },
            Archived::Type(7),
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
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(3)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(13)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                ],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextEffectAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(10)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_styles_single_range() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/TextStylesSingleRange");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(3)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(10),
                    ],
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
                data: vec![vec![OutputData::String("Everything")]],
            },
            Archived::Class(Class {
                name_index: 3,
                version: 1,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 6,
                data: vec![
                    vec![OutputData::SignedInteger(5)],
                    vec![OutputData::Object(7)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(13)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(15)],
                    vec![OutputData::Object(8)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(8)],
                ],
            },
            Archived::Class(Class {
                name_index: 6,
                version: 0,
                parent_index: Some(2),
            }),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextBoldAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(1)]],
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
            Archived::Type(7),
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "__kIMTextStrikethroughAttributeName",
                )]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 9,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("__kIMTextUnderlineAttributeName")]],
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
    fn test_parse_text_transcription() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/Transcription");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                    vec![OutputData::SignedInteger(1), OutputData::UnsignedInteger(1)],
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
                data: vec![vec![OutputData::String("￼")]],
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
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(11)],
                    vec![OutputData::Object(12)],
                    vec![OutputData::Object(16)],
                    vec![OutputData::Object(17)],
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
                    "__kIMFileTransferGUIDAttributeName",
                )]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "4C339597-EBBB-4978-9B87-521C0471A848",
                )]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("IMAudioTranscription")]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String("This is a test")]],
            },
            Archived::Object {
                class: 4,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 13,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(14),
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
                class: 13,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_url() {
        let typedstream_path = current_dir().unwrap().as_path().join("src/test_data/URL");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::String("NSURL")],
            vec![Type::SignedInt],
            vec![Type::String("NSNumber")],
            vec![Type::String("NSValue")],
            vec![Type::EmbeddedData],
            vec![Type::SignedInt],
            vec![Type::String("NSData")],
            vec![Type::Array(582)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(34),
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
                    "https://github.com/ReagentX/Logria",
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
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(13)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(20)],
                    vec![OutputData::Object(21)],
                ],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMLinkAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![
                    vec![OutputData::SignedInteger(0)],
                    vec![OutputData::Object(12)],
                ],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "https://github.com/ReagentX/Logria",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 15,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Class(Class {
                name_index: 12,
                version: 0,
                parent_index: Some(16),
            }),
            Archived::Class(Class {
                name_index: 13,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(15),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "__kIMBaseWritingDirectionAttributeName",
                )]],
            },
            Archived::Object {
                class: 15,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMDataDetectedAttributeName")]],
            },
            Archived::Object {
                class: 22,
                data: vec![
                    vec![OutputData::SignedInteger(582)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 10, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 11, 128, 1, 172,
                        13, 14, 28, 36, 37, 38, 44, 45, 46, 50, 54, 58, 85, 36, 110, 117, 108, 108,
                        215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 26, 82, 77, 83,
                        86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81, 84, 81, 80, 82, 83, 82, 82,
                        86, 78, 128, 6, 128, 10, 128, 2, 128, 7, 16, 1, 128, 8, 212, 29, 30, 31,
                        16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114, 97, 110, 103, 101, 118,
                        97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16, 20, 78, 83, 46, 114, 97,
                        110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97, 116, 105, 111, 110, 90,
                        78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128, 3, 128, 4, 16, 4, 128, 5,
                        16, 34, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99, 108, 97, 115, 115, 110, 97,
                        109, 101, 88, 36, 99, 108, 97, 115, 115, 101, 115, 87, 78, 83, 86, 97, 108,
                        117, 101, 162, 41, 43, 88, 78, 83, 79, 98, 106, 101, 99, 116, 95, 16, 34,
                        104, 116, 116, 112, 115, 58, 47, 47, 103, 105, 116, 104, 117, 98, 46, 99,
                        111, 109, 47, 82, 101, 97, 103, 101, 110, 116, 88, 47, 76, 111, 103, 114,
                        105, 97, 87, 72, 116, 116, 112, 85, 82, 76, 210, 47, 16, 48, 49, 90, 78,
                        83, 46, 111, 98, 106, 101, 99, 116, 115, 160, 128, 9, 210, 39, 40, 51, 52,
                        94, 78, 83, 77, 117, 116, 97, 98, 108, 101, 65, 114, 114, 97, 121, 163, 51,
                        53, 43, 87, 78, 83, 65, 114, 114, 97, 121, 210, 39, 40, 55, 56, 95, 16, 15,
                        68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115, 117, 108, 116, 162,
                        57, 43, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115,
                        117, 108, 116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41, 0, 50, 0, 55, 0,
                        73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100, 0, 113, 0, 119, 0, 134, 0, 137, 0,
                        144, 0, 147, 0, 149, 0, 151, 0, 154, 0, 157, 0, 159, 0, 161, 0, 163, 0,
                        165, 0, 167, 0, 169, 0, 178, 0, 199, 0, 222, 0, 233, 0, 235, 0, 237, 0,
                        239, 0, 241, 0, 243, 0, 245, 0, 250, 1, 5, 1, 14, 1, 22, 1, 25, 1, 34, 1,
                        71, 1, 79, 1, 84, 1, 95, 1, 96, 1, 98, 1, 103, 1, 118, 1, 122, 1, 130, 1,
                        135, 1, 153, 1, 156, 1, 174, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0,
                        59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 176,
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

    #[test]
    fn test_parse_text_url_message() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/URLMessage");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
            vec![Type::String("NSURL")],
            vec![Type::SignedInt],
            vec![Type::String("NSNumber")],
            vec![Type::String("NSValue")],
            vec![Type::EmbeddedData],
            vec![Type::SignedInt],
            vec![Type::String("NSMutableData")],
            vec![Type::String("NSData")],
            vec![Type::Array(604)],
        ];

        let expected_objects = vec![
            Archived::Object {
                class: 1,
                data: vec![
                    vec![OutputData::Object(4)],
                    vec![
                        OutputData::SignedInteger(1),
                        OutputData::UnsignedInteger(56),
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
                    "https://twitter.com/xxxxxxxxx/status/0000223300009216128",
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
                    vec![OutputData::SignedInteger(4)],
                    vec![OutputData::Object(9)],
                    vec![OutputData::Object(10)],
                    vec![OutputData::Object(13)],
                    vec![OutputData::Object(14)],
                    vec![OutputData::Object(18)],
                    vec![OutputData::Object(19)],
                    vec![OutputData::Object(22)],
                    vec![OutputData::Object(23)],
                ],
            },
            Archived::Class(Class {
                name_index: 8,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMLinkAttributeName")]],
            },
            Archived::Object {
                class: 11,
                data: vec![
                    vec![OutputData::SignedInteger(0)],
                    vec![OutputData::Object(12)],
                ],
            },
            Archived::Class(Class {
                name_index: 10,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String(
                    "https://twitter.com/xxxxxxxxx/status/0000223300009216128",
                )]],
            },
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMMessagePartAttributeName")]],
            },
            Archived::Object {
                class: 15,
                data: vec![vec![OutputData::SignedInteger(0)]],
            },
            Archived::Class(Class {
                name_index: 12,
                version: 0,
                parent_index: Some(16),
            }),
            Archived::Class(Class {
                name_index: 13,
                version: 0,
                parent_index: Some(3),
            }),
            Archived::Type(15),
            Archived::Object {
                class: 6,
                data: vec![vec![OutputData::String("__kIMDataDetectedAttributeName")]],
            },
            Archived::Object {
                class: 20,
                data: vec![
                    vec![OutputData::SignedInteger(604)],
                    vec![OutputData::Array(&[
                        98, 112, 108, 105, 115, 116, 48, 48, 212, 1, 2, 3, 4, 5, 6, 7, 12, 88, 36,
                        118, 101, 114, 115, 105, 111, 110, 89, 36, 97, 114, 99, 104, 105, 118, 101,
                        114, 84, 36, 116, 111, 112, 88, 36, 111, 98, 106, 101, 99, 116, 115, 18, 0,
                        1, 134, 160, 95, 16, 15, 78, 83, 75, 101, 121, 101, 100, 65, 114, 99, 104,
                        105, 118, 101, 114, 210, 8, 9, 13, 11, 87, 118, 101, 114, 115, 105, 111,
                        110, 89, 100, 100, 45, 114, 101, 115, 117, 108, 116, 128, 11, 128, 1, 172,
                        13, 14, 28, 36, 37, 38, 44, 45, 46, 50, 54, 58, 85, 36, 110, 117, 108, 108,
                        215, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 26, 82, 77, 83,
                        86, 36, 99, 108, 97, 115, 115, 82, 65, 82, 81, 84, 81, 80, 82, 83, 82, 82,
                        86, 78, 128, 6, 128, 13, 128, 2, 128, 7, 16, 1, 128, 8, 212, 29, 30, 31,
                        16, 32, 33, 34, 35, 95, 16, 18, 78, 83, 46, 114, 97, 110, 103, 101, 118,
                        97, 108, 46, 108, 101, 110, 103, 116, 104, 95, 16, 20, 78, 83, 46, 114, 97,
                        110, 103, 101, 118, 97, 108, 46, 108, 111, 99, 97, 116, 105, 111, 110, 90,
                        78, 83, 46, 115, 112, 101, 99, 105, 97, 108, 128, 3, 128, 4, 16, 4, 128, 5,
                        16, 56, 16, 0, 210, 39, 40, 41, 42, 90, 36, 99, 108, 97, 115, 115, 110, 97,
                        109, 101, 88, 36, 99, 108, 97, 115, 115, 101, 115, 87, 78, 83, 86, 97, 108,
                        117, 101, 162, 41, 43, 88, 78, 83, 79, 98, 106, 101, 99, 116, 95, 16, 56,
                        104, 116, 116, 112, 115, 58, 47, 47, 116, 119, 105, 116, 116, 101, 114, 46,
                        99, 111, 109, 47, 120, 120, 120, 120, 120, 120, 120, 120, 120, 47, 115,
                        116, 97, 116, 117, 115, 47, 48, 48, 48, 48, 50, 50, 51, 51, 48, 48, 48, 48,
                        57, 50, 49, 54, 49, 50, 56, 87, 72, 116, 116, 112, 85, 82, 76, 210, 47, 16,
                        48, 49, 90, 78, 83, 46, 111, 98, 106, 101, 99, 116, 115, 160, 128, 9, 210,
                        39, 40, 51, 52, 94, 78, 83, 77, 117, 116, 97, 98, 108, 101, 65, 114, 114,
                        97, 121, 163, 51, 53, 43, 87, 78, 83, 65, 114, 114, 97, 121, 210, 39, 40,
                        55, 56, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101, 114, 82, 101, 115,
                        117, 108, 116, 162, 57, 43, 95, 16, 15, 68, 68, 83, 99, 97, 110, 110, 101,
                        114, 82, 101, 115, 117, 108, 116, 16, 1, 0, 8, 0, 17, 0, 26, 0, 36, 0, 41,
                        0, 50, 0, 55, 0, 73, 0, 78, 0, 86, 0, 96, 0, 98, 0, 100, 0, 113, 0, 119, 0,
                        134, 0, 137, 0, 144, 0, 147, 0, 149, 0, 151, 0, 154, 0, 157, 0, 159, 0,
                        161, 0, 163, 0, 165, 0, 167, 0, 169, 0, 178, 0, 199, 0, 222, 0, 233, 0,
                        235, 0, 237, 0, 239, 0, 241, 0, 243, 0, 245, 0, 250, 1, 5, 1, 14, 1, 22, 1,
                        25, 1, 34, 1, 93, 1, 101, 1, 106, 1, 117, 1, 118, 1, 120, 1, 125, 1, 140,
                        1, 144, 1, 152, 1, 157, 1, 175, 1, 178, 1, 196, 0, 0, 0, 0, 0, 0, 2, 1, 0,
                        0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 198,
                    ])],
                ],
            },
            Archived::Class(Class {
                name_index: 16,
                version: 0,
                parent_index: Some(21),
            }),
            Archived::Class(Class {
                name_index: 17,
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
                class: 15,
                data: vec![vec![OutputData::SignedInteger(-1)]],
            },
        ];

        assert_eq!(typedstream.type_table, expected_types);
        assert_eq!(typedstream.object_table, expected_objects);
    }

    #[test]
    fn test_parse_text_weird() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/WeirdText");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

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

        println!("\nFound {:?} objects:", typedstream.object_table.len());
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
                        OutputData::UnsignedInteger(21),
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
                data: vec![vec![OutputData::String("𝖍𝖊𝖑𝖑𝖔 𝖜𝖔𝖗𝖑𝖉")]],
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
    fn test_parse_large_with_null() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/HugeWithRefs");
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        let mut typedstream = TypedStreamDeserializer::new(&bytes);

        // Unwrapping here means we resolved the object
        let root = typedstream.oxidize().unwrap();
        println!("\nResults: {root:?}");
        // Unwrapping here means we resolved the properties
        let root_obj = typedstream.resolve_properties(root).unwrap();
        println!("\nResults: {root_obj:?}");

        // This will only complete if the circular references are handled
        let primitives = root_obj.primitives();
        println!("\nPrimitive Values: {primitives:?}");
    }
}
