use desynt::PathResolver;
use syn::{Path, parse_str};

#[test]
fn empty_resolver() {
    let resolver = PathResolver::new();
    assert!(resolver.is_empty());
    assert_eq!(resolver.len(), 0);

    let path: Path = parse_str("std::string::String").unwrap();
    assert!(resolver.resolve(&path).is_none());
}

#[test]
fn basic_mapping() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::string::String", "String");

    let path: Path = parse_str("std::string::String").unwrap();
    assert_eq!(resolver.resolve(&path), Some("String"));

    assert!(!resolver.is_empty());
    assert_eq!(resolver.len(), 1);
}

#[test]
fn resolve_with_leading_colons() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::string::String", "String");

    let path: Path = parse_str("::std::string::String").unwrap();
    assert_eq!(resolver.resolve(&path), Some("String"));
}

#[test]
fn resolve_with_raw_identifiers() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::string::String", "String");

    let path: Path = parse_str("::r#std::r#string::String").unwrap();
    assert_eq!(resolver.resolve(&path), Some("String"));
}

#[test]
fn multiple_mappings_same_canonical() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::primitive::f64", "f64");
    resolver.add_mapping("core::primitive::f64", "f64");
    resolver.add_mapping("f64", "f64");

    let std_path: Path = parse_str("::std::primitive::f64").unwrap();
    let core_path: Path = parse_str("::core::primitive::f64").unwrap();
    let simple_path: Path = parse_str("f64").unwrap();

    assert_eq!(resolver.resolve(&std_path), Some("f64"));
    assert_eq!(resolver.resolve(&core_path), Some("f64"));
    assert_eq!(resolver.resolve(&simple_path), Some("f64"));
    assert_eq!(resolver.len(), 3);
}

#[test]
fn primitives_resolver() {
    let resolver = PathResolver::with_primitives();
    assert!(!resolver.is_empty());

    // Test primitive types
    let f64_std: Path = parse_str("::std::primitive::f64").unwrap();
    let f64_core: Path = parse_str("::core::primitive::f64").unwrap();
    let i32_std: Path = parse_str("std::i32").unwrap();

    assert_eq!(resolver.resolve(&f64_std), Some("f64"));
    assert_eq!(resolver.resolve(&f64_core), Some("f64"));
    assert_eq!(resolver.resolve(&i32_std), Some("i32"));

    // Test common std types
    let string_path: Path = parse_str("std::string::String").unwrap();
    let vec_path: Path = parse_str("std::vec::Vec").unwrap();

    assert_eq!(resolver.resolve(&string_path), Some("String"));
    assert_eq!(resolver.resolve(&vec_path), Some("Vec"));
}

#[test]
fn has_mapping() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::string::String", "String");

    let mapped_path: Path = parse_str("std::string::String").unwrap();
    let unmapped_path: Path = parse_str("std::vec::Vec").unwrap();

    assert!(resolver.has_mapping(&mapped_path));
    assert!(!resolver.has_mapping(&unmapped_path));
}

#[test]
fn canonical_types_and_patterns() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::string::String", "String");
    resolver.add_mapping("std::vec::Vec", "Vec");
    resolver.add_mapping("core::option::Option", "Option");

    let canonical_types: Vec<&str> = resolver.canonical_types().collect();
    let path_patterns: Vec<&str> = resolver.path_patterns().collect();

    assert_eq!(canonical_types.len(), 3);
    assert_eq!(path_patterns.len(), 3);

    assert!(canonical_types.contains(&"String"));
    assert!(canonical_types.contains(&"Vec"));
    assert!(canonical_types.contains(&"Option"));

    assert!(path_patterns.contains(&"std::string::String"));
    assert!(path_patterns.contains(&"std::vec::Vec"));
    assert!(path_patterns.contains(&"core::option::Option"));
}

#[test]
fn clear_mappings() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::string::String", "String");
    resolver.add_mapping("std::vec::Vec", "Vec");

    assert_eq!(resolver.len(), 2);
    assert!(!resolver.is_empty());

    resolver.clear();

    assert_eq!(resolver.len(), 0);
    assert!(resolver.is_empty());
}

#[test]
fn complex_raw_identifier_resolution() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("my::custom::Type", "CustomType");

    let complex_path: Path = parse_str("::r#my::r#custom::r#Type").unwrap();
    assert_eq!(resolver.resolve(&complex_path), Some("CustomType"));
}

#[test]
fn case_sensitive_mapping() {
    let mut resolver = PathResolver::new();
    resolver.add_mapping("std::string::String", "String");
    resolver.add_mapping("std::string::string", "string");

    let upper_path: Path = parse_str("std::string::String").unwrap();
    let lower_path: Path = parse_str("std::string::string").unwrap();

    assert_eq!(resolver.resolve(&upper_path), Some("String"));
    assert_eq!(resolver.resolve(&lower_path), Some("string"));
}

#[test]
fn no_mapping_returns_none() {
    let resolver = PathResolver::new();

    let unmapped_path: Path = parse_str("some::unknown::Path").unwrap();
    assert_eq!(resolver.resolve(&unmapped_path), None);
    assert!(!resolver.has_mapping(&unmapped_path));
}

#[test]
fn default_resolver() {
    let resolver = PathResolver::default();
    assert!(resolver.is_empty());
    assert_eq!(resolver.len(), 0);
}
