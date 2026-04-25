use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_base_url: String,
    pub graphql_url: String,
    pub app_env: String,
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        #[cfg(not(target_arch = "wasm32"))]
        dotenvy::dotenv().ok();

        envy::from_env::<Config>().expect("Failed to load config from environment")
    })
}

pub fn api_base_url() -> &'static str {
    &get_config().api_base_url
}

pub fn graphql_url() -> &'static str {
    &get_config().graphql_url
}

pub fn app_env() -> &'static str {
    &get_config().app_env
}
