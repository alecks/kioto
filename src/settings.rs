use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Settings {
    pub bot: Bot,
    pub http: Http,
}

#[derive(Deserialize)]
pub struct Http {
    pub address: String,
}

#[derive(Deserialize)]
pub struct Bot {
    pub prefix: String,
    pub token: String,
    pub name: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Config").required(false))?;
        s.merge(Environment::with_prefix("ccord"))?;
        s.try_into()
    }
}

impl TypeMapKey for Settings {
    type Value = Arc<Settings>;
}
