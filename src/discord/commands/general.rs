use crate::discord::util::{ClientShardManager, Settings};
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
        .ok_or_else(|| "Couldn't get shard manager")?
        .clone();
    let shard_latency = shard_manager
        .lock()
        .await
        .runners
        .lock()
        .await
        .get(&ShardId(ctx.shard_id))
        .ok_or_else(|| "Couldn't get shard")?
        .latency
        .ok_or_else(|| "Couldn't get shard latency")?
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

#[command]
#[description = "Displays bot information."]
#[aliases(about, meta)]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let cfg = data.get::<Settings>().unwrap();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(&cfg.bot.name)
                    .description(
                        "An open-source bot made with ❤️ by [Alex](https://github.com/fjah), et al.",
                    )
                    .url("https://github.com/fjah/calendarcord")
            })
        })
        .await?;
    Ok(())
}
