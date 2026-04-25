use std::{env, fs, path::Path};

fn main() {
    // Select profile: BUILD_PROFILE env var, default "local"
    let profile = env::var("BUILD_PROFILE").unwrap_or_else(|_| "local".to_string());

    // Locate workspace root from this package's manifest dir (packages/ui → ../../)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .expect("could not resolve workspace root from CARGO_MANIFEST_DIR");

    let env_path = workspace_root.join(format!(".env.{}", profile));

    println!("cargo:rerun-if-changed={}", env_path.display());
    println!("cargo:rerun-if-env-changed=BUILD_PROFILE");

    // Defaults — used when the .env file is absent or a key is missing
    let mut api_base = "http://localhost:9678/api/v1".to_string();
    let mut graphql_url = "http://localhost:9678/graphql".to_string();
    let mut app_env = profile.clone();

    if let Ok(content) = fs::read_to_string(&env_path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, val)) = line.split_once('=') {
                let key = key.trim();
                let val = val.trim().trim_matches('"').trim_matches('\'');
                match key {
                    "API_BASE_URL" => api_base = val.to_string(),
                    "GRAPHQL_URL" => graphql_url = val.to_string(),
                    "APP_ENV" => app_env = val.to_string(),
                    _ => {}
                }
            }
        }
    }

    println!("cargo:rustc-env=API_BASE_URL={api_base}");
    println!("cargo:rustc-env=GRAPHQL_URL={graphql_url}");
    println!("cargo:rustc-env=APP_ENV={app_env}");
}
