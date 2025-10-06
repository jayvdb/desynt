use desynt::DynamicPathResolver;
use syn::Path;

#[test]
fn test_progressive_path_resolution() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // with_primitives() already includes stdlib mappings
    // Add only custom mappings for testing
    resolver.add_mapping("my::custom::Type", "CustomType");

    let test_cases = vec![
        // Case 1: Direct mapping should work
        ("std::option::Option", Some("Option")),
        ("std::vec::Vec", Some("Vec")),
        // Case 2: Generic types should resolve to base type
        ("std::option::Option<String>", Some("Option")),
        ("std::vec::Vec<i32>", Some("Vec")),
        ("std::collections::HashMap<String, i32>", Some("HashMap")),
        // Case 3: Progressive path shortening for single base types
        ("Option<butane::ForeignKey<Foo>>", Some("Option")),
        ("Vec<String>", Some("Vec")),
        ("HashMap<K, V>", Some("HashMap")),
        // Case 4: Progressive path shortening for qualified paths
        ("option::Option<T>", Some("Option")),
        ("vec::Vec<T>", Some("Vec")),
        ("collections::HashMap<K, V>", Some("HashMap")),
        // Case 5: Nested generics should resolve to outermost type
        ("Option<Vec<HashMap<String, i32>>>", Some("Option")),
        ("Vec<Option<String>>", Some("Vec")),
        // Case 6: Should NOT resolve arbitrary types with same suffix
        ("unknown::Type", None), // Should NOT resolve to "CustomType"
        ("other::Option", None), // Should NOT resolve because not std library
        // Case 7: Custom types should work normally
        ("my::custom::Type", Some("CustomType")),
        ("my::custom::Type<T>", Some("CustomType")),
    ];

    for (input, expected) in test_cases {
        let path: Path = syn::parse_str(input).unwrap();
        let result = resolver.resolve(&path);
        assert_eq!(result, expected, "Failed for: {}", input);
        println!("✓ {} -> {:?}", input, result);
    }
}

#[test]
fn test_conservative_suffix_matching() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add mappings that could create false positives
    resolver.add_mapping("std::option::Option", "StdOption");
    resolver.add_mapping("my::special::Option", "MyOption");
    resolver.add_mapping("another::Type", "AnotherType");
    resolver.add_mapping("my::special::Type", "SpecialType");

    let test_cases = vec![
        // These should work because they follow std library patterns
        ("Option<T>", Some("StdOption")), // Should find std::option::Option
        ("option::Option<T>", Some("StdOption")), // Should find std::option::Option
        // These should NOT create false positives
        ("unknown::Option<T>", None), // Should NOT resolve to MyOption
        ("random::Type<T>", None),    // Should NOT resolve to AnotherType or SpecialType
        ("other::Option", None),      // Should NOT resolve to any Option mapping
        // Exact matches should still work
        ("my::special::Option", Some("MyOption")),
        ("my::special::Type", Some("SpecialType")),
        ("another::Type", Some("AnotherType")),
        // Generics of exact matches should work
        ("my::special::Option<T>", Some("MyOption")),
        ("my::special::Type<T>", Some("SpecialType")),
        ("another::Type<T>", Some("AnotherType")),
    ];

    for (input, expected) in test_cases {
        let path: Path = syn::parse_str(input).unwrap();
        let result = resolver.resolve(&path);
        assert_eq!(result, expected, "Failed for: {}", input);
        println!("✓ {} -> {:?}", input, result);
    }
}
