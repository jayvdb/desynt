use desynt::{EMPTY_RESOLVER, EmptyStorage, PRIMITIVE_RESOLVER, PathResolver};
use syn::{Path, parse_str};

#[test]
fn const_empty_resolver() {
    const RESOLVER: PathResolver<EmptyStorage> = PathResolver::empty();

    assert!(RESOLVER.is_empty());
    assert_eq!(RESOLVER.len(), 0);
    assert!(!RESOLVER.uses_primitives());

    let path: Path = parse_str("std::string::String").unwrap();
    assert!(RESOLVER.resolve(&path).is_none());
}

#[test]
fn const_primitive_resolver() {
    const RESOLVER: PathResolver<EmptyStorage> = PathResolver::primitives_only();

    assert!(!RESOLVER.is_empty()); // has primitives
    assert_eq!(RESOLVER.len(), 74); // number of primitive mappings
    assert!(RESOLVER.uses_primitives());

    let path: Path = parse_str("std::primitive::i32").unwrap();
    assert_eq!(RESOLVER.resolve(&path), Some("i32"));
}

#[test]
fn global_primitive_resolver() {
    assert!(!PRIMITIVE_RESOLVER.is_empty());
    assert_eq!(PRIMITIVE_RESOLVER.len(), 74);
    assert!(PRIMITIVE_RESOLVER.uses_primitives());

    let path: Path = parse_str("std::primitive::f64").unwrap();
    assert_eq!(PRIMITIVE_RESOLVER.resolve(&path), Some("f64"));
}

#[test]
fn global_empty_resolver() {
    assert!(EMPTY_RESOLVER.is_empty());
    assert_eq!(EMPTY_RESOLVER.len(), 0);
    assert!(!EMPTY_RESOLVER.uses_primitives());

    let path: Path = parse_str("std::primitive::f64").unwrap();
    assert!(EMPTY_RESOLVER.resolve(&path).is_none());
}

#[test]
fn primitive_type_resolution() {
    const RESOLVER: PathResolver<EmptyStorage> = PathResolver::primitives_only();

    let test_cases = [
        ("std::primitive::i8", "i8"),
        ("core::primitive::i16", "i16"),
        ("std::i32", "i32"),
        ("core::i64", "i64"),
        ("std::primitive::u8", "u8"),
        ("core::primitive::u16", "u16"),
        ("std::u32", "u32"),
        ("core::u64", "u64"),
        ("std::primitive::f32", "f32"),
        ("core::primitive::f64", "f64"),
        ("std::primitive::bool", "bool"),
        ("core::primitive::char", "char"),
        ("std::primitive::str", "str"),
        ("std::string::String", "String"),
        ("std::vec::Vec", "Vec"),
        ("std::option::Option", "Option"),
        ("std::result::Result", "Result"),
    ];

    for (path_str, expected) in &test_cases {
        let path: Path = parse_str(path_str).unwrap();
        assert_eq!(
            RESOLVER.resolve(&path),
            Some(*expected),
            "Failed to resolve {} to {}",
            path_str,
            expected
        );
    }
}

#[test]
fn const_resolver_with_runtime_usage() {
    // This test shows that const resolvers can be used in runtime contexts
    let mut runtime_resolver = desynt::DynamicPathResolver::default();
    runtime_resolver.add_mapping("custom::Type", "CustomType");

    // Compare const and runtime resolvers
    let primitive_path: Path = parse_str("std::primitive::i32").unwrap();
    let custom_path: Path = parse_str("custom::Type").unwrap();

    // Const resolver handles primitives
    assert_eq!(PRIMITIVE_RESOLVER.resolve(&primitive_path), Some("i32"));
    assert!(PRIMITIVE_RESOLVER.resolve(&custom_path).is_none());

    // Runtime resolver handles custom types but not primitives (unless enabled)
    assert!(runtime_resolver.resolve(&primitive_path).is_none());
    assert_eq!(runtime_resolver.resolve(&custom_path), Some("CustomType"));
}

#[test]
fn const_resolver_immutability() {
    // Const resolvers are immutable - this test just verifies they work in const contexts
    const RESOLVER1: PathResolver<EmptyStorage> = PathResolver::empty();
    const RESOLVER2: PathResolver<EmptyStorage> = PathResolver::primitives_only();

    // These should compile without issues
    assert_ne!(RESOLVER1.len(), RESOLVER2.len());
    assert_ne!(RESOLVER1.uses_primitives(), RESOLVER2.uses_primitives());
}
