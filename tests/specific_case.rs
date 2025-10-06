use desynt::DynamicPathResolver;
use syn::Path;

#[test]
fn test_specific_requested_case() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add mapping for Option (the user wants this to resolve to Some("Option"))
    resolver.add_mapping("Option", "Option");

    // Test the exact case from the user request
    let path_str = "Option<butane::ForeignKey<Foo>>";
    let path: Path = syn::parse_str(path_str).unwrap();

    let result = resolver.resolve(&path);

    // This should now resolve to Some("Option") as requested
    assert_eq!(result, Some("Option"));
    println!("✓ {} resolves to {:?}", path_str, result);
}

#[test]
fn test_more_realistic_cases() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add mappings that would be more typical in real code
    resolver.add_mapping("std::option::Option", "Option");
    resolver.add_mapping("std::vec::Vec", "Vec");
    resolver.add_mapping("std::collections::HashMap", "HashMap");

    // Also add short-form mappings for convenience
    resolver.add_mapping("Option", "Option");
    resolver.add_mapping("Vec", "Vec");
    resolver.add_mapping("HashMap", "HashMap");

    let test_cases = vec![
        // Simple generic types (using short form mappings)
        ("Option<String>", "Option"),
        ("Vec<i32>", "Vec"),
        ("HashMap<String, i32>", "HashMap"),
        // Nested generic types
        ("Option<Vec<String>>", "Option"),
        ("Vec<Option<i32>>", "Vec"),
        // Complex nested types like the user's case
        ("Option<butane::ForeignKey<User>>", "Option"),
        ("Vec<diesel::result::QueryResult<User>>", "Vec"),
        // Fully qualified paths with generics
        ("std::option::Option<String>", "Option"),
        ("std::vec::Vec<User>", "Vec"),
        ("std::collections::HashMap<String, Value>", "HashMap"),
    ];

    for (input, expected) in test_cases {
        let path: Path = syn::parse_str(input).unwrap();
        let result = resolver.resolve(&path);
        assert_eq!(result, Some(expected), "Failed for: {}", input);
        println!("✓ {} -> {}", input, expected);
    }
}
