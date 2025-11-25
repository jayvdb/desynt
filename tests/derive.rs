//! Integration tests for derive macro scenarios
#![cfg(test)]

use std::collections::HashMap;

use desynt::{PathResolver, TypeGroups};
use syn::{DeriveInput, Type, TypePath, parse_str};

#[test]
fn simple_struct() {
    // This test simulates the exact scenario from the user's issue:
    // Type aliases with raw identifiers used in a struct

    // Create type aliases mapping
    let mut mappings = HashMap::new();
    mappings.insert("MyOption".to_string(), "Option".to_string());
    mappings.insert("MyVec".to_string(), "Vec".to_string());
    mappings.insert("MyHashMap".to_string(), "HashMap".to_string());

    let resolver = PathResolver::new(mappings, TypeGroups::ALL);

    // Parse the struct definition
    let input: DeriveInput = parse_str(
        r#"
        struct GenericTestStruct {
            optional_field: r#MyOption<String>,
            list_field: r#MyVec<i32>,
            map_field: r#MyHashMap<String, bool>,
            normal_option: Option<String>,
        }
    "#,
    )
    .unwrap();

    // Extract and verify each field type
    if let syn::Data::Struct(data) = input.data {
        if let syn::Fields::Named(fields) = data.fields {
            for field in fields.named {
                let field_name = field.ident.unwrap().to_string();

                if let Type::Path(TypePath { path, .. }) = field.ty {
                    let resolved = resolver.resolve(&path);

                    match field_name.as_str() {
                        "optional_field" => {
                            assert_eq!(
                                resolved,
                                Some("Option"),
                                "r#MyOption<String> should resolve to Option"
                            );
                        }
                        "list_field" => {
                            assert_eq!(resolved, Some("Vec"), "r#MyVec<i32> should resolve to Vec");
                        }
                        "map_field" => {
                            assert_eq!(
                                resolved,
                                Some("HashMap"),
                                "r#MyHashMap<String, bool> should resolve to HashMap"
                            );
                        }
                        "normal_option" => {
                            assert_eq!(
                                resolved,
                                Some("Option"),
                                "Option<String> should resolve to Option"
                            );
                        }
                        _ => panic!("Unexpected field: {}", field_name),
                    }
                }
            }
        }
    }
}

#[test]
fn with_macro() {
    // Test extracting fields from a derive macro context
    let code = r#"
        #[derive(ToSchema)]
        struct GenericTestStruct {
            optional_field: r#MyOption<String>,
            list_field: r#MyVec<i32>,
            map_field: r#MyHashMap<String, bool>,
            normal_option: Option<String>,
        }
    "#;

    let input: DeriveInput = parse_str(code).unwrap();

    let mut mappings = HashMap::new();
    mappings.insert("MyOption".to_string(), "Option".to_string());
    mappings.insert("MyVec".to_string(), "Vec".to_string());
    mappings.insert("MyHashMap".to_string(), "HashMap".to_string());

    let resolver = PathResolver::new(mappings, TypeGroups::ALL);

    if let syn::Data::Struct(data_struct) = input.data {
        if let syn::Fields::Named(fields_named) = data_struct.fields {
            let field_types: Vec<_> = fields_named
                .named
                .iter()
                .filter_map(|field| {
                    if let Type::Path(type_path) = &field.ty {
                        Some((
                            field.ident.as_ref().unwrap().to_string(),
                            resolver.resolve(&type_path.path),
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            assert_eq!(field_types.len(), 4);

            for (name, resolved) in &field_types {
                match name.as_str() {
                    "optional_field" => assert_eq!(*resolved, Some("Option")),
                    "list_field" => assert_eq!(*resolved, Some("Vec")),
                    "map_field" => assert_eq!(*resolved, Some("HashMap")),
                    "normal_option" => assert_eq!(*resolved, Some("Option")),
                    _ => panic!("Unexpected field: {}", name),
                }
            }
        }
    }
}
