#![cfg(test)]

use std::collections::HashMap;

use desynt::{HasRaw, PathResolver, StripRaw, TypeGroups};
use syn::{Path, parse_str};

#[test]
fn strip_raw() {
    let raw_path: Path = parse_str("std::r#type::r#match").unwrap();
    let stripped = raw_path.strip_raw();

    let segments: Vec<String> = stripped
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect();

    assert_eq!(segments, vec!["std", "type", "match"]);
}

#[test]
fn has_raw() {
    let raw_path: Path = parse_str("std::r#type::normal").unwrap();
    let normal_path: Path = parse_str("std::normal::other").unwrap();

    assert!(raw_path.has_raw());
    assert!(!normal_path.has_raw());
}

#[test]
fn raw_identifier_in_module() {
    // Test raw identifier in a module path
    let path: Path = parse_str("r#custom::r#MyType").unwrap();

    let mut mappings = HashMap::new();
    mappings.insert("custom::MyType".to_string(), "CustomType".to_string());

    let resolver = PathResolver::new(mappings, TypeGroups::ALL);

    let result = resolver.resolve(&path);
    assert_eq!(result, Some("CustomType"));
}
