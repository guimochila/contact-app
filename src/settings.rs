use config::{Config, ConfigError};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "config.toml";
const CONFIG_FILE_PREFIX: &str = "./";

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name(&format!(
                "{}{}",
                CONFIG_FILE_PREFIX, CONFIG_FILE_PATH
            )))
            .build()
            .unwrap();

        settings.try_deserialize::<Settings>()
    }
}
