use desynt::DynamicPathResolver;
use syn::{Path, parse_str};

#[test]
fn empty_resolver() {
    let resolver = DynamicPathResolver::default();
    assert!(resolver.is_empty());
    assert_eq!(resolver.len(), 0);

    let path: Path = parse_str("std::string::String").unwrap();
    assert!(resolver.resolve(&path).is_none());
}

#[test]
fn basic_mapping() {
    let mut resolver = DynamicPathResolver::default();
    resolver.add_mapping("std::string::String", "String");

    let path: Path = parse_str("std::string::String").unwrap();
    assert_eq!(resolver.resolve(&path), Some("String"));
}

#[test]
fn multiple_mappings() {
    let mut resolver = DynamicPathResolver::default();
    resolver.add_mapping("std::string::String", "String");
    resolver.add_mapping("std::vec::Vec", "Vec");

    let string_path: Path = parse_str("std::string::String").unwrap();
    let vec_path: Path = parse_str("std::vec::Vec").unwrap();

    assert_eq!(resolver.resolve(&string_path), Some("String"));
    assert_eq!(resolver.resolve(&vec_path), Some("Vec"));
}

#[test]
fn overwrite_mapping() {
    let mut resolver = DynamicPathResolver::default();
    resolver.add_mapping("std::string::String", "FirstString");
    resolver.add_mapping("std::string::String", "SecondString");

    let path: Path = parse_str("std::string::String").unwrap();
    assert_eq!(resolver.resolve(&path), Some("SecondString"));
}

#[test]
fn leading_colon_normalization() {
    let mut resolver = DynamicPathResolver::default();
    resolver.add_mapping("std::string::String", "String");

    let path1: Path = parse_str("std::string::String").unwrap();
    let path2: Path = parse_str("::std::string::String").unwrap();

    assert_eq!(resolver.resolve(&path1), Some("String"));
    assert_eq!(resolver.resolve(&path2), Some("String"));
}

#[test]
fn raw_identifier_normalization() {
    let mut resolver = DynamicPathResolver::default();
    resolver.add_mapping("std::string::String", "String");

    let normal_path: Path = parse_str("std::string::String").unwrap();
    let raw_path: Path = parse_str("r#std::r#string::String").unwrap();

    assert_eq!(resolver.resolve(&normal_path), Some("String"));
    assert_eq!(resolver.resolve(&raw_path), Some("String"));
}

#[test]
fn primitive_types_disabled() {
    let resolver = DynamicPathResolver::default();
    assert!(!resolver.uses_primitives());

    let path: Path = parse_str("std::primitive::i32").unwrap();
    assert!(resolver.resolve(&path).is_none());
}

#[test]
fn primitive_types_enabled() {
    let resolver = DynamicPathResolver::with_primitives();
    assert!(resolver.uses_primitives());

    let path: Path = parse_str("std::primitive::i32").unwrap();
    assert_eq!(resolver.resolve(&path), Some("i32"));
}

#[test]
fn set_use_primitives() {
    let mut resolver = DynamicPathResolver::default();
    assert!(!resolver.uses_primitives());

    resolver.set_use_primitives(true);
    assert!(resolver.uses_primitives());

    let path: Path = parse_str("std::primitive::f64").unwrap();
    assert_eq!(resolver.resolve(&path), Some("f64"));

    resolver.set_use_primitives(false);
    assert!(!resolver.uses_primitives());
    assert!(resolver.resolve(&path).is_none());
}

#[test]
fn has_mapping() {
    let mut resolver = DynamicPathResolver::default();
    resolver.add_mapping("custom::Type", "CustomType");

    let custom_path: Path = parse_str("custom::Type").unwrap();
    let unknown_path: Path = parse_str("unknown::Type").unwrap();

    assert!(resolver.has_mapping(&custom_path));
    assert!(!resolver.has_mapping(&unknown_path));
}

#[test]
fn has_mapping_with_primitives() {
    let resolver = DynamicPathResolver::with_primitives();

    let primitive_path: Path = parse_str("std::primitive::i32").unwrap();
    let unknown_path: Path = parse_str("unknown::Type").unwrap();

    assert!(resolver.has_mapping(&primitive_path));
    assert!(!resolver.has_mapping(&unknown_path));
}

#[test]
fn clear_mappings() {
    let mut resolver = DynamicPathResolver::default();
    resolver.add_mapping("custom::Type", "CustomType");

    assert_eq!(resolver.len(), 1);
    assert!(!resolver.is_empty());

    resolver.clear();

    assert_eq!(resolver.len(), 0);
    assert!(resolver.is_empty());
}

#[test]
fn from_map() {
    let mut mappings = std::collections::HashMap::new();
    mappings.insert("custom::Type".to_string(), "CustomType".to_string());
    mappings.insert("another::Type".to_string(), "AnotherType".to_string());

    let resolver = DynamicPathResolver::from_map(mappings, false);

    assert_eq!(resolver.len(), 2);
    assert!(!resolver.uses_primitives());

    let path1: Path = parse_str("custom::Type").unwrap();
    let path2: Path = parse_str("another::Type").unwrap();

    assert_eq!(resolver.resolve(&path1), Some("CustomType"));
    assert_eq!(resolver.resolve(&path2), Some("AnotherType"));
}

#[test]
fn len_with_primitives() {
    let empty_resolver = DynamicPathResolver::default();
    let primitive_resolver = DynamicPathResolver::with_primitives();

    assert_eq!(empty_resolver.len(), 0);
    assert_eq!(primitive_resolver.len(), 74); // Number of primitive mappings
}
