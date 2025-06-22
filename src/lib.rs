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

    use crate::deserializer::typedstream::TypedStreamDeserializer;

    #[test]
    fn test_parse_text_basic() {
        let typedstream_path = current_dir()
            .unwrap()
            .as_path()
            .join("src/test_data/0123456789");
        println!("Parsing file: {:?}", typedstream_path);
        let mut file = File::open(typedstream_path).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();

        // Skip the header for now
        let mut typedstream = TypedStreamDeserializer::new(&bytes);
        let result = typedstream.oxidize().unwrap();

        println!("\n{:#?}\n", result);

        println!("\n\nFound {:?} types:", typedstream.types_table.len());
        typedstream
            .types_table
            .iter()
            .enumerate()
            .for_each(|(idx, item)| println!("\t{idx}: {item:x?}"));
    }
}
