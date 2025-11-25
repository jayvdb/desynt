# desynt

A Rust library for handling syn objects with raw identifier prefixes and path normalization.

[![Crates.io](https://img.shields.io/crates/v/desynt.svg)](https://crates.io/crates/desynt)
[![Documentation](https://docs.rs/desynt/badge.svg)](https://docs.rs/desynt)
[![License](https://img.shields.io/crates/l/desynt.svg)](LICENSE)

## Overview

`desynt` provides utilities for working with [`syn`](https://github.com/dtolnay/syn) objects, particularly focusing on:

- **Stripping raw identifier prefixes** (`r#`) from `Ident`, `Path`, and `PathSegment` objects
- **Path resolution and normalization** via `PathResolver` for canonical type mapping
- **Built-in type support** for Rust primitives, prelude types, and common std types
- **Multiple storage backends** - HashMap (dynamic) or phf::Map (static)

This library is especially useful when working with proc macros or code analysis tools where you need to normalize type references regardless of how they're written in the source code.

## Features

### Raw Identifier Stripping

Use the `StripRaw` trait to remove `r#` prefixes from syn objects:

```rust
use syn::Ident;
use desynt::StripRaw;

let ident: Ident = syn::parse_str("r#type").unwrap();
let stripped = ident.strip_raw();
assert_eq!(stripped.to_string(), "type");
```

### Path Resolution

Use `PathResolver` to normalize type paths and resolve them to canonical forms. This is particularly useful in proc macros where types can be referenced in multiple ways:

```rust
use desynt::{TypeGroups, DynamicPathResolver};
use std::collections::HashMap;
use syn::Path;

let mut mappings = HashMap::new();
mappings.insert("my_crate::types::UserId".to_string(), "UserId".to_string());

let resolver = DynamicPathResolver::from_map(mappings, TypeGroups::ALL);

// Resolves custom types
let path: Path = syn::parse_str("my_crate::types::UserId").unwrap();
assert_eq!(resolver.resolve(&path), Some("UserId"));

// Also handles type groups with various path forms
let path: Path = syn::parse_str("std::option::Option").unwrap();
assert_eq!(resolver.resolve(&path), Some("Option"));

let path: Path = syn::parse_str("Option<String>").unwrap();
assert_eq!(resolver.resolve(&path), Some("Option"));
```

### Type Group Categories

The `TypeGroups` struct lets you control which standard types are automatically resolved:

- **Primitives**: Language primitives (i8, u32, f64, bool, char, str, etc.)
- **Prelude**: Types in the Rust prelude (String, Vec, Option, Result, Box)
- **Common std**: Frequently used std types (HashMap, HashSet)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
