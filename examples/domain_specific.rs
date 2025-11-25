use desynt::{DynamicPathResolver, PathResolver, create_static_resolver};
use phf::{Map, phf_map};

// Domain-specific type mappings for web applications
const WEB_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
    // HTTP and web frameworks
    "actix_web::HttpRequest" => "Request",
    "actix_web::HttpResponse" => "Response",
    "actix_web::web::Data" => "AppState",
    "axum::extract::State" => "AppState",
    "axum::extract::Path" => "PathParams",
    "axum::response::Json" => "JsonResponse",
    "warp::filters::path::Tail" => "PathTail",
    "warp::reply::Json" => "JsonReply",

    // Authentication and sessions
    "actix_session::Session" => "UserSession",
    "jsonwebtoken::TokenData" => "JwtToken",
    "oauth2::AccessToken" => "OAuthToken",

    // Middleware and extractors
    "actix_web::middleware::Logger" => "RequestLogger",
    "tower::ServiceBuilder" => "MiddlewareStack",
    "axum::middleware::Next" => "NextHandler",

    // Error handling
    "actix_web::error::Error" => "WebError",
    "actix_web::Result" => "WebResult",
    "axum::response::ErrorResponse" => "ErrorResponse",
};

// Database and persistence mappings
const DATABASE_MAPPINGS: Map<&'static str, &'static str> = phf_map! {
    // Database connections and pools
    "sqlx::Pool" => "DbPool",
    "sqlx::Connection" => "DbConnection",
    "sqlx::Transaction" => "DbTransaction",
    "diesel::PgConnection" => "PostgresConn",
    "diesel::MysqlConnection" => "MysqlConn",
    "tokio_postgres::Client" => "AsyncPgClient",
    "redis::Connection" => "RedisConn",
    "mongodb::Client" => "MongoClient",

    // Query results and rows
    "sqlx::Row" => "DbRow",
    "sqlx::postgres::PgRow" => "PostgresRow",
    "diesel::QueryResult" => "QueryResult",
    "tokio_postgres::Row" => "AsyncRow",
    "redis::Value" => "RedisValue",
    "mongodb::results::InsertOneResult" => "InsertResult",

    // Schema and migrations
    "diesel::schema::table" => "TableSchema",
    "sqlx::migrate::Migration" => "DbMigration",
    "sea_orm::entity::Model" => "EntityModel",

    // ORM and query builders
    "diesel::QueryDsl" => "QueryBuilder",
    "sea_orm::Select" => "SelectQuery",
    "sqlx::query::Query" => "SqlQuery",
};

// Create const resolvers for different domains
const WEB_RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
    create_static_resolver(&WEB_MAPPINGS, true);

const DATABASE_RESOLVER: PathResolver<&'static Map<&'static str, &'static str>> =
    create_static_resolver(&DATABASE_MAPPINGS, true);

fn main() {
    println!("=== Domain-Specific PathResolver Examples ===\n");

    // Test web domain resolver
    println!("1. Web Application Resolver:");
    test_web_types();

    // Test database domain resolver
    println!("\n2. Database & Persistence Resolver:");
    test_database_types();

    // Test combined dynamic resolver
    println!("\n3. Combined Dynamic Resolver:");
    test_combined_resolver();

    // Test domain-specific path resolution
    println!("\n4. Domain-Specific Path Analysis:");
    analyze_domain_paths();

    println!("\n5. Performance Comparison:");
    performance_comparison();

    println!("\n=== Summary ===");
    println!("✓ Domain-specific resolvers for specialized type mapping");
    println!("✓ Static PHF maps for compile-time domain knowledge");
    println!("✓ Combined resolvers for full-stack applications");
    println!("✓ Zero-cost abstractions with const initialization");
    println!("✓ Type safety across domain boundaries");
}

fn test_web_types() {
    let web_paths = [
        "actix_web::HttpRequest",
        "actix_web::HttpResponse",
        "axum::extract::State",
        "axum::response::Json",
        "jsonwebtoken::TokenData",
        "std::string::String", // primitive should still work
        "unknown::WebType",
    ];

    println!("Testing Web domain resolver:");
    println!("  Total mappings: {}", WEB_RESOLVER.len());
    println!("  Uses primitives: {}", WEB_RESOLVER.uses_primitives());

    for path_str in &web_paths {
        let path: syn::Path = syn::parse_str(path_str).unwrap();
        match WEB_RESOLVER.resolve(&path) {
            Some(canonical) => println!("  {} -> {}", path_str, canonical),
            None => println!("  {} -> (not found)", path_str),
        }
    }
}

fn test_database_types() {
    let db_paths = [
        "sqlx::Pool",
        "sqlx::Transaction",
        "diesel::PgConnection",
        "tokio_postgres::Client",
        "mongodb::Client",
        "std::option::Option", // primitive should still work
        "unknown::DbType",
    ];

    println!("Testing Database domain resolver:");
    println!("  Total mappings: {}", DATABASE_RESOLVER.len());
    println!("  Uses primitives: {}", DATABASE_RESOLVER.uses_primitives());

    for path_str in &db_paths {
        let path: syn::Path = syn::parse_str(path_str).unwrap();
        match DATABASE_RESOLVER.resolve(&path) {
            Some(canonical) => println!("  {} -> {}", path_str, canonical),
            None => println!("  {} -> (not found)", path_str),
        }
    }
}

fn test_combined_resolver() {
    // Create a dynamic resolver that combines both domains
    let mut combined = DynamicPathResolver::with_primitives();

    // Add web domain mappings
    combined.add_mapping("actix_web::HttpRequest", "Request");
    combined.add_mapping("axum::extract::State", "AppState");
    combined.add_mapping("jsonwebtoken::TokenData", "JwtToken");

    // Add database domain mappings
    combined.add_mapping("sqlx::Pool", "DbPool");
    combined.add_mapping("diesel::PgConnection", "PostgresConn");
    combined.add_mapping("mongodb::Client", "MongoClient");

    // Add application-specific mappings
    combined.add_mapping("my_app::models::User", "UserModel");
    combined.add_mapping("my_app::services::AuthService", "AuthService");

    let test_paths = [
        "actix_web::HttpRequest", // web domain
        "sqlx::Pool",             // database domain
        "my_app::models::User",   // app-specific
        "std::vec::Vec",          // primitive
        "unknown::Type",          // unmapped
    ];

    println!("Testing Combined resolver:");
    println!("  Total mappings: {}", combined.len());
    println!("  Uses primitives: {}", combined.uses_primitives());

    for path_str in &test_paths {
        let path: syn::Path = syn::parse_str(path_str).unwrap();
        match combined.resolve(&path) {
            Some(canonical) => println!("  {} -> {}", path_str, canonical),
            None => println!("  {} -> (not found)", path_str),
        }
    }
}

fn analyze_domain_paths() {
    println!("Analyzing domain-specific path patterns:");

    // Check which resolver handles specific types
    let analysis_paths = [
        "actix_web::HttpResponse",
        "sqlx::Pool",
        "std::string::String",
        "serde_json::Value",
    ];

    for path_str in &analysis_paths {
        let path: syn::Path = syn::parse_str(path_str).unwrap();
        println!("  {}:", path_str);

        if WEB_RESOLVER.has_mapping(&path) {
            println!("    ✓ Web domain: {}", WEB_RESOLVER.resolve(&path).unwrap());
        }

        if DATABASE_RESOLVER.has_mapping(&path) {
            println!(
                "    ✓ Database domain: {}",
                DATABASE_RESOLVER.resolve(&path).unwrap()
            );
        }

        if !WEB_RESOLVER.has_mapping(&path) && !DATABASE_RESOLVER.has_mapping(&path) {
            println!("    - Not in specialized domains");
        }
    }
}

fn performance_comparison() {
    println!("Performance characteristics:");
    println!("  Web resolver:");
    println!("    - Static PHF map with {} entries", WEB_MAPPINGS.len());
    println!("    - O(1) lookup time");
    println!("    - Zero runtime initialization");
    println!("    - Compile-time const creation");

    println!("  Database resolver:");
    println!(
        "    - Static PHF map with {} entries",
        DATABASE_MAPPINGS.len()
    );
    println!("    - O(1) lookup time");
    println!("    - Zero runtime initialization");
    println!("    - Compile-time const creation");

    println!("  Combined dynamic resolver:");
    println!("    - Runtime HashMap creation");
    println!("    - O(1) average lookup time");
    println!("    - Flexible runtime modification");
    println!("    - Memory allocation overhead");

    // Demonstrate const usage
    const _COMPILE_TIME_WEB: PathResolver<&'static Map<&'static str, &'static str>> = WEB_RESOLVER;
    const _COMPILE_TIME_DB: PathResolver<&'static Map<&'static str, &'static str>> =
        DATABASE_RESOLVER;

    println!("  ✓ All static resolvers created at compile time!");
}
