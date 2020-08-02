use crate::discord::util::ClientShardManager;
use crate::util::Settings;
use serenity::client::{bridge::gateway::ShardId, Context};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;

#[group]
#[commands(ping, info)]
struct General;

#[command]
#[description = "Retrieves the latency of the bot."]
#[aliases(latency, pong)]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let shard_manager = ctx
        .data
        .read()
        .await
        .get::<ClientShardManager>()
        .ok_or("Couldn't get shard manager")?
        .clone();
    let shard_latency = shard_manager
        .lock()
        .await
        .runners
        .lock()
        .await
        .get(&ShardId(ctx.shard_id))
        .ok_or("Couldn't get shard")?
        .latency
        .ok_or("Couldn't get shard latency")?
        .as_millis();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("🏓")
                    .description(format!("Shard: **{}ms**", shard_latency))
            })
        })
        .await?;
    Ok(())
}

static VERSION: &str = env!("CARGO_PKG_VERSION");
static REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

#[command]
#[description = "Displays bot information."]
#[aliases(about, meta)]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let cfg = data.get::<Settings>().unwrap();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!("{} v{}", cfg.meta.name, VERSION))
                    .description(format!(
                        "An open-source bot made with ❤️ by [the community]({}/graphs/contributors).",
                        REPOSITORY
                    ))
                    .url(REPOSITORY)
            })
        })
        .await?;
    Ok(())
}
