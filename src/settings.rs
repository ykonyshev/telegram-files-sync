use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

const CONFIG_FOLDER: &str = "config";

#[derive(Debug, Deserialize)]
pub struct Telegram {
    pub api_id: i32,
    pub api_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct DB {
    pub connection_string: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub telegram: Telegram,
    pub db: DB
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        let settings = Config::builder()
            .add_source(File::with_name(&format!("{}/{}", CONFIG_FOLDER, run_mode)))
            .build()?;

        settings.try_deserialize()
    }
}
