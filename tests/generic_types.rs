use desynt::DynamicPathResolver;
use syn::Path;

#[test]
fn test_generic_type_resolution() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add mapping for Option - try both variations
    resolver.add_mapping("std::option::Option", "Option");
    resolver.add_mapping("Option", "Option");

    // Test parsing a path with generic arguments
    let path_str = "Option<butane::ForeignKey<Foo>>";
    let path: Path = syn::parse_str(path_str).unwrap();

    println!("Original path: {}", path_str);
    println!(
        "Parsed path segments: {:?}",
        path.segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
    );

    // This should resolve to "Option" now
    let result = resolver.resolve(&path);
    println!("Resolver result: {:?}", result);

    // It should now resolve to Some("Option")
    assert_eq!(result, Some("Option"));
}

#[test]
fn test_complex_generic_types() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add mappings for base types
    resolver.add_mapping("Vec", "Vec");
    resolver.add_mapping("HashMap", "HashMap");
    resolver.add_mapping("Option", "Option");

    let test_cases = vec![
        ("Vec<String>", Some("Vec")),
        ("HashMap<String, i32>", Some("HashMap")),
        ("Option<Vec<HashMap<String, i32>>>", Some("Option")),
        ("std::option::Option<std::vec::Vec<String>>", Some("Option")),
    ];

    for (case, expected) in test_cases {
        let path: Path = syn::parse_str(case).unwrap();
        let result = resolver.resolve(&path);
        println!("Testing: {} -> {:?}", case, result);
        assert_eq!(result, expected, "Failed for case: {}", case);
    }
}

#[test]
fn test_generic_resolution_strategies() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Test different resolution strategies

    // Strategy 1: Full path resolution (existing behavior)
    resolver.add_mapping("std::option::Option", "Option");

    let path1: Path = syn::parse_str("std::option::Option").unwrap();
    assert_eq!(resolver.resolve(&path1), Some("Option"));

    // Strategy 2: Base type resolution for generic types
    let path2: Path = syn::parse_str("std::option::Option<String>").unwrap();
    assert_eq!(resolver.resolve(&path2), Some("Option")); // Should find via full path reconstruction

    // Strategy 3: Short base type resolution
    resolver.add_mapping("Result", "Result");
    let path3: Path = syn::parse_str("Result<String, Error>").unwrap();
    assert_eq!(resolver.resolve(&path3), Some("Result")); // Should find via base type lookup

    // Strategy 4: No mapping found
    let path4: Path = syn::parse_str("UnknownType<T>").unwrap();
    assert_eq!(resolver.resolve(&path4), None);
}
