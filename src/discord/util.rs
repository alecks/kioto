use crate::settings::Settings as FSettings;
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};
use std::sync::Arc;

pub struct ClientShardManager;

impl TypeMapKey for ClientShardManager {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Settings;

impl TypeMapKey for Settings {
    type Value = Arc<FSettings>;
}
