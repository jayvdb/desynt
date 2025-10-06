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

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
