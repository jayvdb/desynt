use desynt::{PathResolver, create_static_resolver};
use phf::{Map, phf_map};
use rstest::rstest;
use syn::{Path, parse_str};

// Test PHF mappings
static TEST_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
    "custom::Type1" => "Type1",
    "custom::Type2" => "Type2",
    "another::Custom" => "AnotherCustom",
    "my::special::Type" => "SpecialType",
};

type StaticPathResolver = PathResolver<&'static Map<&'static str, &'static str>>;

#[test]
fn static_resolver_basic() {
    const RESOLVER: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, false);

    assert_eq!(RESOLVER.len(), 4);
    assert!(!RESOLVER.uses_primitives());
    assert!(!RESOLVER.is_empty());

    let path: Path = parse_str("custom::Type1").unwrap();
    assert_eq!(RESOLVER.resolve(&path), Some("Type1"));
}

#[test]
fn static_resolver_empty() {
    static EMPTY_MAPPINGS: Map<&'static str, &'static str> = phf_map! {};
    const RESOLVER: StaticPathResolver = create_static_resolver(&EMPTY_MAPPINGS, false);

    assert_eq!(RESOLVER.len(), 0);
    assert!(!RESOLVER.uses_primitives());
    assert!(RESOLVER.is_empty());

    let path: Path = parse_str("custom::Type1").unwrap();
    assert!(RESOLVER.resolve(&path).is_none());
}

#[test]
fn static_resolver_with_primitives() {
    static EMPTY_MAPPINGS: Map<&'static str, &'static str> = phf_map! {};
    const RESOLVER: StaticPathResolver = create_static_resolver(&EMPTY_MAPPINGS, true);

    assert_eq!(RESOLVER.len(), 74); // only primitives
    assert!(RESOLVER.uses_primitives());
    assert!(!RESOLVER.is_empty());

    let path: Path = parse_str("std::primitive::i32").unwrap();
    assert_eq!(RESOLVER.resolve(&path), Some("i32"));
}

#[test]
fn static_resolver_custom_and_primitives() {
    const RESOLVER: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, true);

    assert_eq!(RESOLVER.len(), 4 + 74); // custom + primitives
    assert!(RESOLVER.uses_primitives());
    assert!(!RESOLVER.is_empty());

    // Test custom mapping
    let custom_path: Path = parse_str("custom::Type1").unwrap();
    assert_eq!(RESOLVER.resolve(&custom_path), Some("Type1"));

    // Test primitive mapping
    let primitive_path: Path = parse_str("std::primitive::i32").unwrap();
    assert_eq!(RESOLVER.resolve(&primitive_path), Some("i32"));
}

#[rstest]
#[case::custom_type1("custom::Type1", Some("Type1"))]
#[case::custom_type2("custom::Type2", Some("Type2"))]
#[case::another_custom("another::Custom", Some("AnotherCustom"))]
#[case::my_special_type("my::special::Type", Some("SpecialType"))]
#[case::unknown_type("unknown::Type", None)]
fn static_resolver_all_mappings(#[case] path_str: &str, #[case] expected: Option<&str>) {
    const RESOLVER: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, false);

    let path: Path = parse_str(path_str).unwrap();
    assert_eq!(
        RESOLVER.resolve(&path),
        expected,
        "Failed to resolve {} correctly",
        path_str
    );
}

#[test]
fn static_resolver_has_mapping() {
    const RESOLVER: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, true);

    let custom_path: Path = parse_str("custom::Type1").unwrap();
    let primitive_path: Path = parse_str("std::primitive::i32").unwrap();
    let unknown_path: Path = parse_str("unknown::Type").unwrap();

    assert!(RESOLVER.has_mapping(&custom_path));
    assert!(RESOLVER.has_mapping(&primitive_path));
    assert!(!RESOLVER.has_mapping(&unknown_path));
}

#[test]
fn static_resolver_path_normalization() {
    const RESOLVER: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, false);

    let normal_path: Path = parse_str("custom::Type1").unwrap();
    let leading_colon_path: Path = parse_str("::custom::Type1").unwrap();
    let raw_path: Path = parse_str("r#custom::Type1").unwrap();

    assert_eq!(RESOLVER.resolve(&normal_path), Some("Type1"));
    assert_eq!(RESOLVER.resolve(&leading_colon_path), Some("Type1"));
    assert_eq!(RESOLVER.resolve(&raw_path), Some("Type1"));
}

#[test]
fn static_resolver_iterators() {
    const RESOLVER: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, false);

    let patterns: Vec<&str> = RESOLVER.path_patterns().collect();
    let types: Vec<&str> = RESOLVER.canonical_types().collect();

    assert_eq!(patterns.len(), 4);
    assert_eq!(types.len(), 4);

    assert!(patterns.contains(&"custom::Type1"));
    assert!(patterns.contains(&"my::special::Type"));
    assert!(types.contains(&"Type1"));
    assert!(types.contains(&"SpecialType"));
}

#[test]
fn static_resolver_const_creation() {
    // Test that static resolvers can be created in const contexts
    const RESOLVER1: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, true);
    const RESOLVER2: StaticPathResolver = create_static_resolver(&TEST_MAPPINGS, false);

    // These should compile and work at compile time
    assert_ne!(RESOLVER1.len(), RESOLVER2.len());
    assert_ne!(RESOLVER1.uses_primitives(), RESOLVER2.uses_primitives());

    // Runtime usage should also work
    let path: Path = parse_str("custom::Type1").unwrap();
    assert_eq!(RESOLVER1.resolve(&path), Some("Type1"));
    assert_eq!(RESOLVER2.resolve(&path), Some("Type1"));
}
