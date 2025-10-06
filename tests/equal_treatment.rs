use desynt::DynamicPathResolver;
use syn::Path;

#[test]
fn test_equal_treatment_of_stdlib_and_custom_types() {
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add both stdlib and custom mappings
    resolver.add_mapping("std::option::Option", "StdOption");
    resolver.add_mapping("butane::AutoPk", "AutoPk");
    resolver.add_mapping("my::custom::Type", "CustomType");

    // Test that both stdlib and custom types get equal treatment for generics
    let test_cases = vec![
        // Standard library type - should work
        ("std::option::Option", Some("StdOption")),
        ("Option<T>", Some("StdOption")), // Single segment generic
        ("std::option::Option<T>", Some("StdOption")), // Full path with generic
        // Custom type 1 - should work exactly the same
        ("butane::AutoPk", Some("AutoPk")),
        ("AutoPk<i64>", Some("AutoPk")), // Single segment generic - this was the issue
        ("butane::AutoPk<i64>", Some("AutoPk")), // Full path with generic
        // Custom type 2 - should also work the same
        ("my::custom::Type", Some("CustomType")),
        ("Type<String>", Some("CustomType")), // Single segment generic
        ("my::custom::Type<String>", Some("CustomType")), // Full path with generic
    ];

    for (input, expected) in test_cases {
        let path: Path = syn::parse_str(input).unwrap();
        let result = resolver.resolve(&path);
        assert_eq!(result, expected, "Failed for: {}", input);
        println!("✓ {} -> {:?}", input, result);
    }

    println!("\n✓ All types (stdlib and custom) are treated equally!");
}
