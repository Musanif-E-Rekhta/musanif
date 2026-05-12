fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = std::path::Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // Pre-register the GraphQL schema with cynic so subsequent
    // `cynic::schema!` invocations resolve against it without needing an
    // absolute path. Re-run codegen whenever the contract drifts.
    let schema_path = workspace_root
        .parent()
        .unwrap()
        .join("musanif-contracts")
        .join("schema.graphql");
    cynic_codegen::register_schema("musanif")
        .from_sdl_file(&schema_path)
        .expect("Failed to register GraphQL schema for cynic")
        .as_default()
        .unwrap();
    println!("cargo:rerun-if-changed={}", schema_path.display());

    let profile = std::env::var("BUILD_PROFILE").unwrap_or_else(|_| "local".to_string());
    let profile_env = workspace_root.join(format!(".env.{}", profile));
    let fallback_env = workspace_root.join(".env");

    if profile_env.exists() {
        dotenvy::from_path(&profile_env).ok();
        println!("cargo:rerun-if-changed={}", profile_env.display());
    } else if fallback_env.exists() {
        dotenvy::from_path(&fallback_env).ok();
        println!("cargo:rerun-if-changed={}", fallback_env.display());
    }

    println!("cargo:rerun-if-env-changed=BUILD_PROFILE");

    let api_base_url = std::env::var("API_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:9678/api/v1".to_string());
    let graphql_url = std::env::var("GRAPHQL_URL")
        .unwrap_or_else(|_| "http://localhost:9678/api/graphql".to_string());
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    println!("cargo:rustc-env=API_BASE_URL={api_base_url}");
    println!("cargo:rustc-env=GRAPHQL_URL={graphql_url}");
    println!("cargo:rustc-env=APP_ENV={app_env}");
}
