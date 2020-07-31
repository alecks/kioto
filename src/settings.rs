use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

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
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("Config").required(false))?;
        s.merge(Environment::with_prefix("ccord"))?;
        s.try_into()
    }
}
