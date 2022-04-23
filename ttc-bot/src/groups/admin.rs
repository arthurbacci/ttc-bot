/*use crate::{typemap::types::ShardManagerType, utils::helper_functions::embed_msg};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    utils::Color,
};

#[group]
#[prefix("admin")]
#[owners_only]
#[commands(shutdown)]
struct Admin;

// --------------------
// Admin group commands
// --------------------

#[command]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;
    let shard_manager = data.get_mut::<ShardManagerType>().unwrap();

    embed_msg(
        ctx,
        &msg.channel_id,
        Some("Goodbye!"),
        Some("ttc-bot shutting down."),
        Some(Color::PURPLE),
        None,
    )
    .await?;
    shard_manager.lock().await.shutdown_all().await;
    Ok(())
}*/

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn register(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    log::info!("Registering slash commands");
    poise::builtins::register_application_commands(ctx, false).await?;
    Ok(())
}
