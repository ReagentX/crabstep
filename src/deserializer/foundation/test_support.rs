//! Shared helpers for the Foundation accessor tests.

extern crate std;

use alloc::{vec, vec::Vec};
use std::{env::current_dir, fs::File, io::Read};

/// Load a fixture by path relative to `src/test_data`.
pub(super) fn load(rel: &str) -> Vec<u8> {
    let path = current_dir().unwrap().join("src/test_data").join(rel);
    let mut file = File::open(&path).unwrap_or_else(|e| panic!("opening fixture {path:?}: {e}"));
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    bytes
}
