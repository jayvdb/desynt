use desynt::{EMPTY_RESOLVER, EmptyStorage, PRIMITIVE_RESOLVER, PathResolver};
use rstest::rstest;
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

#[rstest]
#[case::std_i8("std::primitive::i8", "i8")]
#[case::core_i16("core::primitive::i16", "i16")]
#[case::std_i32("std::i32", "i32")]
#[case::core_i64("core::i64", "i64")]
#[case::std_u8("std::primitive::u8", "u8")]
#[case::core_u16("core::primitive::u16", "u16")]
#[case::std_u32("std::u32", "u32")]
#[case::core_u64("core::u64", "u64")]
#[case::std_f32("std::primitive::f32", "f32")]
#[case::core_f64("core::primitive::f64", "f64")]
#[case::std_bool("std::primitive::bool", "bool")]
#[case::core_char("core::primitive::char", "char")]
#[case::std_str("std::primitive::str", "str")]
#[case::std_string("std::string::String", "String")]
#[case::std_vec("std::vec::Vec", "Vec")]
#[case::std_option("std::option::Option", "Option")]
#[case::std_result("std::result::Result", "Result")]
fn primitive_type_resolution(#[case] path_str: &str, #[case] expected: &str) {
    const RESOLVER: PathResolver<EmptyStorage> = PathResolver::primitives_only();

    let path: Path = parse_str(path_str).unwrap();
    assert_eq!(
        RESOLVER.resolve(&path),
        Some(expected),
        "Failed to resolve {} to {}",
        path_str,
        expected
    );
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
