//! Built-in type path mappings for Rust standard library types.
//!
//! This module contains the mapping functions that resolve various path forms
//! of Rust standard library types to their canonical names.

use crate::TypeGroups;

/// Built-in type mappings for primitives and common standard library types
#[cfg(feature = "static-resolver")]
pub(crate) fn get_builtin_mapping_static(path: &str, groups: TypeGroups) -> Option<&'static str> {
    // Try primitives first if enabled
    if groups.primitives {
        if let Some(result) = get_primitive_mapping_static(path) {
            return Some(result);
        }
    }

    // Try prelude types if enabled
    if groups.prelude {
        if let Some(result) = get_prelude_mapping_static(path) {
            return Some(result);
        }
    }

    // Try common std types if enabled
    if groups.common_std {
        if let Some(result) = get_common_std_mapping_static(path) {
            return Some(result);
        }
    }

    None
}

/// Rust language primitive type mappings
#[cfg(feature = "static-resolver")]
fn get_primitive_mapping_static(path: &str) -> Option<&'static str> {
    match path {
        // Primitive integer types (actual language primitives)
        "std::primitive::i8" | "core::primitive::i8" | "std::i8" | "core::i8" => Some("i8"),
        "std::primitive::i16" | "core::primitive::i16" | "std::i16" | "core::i16" => Some("i16"),
        "std::primitive::i32" | "core::primitive::i32" | "std::i32" | "core::i32" => Some("i32"),
        "std::primitive::i64" | "core::primitive::i64" | "std::i64" | "core::i64" => Some("i64"),
        "std::primitive::i128" | "core::primitive::i128" | "std::i128" | "core::i128" => {
            Some("i128")
        }
        "std::primitive::isize" | "core::primitive::isize" | "std::isize" | "core::isize" => {
            Some("isize")
        }

        // Primitive unsigned integer types
        "std::primitive::u8" | "core::primitive::u8" | "std::u8" | "core::u8" => Some("u8"),
        "std::primitive::u16" | "core::primitive::u16" | "std::u16" | "core::u16" => Some("u16"),
        "std::primitive::u32" | "core::primitive::u32" | "std::u32" | "core::u32" => Some("u32"),
        "std::primitive::u64" | "core::primitive::u64" | "std::u64" | "core::u64" => Some("u64"),
        "std::primitive::u128" | "core::primitive::u128" | "std::u128" | "core::u128" => {
            Some("u128")
        }
        "std::primitive::usize" | "core::primitive::usize" | "std::usize" | "core::usize" => {
            Some("usize")
        }

        // Primitive floating point types
        "std::primitive::f32" | "core::primitive::f32" | "std::f32" | "core::f32" => Some("f32"),
        "std::primitive::f64" | "core::primitive::f64" | "std::f64" | "core::f64" => Some("f64"),

        // Other primitive types
        "std::primitive::bool" | "core::primitive::bool" | "std::bool" | "core::bool" => {
            Some("bool")
        }
        "std::primitive::char" | "core::primitive::char" | "std::char" | "core::char" => {
            Some("char")
        }
        "std::primitive::str" | "core::primitive::str" | "std::str" | "core::str" => Some("str"),

        _ => None,
    }
}

/// Rust prelude type mappings (automatically imported in every module)
#[cfg(feature = "static-resolver")]
fn get_prelude_mapping_static(path: &str) -> Option<&'static str> {
    match path {
        // String type
        "std::string::String" => Some("String"),

        // Vec type
        "std::vec::Vec" | "alloc::vec::Vec" => Some("Vec"),

        // Option type
        "std::option::Option" | "core::option::Option" => Some("Option"),

        // Result type
        "std::result::Result" => Some("Result"),

        // Box type
        "std::boxed::Box" | "alloc::boxed::Box" => Some("Box"),

        _ => None,
    }
}

/// Common std library type mappings (not in prelude but frequently used)
#[cfg(feature = "static-resolver")]
fn get_common_std_mapping_static(path: &str) -> Option<&'static str> {
    match path {
        // HashMap
        "std::collections::HashMap" | "std::collections::hash_map::HashMap" => Some("HashMap"),

        // HashSet
        "std::collections::HashSet" | "std::collections::hash_set::HashSet" => Some("HashSet"),

        // BTreeMap
        "std::collections::BTreeMap" | "std::collections::btree_map::BTreeMap" => Some("BTreeMap"),

        // BTreeSet
        "std::collections::BTreeSet" | "std::collections::btree_set::BTreeSet" => Some("BTreeSet"),

        // LinkedList
        "std::collections::LinkedList" | "std::collections::linked_list::LinkedList" => {
            Some("LinkedList")
        }

        // Cow
        "std::borrow::Cow" => Some("Cow"),

        // RefCell
        "std::cell::RefCell" | "core::cell::RefCell" => Some("RefCell"),

        // Arc
        "std::sync::Arc" | "alloc::sync::Arc" => Some("Arc"),

        // Rc
        "std::rc::Rc" | "alloc::rc::Rc" => Some("Rc"),

        _ => None,
    }
}

/// Fallback built-in type resolution when PHF is not available
#[cfg(not(feature = "static-resolver"))]
pub(crate) fn get_builtin_mapping(path: &str, groups: TypeGroups) -> Option<&'static str> {
    // Try primitives first if enabled
    if groups.primitives {
        if let Some(result) = get_primitive_mapping(path) {
            return Some(result);
        }
    }

    // Try prelude types if enabled
    if groups.prelude {
        if let Some(result) = get_prelude_mapping(path) {
            return Some(result);
        }
    }

    // Try common std types if enabled
    if groups.common_std {
        if let Some(result) = get_common_std_mapping(path) {
            return Some(result);
        }
    }

    None
}

/// Rust language primitive type mappings (non-static fallback)
#[cfg(not(feature = "static-resolver"))]
fn get_primitive_mapping(path: &str) -> Option<&'static str> {
    match path {
        // Primitive integer types (actual language primitives)
        "std::primitive::i8" | "core::primitive::i8" | "std::i8" | "core::i8" => Some("i8"),
        "std::primitive::i16" | "core::primitive::i16" | "std::i16" | "core::i16" => Some("i16"),
        "std::primitive::i32" | "core::primitive::i32" | "std::i32" | "core::i32" => Some("i32"),
        "std::primitive::i64" | "core::primitive::i64" | "std::i64" | "core::i64" => Some("i64"),
        "std::primitive::i128" | "core::primitive::i128" | "std::i128" | "core::i128" => {
            Some("i128")
        }
        "std::primitive::isize" | "core::primitive::isize" | "std::isize" | "core::isize" => {
            Some("isize")
        }

        // Primitive unsigned integer types
        "std::primitive::u8" | "core::primitive::u8" | "std::u8" | "core::u8" => Some("u8"),
        "std::primitive::u16" | "core::primitive::u16" | "std::u16" | "core::u16" => Some("u16"),
        "std::primitive::u32" | "core::primitive::u32" | "std::u32" | "core::u32" => Some("u32"),
        "std::primitive::u64" | "core::primitive::u64" | "std::u64" | "core::u64" => Some("u64"),
        "std::primitive::u128" | "core::primitive::u128" | "std::u128" | "core::u128" => {
            Some("u128")
        }
        "std::primitive::usize" | "core::primitive::usize" | "std::usize" | "core::usize" => {
            Some("usize")
        }

        // Primitive floating point types
        "std::primitive::f32" | "core::primitive::f32" | "std::f32" | "core::f32" => Some("f32"),
        "std::primitive::f64" | "core::primitive::f64" | "std::f64" | "core::f64" => Some("f64"),

        // Other primitive types
        "std::primitive::bool" | "core::primitive::bool" | "std::bool" | "core::bool" => {
            Some("bool")
        }
        "std::primitive::char" | "core::primitive::char" | "std::char" | "core::char" => {
            Some("char")
        }
        "std::primitive::str" | "core::primitive::str" | "std::str" | "core::str" => Some("str"),

        _ => None,
    }
}

/// Rust prelude type mappings (non-static fallback)
#[cfg(not(feature = "static-resolver"))]
fn get_prelude_mapping(path: &str) -> Option<&'static str> {
    match path {
        // String type
        "std::string::String" => Some("String"),

        // Vec type
        "std::vec::Vec" | "alloc::vec::Vec" => Some("Vec"),

        // Option type
        "std::option::Option" | "core::option::Option" => Some("Option"),

        // Result type
        "std::result::Result" => Some("Result"),

        // Box type
        "std::boxed::Box" | "alloc::boxed::Box" => Some("Box"),

        _ => None,
    }
}

/// Common std library type mappings (non-static fallback)
#[cfg(not(feature = "static-resolver"))]
fn get_common_std_mapping(path: &str) -> Option<&'static str> {
    match path {
        // HashMap
        "std::collections::HashMap" | "std::collections::hash_map::HashMap" => Some("HashMap"),

        // HashSet
        "std::collections::HashSet" | "std::collections::hash_set::HashSet" => Some("HashSet"),

        // BTreeMap
        "std::collections::BTreeMap" | "std::collections::btree_map::BTreeMap" => Some("BTreeMap"),

        // BTreeSet
        "std::collections::BTreeSet" | "std::collections::btree_set::BTreeSet" => Some("BTreeSet"),

        // LinkedList
        "std::collections::LinkedList" | "std::collections::linked_list::LinkedList" => {
            Some("LinkedList")
        }

        // Cow
        "std::borrow::Cow" => Some("Cow"),

        // RefCell
        "std::cell::RefCell" | "core::cell::RefCell" => Some("RefCell"),

        // Arc
        "std::sync::Arc" | "alloc::sync::Arc" => Some("Arc"),

        // Rc
        "std::rc::Rc" | "alloc::rc::Rc" => Some("Rc"),

        _ => None,
    }
}
