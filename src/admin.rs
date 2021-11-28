use crate::{helper_functions::embed_msg, ShardManagerType};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandError, CommandResult,
    },
    model::channel::Message,
    utils::Color,
};

#[group]
#[prefixes("admin")]
#[commands(shutdown)]
struct Admin;

// --------------------
// Admin group commands
// --------------------

#[command]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let shard_manager = match data.get_mut::<ShardManagerType>() {
        Some(shard_manager) => shard_manager.lock(),
        None => {
            embed_msg(ctx, msg, "**Error**: Shutdown failed", Color::RED).await?;
            return Err(CommandError::from("No shard manager in data!"));
        }
    };
    embed_msg(ctx, msg, "**Goodbye!**", Color::PURPLE).await?;
    shard_manager.await.shutdown_all().await;
    Ok(())
}
