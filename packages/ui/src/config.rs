use std::sync::OnceLock;

pub struct Config {
    pub api_base_url: &'static str,
    pub graphql_url: &'static str,
    pub app_env: &'static str,
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| Config {
        api_base_url: env!("API_BASE_URL"),
        graphql_url: env!("GRAPHQL_URL"),
        app_env: env!("APP_ENV"),
    })
}

pub fn api_base_url() -> &'static str {
    get_config().api_base_url
}

pub fn graphql_url() -> &'static str {
    get_config().graphql_url
}

pub fn app_env() -> &'static str {
    get_config().app_env
}
