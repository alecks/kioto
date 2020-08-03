use config::{Config, ConfigError, Environment, File};
use log::error;
use serde::Deserialize;
use serenity::prelude::TypeMapKey;

use std::{env, sync::RwLock};
use std::{net::IpAddr, sync::Arc};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[macro_export]
macro_rules! fatal {
    ( $( $x:expr ),* ) => {{
        error!($($x),*);
        std::process::exit(1);
    }};
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub bot: Bot,
    pub http: Http,
    pub meta: Meta,
    pub db: Db,
}

#[derive(Deserialize, Clone)]
pub struct Http {
    pub bind: IpAddr,
    pub port: u16,
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct Bot {
    pub prefix: String,
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize, Clone)]
pub struct Meta {
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub struct Db {
    pub url: String,
}

const CONFIG_FILE_DEFAULTS: &str = "config/default.hjson";
const CONFIG_FILE: &str = "config/config.hjson";
const ENV_PREFIX: &str = "KIOTO";

lazy_static! {
    static ref SETTINGS: RwLock<Settings> = RwLock::new(match Settings::init() {
        Ok(c) => c,
        Err(e) => fatal!("{}", e),
    });
}

impl Settings {
    fn init() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(&Self::get_defaults_location()))?;
        s.merge(File::with_name(CONFIG_FILE).required(false))?;
        s.merge(Environment::with_prefix(ENV_PREFIX).separator("__"))?;

        s.try_into()
    }

    pub fn get() -> Self {
        SETTINGS.read().unwrap().to_owned()
    }

    fn get_defaults_location() -> String {
        env::var(format!("{}_CONFIG", ENV_PREFIX))
            .unwrap_or_else(|_| CONFIG_FILE_DEFAULTS.to_string())
    }
}

impl TypeMapKey for Settings {
    type Value = Arc<Settings>;
}
