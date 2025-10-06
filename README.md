# desynt

A Rust library for handling syn objects with raw identifier prefixes and path normalization.

[![Crates.io](https://img.shields.io/crates/v/desynt.svg)](https://crates.io/crates/desynt)
[![Documentation](https://docs.rs/desynt/badge.svg)](https://docs.rs/desynt)
[![License](https://img.shields.io/crates/l/desynt.svg)](LICENSE)

## Overview

`desynt` provides utilities for working with [`syn`](https://github.com/dtolnay/syn) objects, particularly focusing on:

- **Stripping raw identifier prefixes** (`r#`) from `Ident`, `Path`, and `PathSegment` objects
- **Path normalization** and canonical type resolution
- **Type mapping** from various path representations to canonical forms

This library is especially useful when working with proc macros or code analysis tools where you need to normalize type references regardless of how they're written in the source code.

## Features

### Raw Identifier Stripping

Strip `r#` prefixes from syn objects while preserving spans and other metadata:

```rust
use syn::Ident;
use desynt::StripRaw;

let ident: Ident = syn::parse_str("r#type").unwrap();
let stripped = ident.strip_raw();
assert_eq!(stripped.to_string(), "type");
```

### Path Normalization

Handle complex paths with raw identifiers anywhere in the path:

```rust
use syn::Path;
use desynt::StripRaw;

let path: Path = syn::parse_str("::r#std::r#string::String").unwrap();
let stripped = path.strip_raw();

let segments: Vec<String> = stripped.segments.iter()
    .map(|seg| seg.ident.to_string())
    .collect();
assert_eq!(segments, vec!["std", "string", "String"]);
```

### Type Resolution

Map various path representations to canonical type names:

```rust
use desynt::PathResolver;
use syn::Path;

let mut resolver = PathResolver::new();
resolver.add_mapping("std::string::String", "String");
resolver.add_mapping("std::vec::Vec", "Vec");

// These all resolve to the same canonical type
let paths = [
    "::std::string::String",
    "::r#std::r#string::String", 
    "std::string::String"
];

for path_str in &paths {
    let path: Path = syn::parse_str(path_str).unwrap();
    assert_eq!(resolver.resolve(&path), Some("String"));
}
```

### Pre-built Primitive Mappings

Use the built-in resolver with common primitive and standard library type mappings:

```rust
use desynt::PathResolver;

let resolver = PathResolver::with_primitives();

// Resolves primitive types from various forms
let f64_paths = [
    "::std::primitive::f64",
    "::core::primitive::f64", 
    "std::f64",
    "core::f64"
];

for path_str in &f64_paths {
    let path: syn::Path = syn::parse_str(path_str).unwrap();
    assert_eq!(resolver.resolve(&path), Some("f64"));
}
```

## API Reference

### Traits

- **`StripRaw`** - Strip raw identifier prefixes from syn objects
- **`HasRaw`** - Check if syn objects contain raw identifiers

### Types

- **`PathResolver`** - Maps path patterns to canonical type names

### Implementations

The library provides implementations for:

- `syn::Ident`
- `syn::Path`
- `syn::PathSegment`

### Utility Functions

The `utils` module provides helper functions for string-based operations:

- `is_raw_ident()` - Check if a string is a raw identifier
- `strip_raw_prefix()` - Strip raw prefix from strings
- `ident_from_string()` - Create identifiers from strings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
desynt = "0.1"
syn = { version = "2.0", features = ["full"] }
```

## Use Cases

### Proc Macros

When writing proc macros, you often need to handle type names consistently regardless of how they're written:

```rust
use desynt::{PathResolver, StripRaw};
use syn::{parse_macro_input, ItemFn, Path};

#[proc_macro_attribute]
pub fn my_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let resolver = PathResolver::with_primitives();
    
    // Normalize all type references in function parameters
    for param in &input.sig.inputs {
        if let syn::FnArg::Typed(typed) = param {
            if let syn::Type::Path(type_path) = &*typed.ty {
                let canonical = resolver.resolve(&type_path.path);
                // Use canonical type name for code generation
            }
        }
    }
    
    // ... rest of macro implementation
}
```

### Code Analysis

Analyze Rust code and normalize type references:

```rust
use desynt::PathResolver;
use syn::{visit::Visit, File, Path};

struct TypeCollector {
    resolver: PathResolver,
    types: Vec<String>,
}

impl<'ast> Visit<'ast> for TypeCollector {
    fn visit_path(&mut self, path: &'ast Path) {
        if let Some(canonical) = self.resolver.resolve(path) {
            self.types.push(canonical.to_string());
        }
        syn::visit::visit_path(self, path);
    }
}
```

## Testing

The library includes comprehensive tests covering various scenarios:

```bash
cargo test
```

Test categories:

- **Identity tests** (`tests/ident.rs`) - Testing `Ident` functionality
- **Path tests** (`tests/path.rs`) - Testing `Path` functionality  
- **Primitive tests** (`tests/primitives.rs`) - Testing primitive type handling
- **Resolver tests** (`tests/resolver.rs`) - Testing `PathResolver` functionality
- **Utility tests** (`tests/utils.rs`) - Testing utility functions

## Requirements

- Rust 1.56.0 or later
- `syn` 2.0 or later

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0

- Initial release
- Basic raw identifier stripping for `Ident`, `Path`, and `PathSegment`
- `PathResolver` for canonical type mapping
- Pre-built primitive type mappings
- Comprehensive test suite
- Utility functions for string-based operations