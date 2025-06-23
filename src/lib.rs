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
}
