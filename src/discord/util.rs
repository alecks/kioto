use serenity::{prelude::{Mutex, TypeMapKey}, client::bridge::gateway::ShardManager};
use std::sync::Arc;

pub struct ClientShardManager;

impl TypeMapKey for ClientShardManager {
    type Value = Arc<Mutex<ShardManager>>;
}
