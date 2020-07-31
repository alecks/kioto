use crate::discord::util::ClientShardManager;
use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

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
