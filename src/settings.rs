use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use serenity::prelude::TypeMapKey;
use std::{net::IpAddr, sync::Arc};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub bot: Bot,
    pub http: Http,
    pub meta: Meta,
}

#[derive(Deserialize, Clone)]
pub struct Http {
    pub bind: IpAddr,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct Bot {
    pub prefix: String,
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct Meta {
    pub name: String,
    pub repo_url: String,
}

use std::{env, sync::RwLock};

static CONFIG_FILE_DEFAULTS: &str = "config/default.hjson";
static CONFIG_FILE: &str = "config/config.hjson";

lazy_static! {
    static ref SETTINGS: RwLock<Settings> = RwLock::new(match Settings::init() {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    });
}

impl Settings {
    fn init() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(&Self::get_config_defaults_location()))?;
        s.merge(File::with_name(CONFIG_FILE).required(false))?;
        s.merge(Environment::with_prefix("CCORD").separator("__"))?;

        s.try_into()
    }

    pub fn get() -> Self {
        SETTINGS.read().unwrap().to_owned()
    }

    pub fn get_config_defaults_location() -> String {
        env::var("CCORD_CONFIG").unwrap_or_else(|_| CONFIG_FILE_DEFAULTS.to_string())
    }
}

impl TypeMapKey for Settings {
    type Value = Arc<Settings>;
}
