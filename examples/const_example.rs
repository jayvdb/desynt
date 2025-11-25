use desynt::{
    DynamicPathResolver, EmptyStorage, PRIMITIVE_RESOLVER, PathResolver, TypeGroups,
    create_static_resolver,
};
use phf::{Map, phf_map};

// Custom mappings for application-specific types using PHF
const CUSTOM_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
    // Web framework types
    "actix_web::HttpResponse" => "HttpResponse",
    "axum::response::Response" => "Response",
    "warp::reply::Reply" => "Reply",

    // Database types
    "sqlx::Row" => "DbRow",
    "diesel::QueryResult" => "QueryResult",
    "tokio_postgres::Row" => "PgRow",

    // Serialization types
    "serde_json::Value" => "JsonValue",
    "toml::Value" => "TomlValue",
    "yaml_rust::Yaml" => "YamlValue",

    // Async types
    "tokio::task::JoinHandle" => "JoinHandle",
    "futures::future::BoxFuture" => "BoxFuture",
    "async_std::task::Task" => "Task",

    // Error types
    "anyhow::Error" => "AnyError",
    "eyre::Report" => "ErrorReport",
    "thiserror::Error" => "ThisError",

    // Collection types
    "indexmap::IndexMap" => "IndexMap",
    "smallvec::SmallVec" => "SmallVec",
    "tinyvec::TinyVec" => "TinyVec",
};

// Create const resolvers using the unified PathResolver API
const PRIMITIVE_ONLY: PathResolver<EmptyStorage> = PathResolver::primitives_only();
const EMPTY_RESOLVER: PathResolver<EmptyStorage> = PathResolver::empty();

// Static resolver with custom mappings - truly const!
const FULL_STATIC_RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
    create_static_resolver(&CUSTOM_MAPPINGS, TypeGroups::ALL);
const CUSTOM_ONLY_RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
    create_static_resolver(&CUSTOM_MAPPINGS, TypeGroups::NONE);

fn main() {
    println!("=== Unified PathResolver Examples ===\n");

    // Test const PathResolver (primitives only)
    println!("1. Const Primitive Resolver:");
    test_resolver(&PRIMITIVE_ONLY, "Primitives Only");

    // Test empty resolver
    println!("\n2. Empty Const Resolver:");
    test_resolver(&EMPTY_RESOLVER, "Empty");

    // Test full static resolver with custom mappings
    println!("\n3. Full Static Resolver - TRULY CONST!");
    test_resolver(&FULL_STATIC_RESOLVER, "Full Static");

    // Test custom-only resolver
    println!("\n4. Custom-Only Static Resolver (no primitives):");
    test_resolver(&CUSTOM_ONLY_RESOLVER, "Custom Only");

    // Test global primitive resolver
    println!("\n5. Global Primitive Resolver:");
    test_resolver(&PRIMITIVE_RESOLVER, "Global Primitives");

    // Test dynamic resolver for comparison
    println!("\n6. Dynamic Resolver (for comparison):");
    test_dynamic_resolver();

    println!("\n7. Testing Raw Identifiers:");
    test_raw_identifiers();

    println!("\n=== Summary ===");
    println!("✓ Unified PathResolver API with generic storage");
    println!("✓ HashMap for dynamic runtime mappings");
    println!("✓ PHF maps for static compile-time mappings");
    println!("✓ EmptyStorage for const resolvers with only primitives");
    println!("✓ Zero runtime cost for static resolvers");
    println!("✓ O(1) lookup performance with PHF maps");
    println!("✓ Type safety and const guarantees");
}

fn test_resolver<M>(resolver: &PathResolver<M>, name: &str)
where
    M: desynt::MappingStorage,
{
    let test_paths = [
        "actix_web::HttpResponse",
        "std::primitive::i32",
        "serde_json::Value",
        "std::string::String",
        "anyhow::Error",
        "unknown::Type",
    ];

    println!("Testing {} resolver:", name);
    println!("  Total mappings: {}", resolver.len());
    println!("  Uses primitives: {}", resolver.uses_primitives());
    println!("  Is empty: {}", resolver.is_empty());

    for path_str in &test_paths {
        let path: syn::Path = syn::parse_str(path_str).unwrap();
        match resolver.resolve(&path) {
            Some(canonical) => println!("  {} -> {}", path_str, canonical),
            None => println!("  {} -> (not found)", path_str),
        }
    }
}

fn test_dynamic_resolver() {
    // Create dynamic resolver
    let mut resolver = DynamicPathResolver::with_primitives();

    // Add some custom mappings
    resolver.add_mapping("my::custom::Type", "MyType");
    resolver.add_mapping("another::Custom", "AnotherCustom");

    println!("Testing Dynamic resolver:");
    println!("  Total mappings: {}", resolver.len());
    println!("  Uses primitives: {}", resolver.uses_primitives());

    let test_paths = [
        "my::custom::Type",
        "std::primitive::i32",
        "another::Custom",
        "unknown::Type",
    ];

    for path_str in &test_paths {
        let path: syn::Path = syn::parse_str(path_str).unwrap();
        match resolver.resolve(&path) {
            Some(canonical) => println!("  {} -> {}", path_str, canonical),
            None => println!("  {} -> (not found)", path_str),
        }
    }

    // Test modification
    resolver.set_groups(TypeGroups::NONE);
    println!("  After disabling primitives:");
    let primitive_path: syn::Path = syn::parse_str("std::primitive::i32").unwrap();
    match resolver.resolve(&primitive_path) {
        Some(canonical) => println!("    std::primitive::i32 -> {}", canonical),
        None => println!("    std::primitive::i32 -> (not found)"),
    }
}

fn test_raw_identifiers() {
    let raw_tests = [
        "::r#std::r#primitive::r#f64",
        "r#actix_web::HttpResponse",
        "serde_json::r#Value",
    ];

    println!("Testing raw identifiers with static resolver:");
    for path_str in &raw_tests {
        if let Ok(path) = syn::parse_str::<syn::Path>(path_str) {
            match FULL_STATIC_RESOLVER.resolve(&path) {
                Some(canonical) => println!("  {} -> {}", path_str, canonical),
                None => println!("  {} -> (not found)", path_str),
            }
        }
    }
}
