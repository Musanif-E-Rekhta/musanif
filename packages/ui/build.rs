fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = std::path::Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let profile = std::env::var("APP_PROFILE").unwrap_or_else(|_| "local".to_string());
    let profile_env = workspace_root.join(format!(".env.{}", profile));
    let fallback_env = workspace_root.join(".env");

    if profile_env.exists() {
        dotenvy::from_path(&profile_env).ok();
        println!("cargo:rerun-if-changed={}", profile_env.display());
    } else if fallback_env.exists() {
        dotenvy::from_path(&fallback_env).ok();
        println!("cargo:rerun-if-changed={}", fallback_env.display());
    }

    println!("cargo:rerun-if-env-changed=APP_PROFILE");

    let api_base_url = std::env::var("API_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:9678/api/v1".to_string());
    let graphql_url = std::env::var("GRAPHQL_URL")
        .unwrap_or_else(|_| "http://localhost:9678/graphql".to_string());
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    println!("cargo:rustc-env=API_BASE_URL={api_base_url}");
    println!("cargo:rustc-env=GRAPHQL_URL={graphql_url}");
    println!("cargo:rustc-env=APP_ENV={app_env}");
}
