use std::sync::LazyLock;

pub struct Config {
    pub air_api_key: String,
}

impl Config {
    fn init() -> Self {
        Config {
            air_api_key: std::env::var("AIR_API_KEY").unwrap(),
        }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::init);
