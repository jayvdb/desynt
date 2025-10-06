use desynt::DynamicPathResolver;
use syn::Path;

fn main() {
    println!("=== Generic Type Resolution Demo ===");
    println!();

    let resolver = DynamicPathResolver::with_primitives();

    // No need to add explicit stdlib mappings - with_primitives() includes:
    // - std::option::Option -> Option
    // - std::vec::Vec -> Vec
    // - std::collections::HashMap -> HashMap
    // - std::result::Result -> Result
    // (and many more primitive and stdlib types)

    // Test cases that should now work automatically
    let test_cases = vec![
        "Option<butane::ForeignKey<Foo>>", // Should resolve via "Option"
        "Vec<String>",                     // Should resolve via "Vec"
        "HashMap<String, i32>",            // Should resolve via "HashMap"
        "Result<User, DatabaseError>",     // Should resolve via "Result"
        "Option<Vec<HashMap<String, Value>>>", // Should resolve via "Option"
        "std::option::Option<String>",     // Should resolve via "std::option::Option"
        "option::Option<i32>", // Should resolve via "option::Option" -> "std::option::Option"
        "vec::Vec<String>",    // Should resolve via "vec::Vec" -> "std::vec::Vec"
        "collections::HashMap<K, V>", // Should resolve via "collections::HashMap" -> "std::collections::HashMap"
    ];

    println!("Testing generic type resolution:");
    for case in &test_cases {
        let path: Path = syn::parse_str(case).unwrap();
        let result = resolver.resolve(&path);
        println!("  {} -> {:?}", case, result);
    }

    println!();
    println!("✓ The resolver automatically handles path variations!");
    println!(
        "✓ Only need to map 'std::option::Option' to handle 'Option<T>' and 'option::Option<T>'"
    );

    // Demonstrate the specific requested case
    let specific_case = "Option<butane::ForeignKey<Foo>>";
    let path: Path = syn::parse_str(specific_case).unwrap();
    let result = resolver.resolve(&path);
    assert_eq!(result, Some("Option"));
    println!("✓ Assertion passed: {} == Some(\"Option\")", specific_case);
}
