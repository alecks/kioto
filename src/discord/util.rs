use crate::util::DbPool as PrevDbPool;
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};
use std::sync::Arc;

pub struct ClientShardManager;

impl TypeMapKey for ClientShardManager {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct DbPool;

impl TypeMapKey for DbPool {
    type Value = Arc<PrevDbPool>;
}
