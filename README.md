# crabstep

`crabstep` is a Rust library that deserializes Apple's `typedstream` data into cross-platform Rust data structures.

## Overview

The typedstream format is a binary serialization protocol designed for `C` and `Objective-C` data structures. It is primarily used in Apple's Foundation framework, specifically within the `NSArchiver` and `NSUnarchiver` classes.

### Quick Start

```rust,no_run
use std::{env::current_dir, fs::File, io::Read};

use crabstep::deserializer::typedstream::TypedStreamDeserializer;

// Read the typedstream file into memory
let typedstream_path = current_dir()
    .unwrap()
    .as_path()
    .join("path/to/typedstream/file");
let mut file = File::open(typedstream_path).unwrap();
let mut bytes = vec![];
file.read_to_end(&mut bytes).unwrap();

// Oxidize the typedstream
let mut typedstream = TypedStreamDeserializer::new(&bytes);
let result = typedstream.oxidize();
```

## Origin

The format is derived from the data structure used by `NeXTSTEP`'s `NXTypedStream` APIs.

## Features

- Pure Rust implementation for efficient and safe deserialization
- No dependencies on Apple frameworks
- Robust error handling for malformed or incomplete `typedstream` data

## Reverse Engineering

 A blog post describing the reverse engineering of `typedstream` is located [here](https://chrissardegna.com/blog/reverse-engineering-apples-typedstream-format/).
