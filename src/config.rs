use crate::error::{Error, Result};
use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| 
        Config::load_from_env()
        .unwrap_or_else(|e| panic!("Failed to load configuration: {e:?}")))
}

#[allow(non_snake_case)]
pub struct Config {
    pub WEB_FOLDER: String,
}

impl Config {
    pub fn load_from_env() -> Result<Config> {
    
        Ok(Config {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissing(name))
}