//! Integration tests for type alias scenarios
#![cfg(test)]

use std::collections::HashMap;

use desynt::{PathResolver, TypeGroups};
use syn::parse_str;

#[test]
fn definitions_with_raw_identifiers() {
    // Test the actual type alias definitions with raw identifiers

    // Parse type alias definitions (these would be in the actual code)
    let alias1: syn::ItemType = parse_str("type r#MyOption<T> = Option<T>;").unwrap();
    let alias2: syn::ItemType = parse_str("type r#MyVec<T> = Vec<T>;").unwrap();
    let alias3: syn::ItemType =
        parse_str("type r#MyHashMap<K, V> = std::collections::HashMap<K, V>;").unwrap();

    // Verify that the aliases are parsed correctly - syn preserves the r# prefix
    assert_eq!(alias1.ident.to_string(), "r#MyOption");
    assert_eq!(alias2.ident.to_string(), "r#MyVec");
    assert_eq!(alias3.ident.to_string(), "r#MyHashMap");

    // Now test resolution of these types when used
    let mut mappings = HashMap::new();
    mappings.insert("MyOption".to_string(), "Option".to_string());
    mappings.insert("MyVec".to_string(), "Vec".to_string());
    mappings.insert("MyHashMap".to_string(), "HashMap".to_string());

    let resolver = PathResolver::new(mappings, TypeGroups::ALL);

    // Test that using these type aliases (with raw identifiers) resolves correctly
    let usage1 = parse_str("r#MyOption<String>").unwrap();
    let usage2 = parse_str("r#MyVec<i32>").unwrap();
    let usage3 = parse_str("r#MyHashMap<String, bool>").unwrap();

    assert_eq!(resolver.resolve(&usage1), Some("Option"));
    assert_eq!(resolver.resolve(&usage2), Some("Vec"));
    assert_eq!(resolver.resolve(&usage3), Some("HashMap"));
}
