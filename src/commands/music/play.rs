use crate::primitives::Context;
use anyhow::{Context as _, Result};

#[poise::command(prefix_command, slash_command)]
/// Toca uma música
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL do youtube ou nome"] song: String,
) -> Result<()> {
    let reply = ctx.say(format!("Tentando tocar `{song}`...")).await?;
    let guild = ctx.guild().context("No Guild!")?;
    let mut query = song;

    if !query.starts_with("http") {
        query = format!("ytsearch:{query}");
    }

    let client = songbird::get(ctx.serenity_context())
        .await
        .context("Couldn't start songbird client")?;

    let handler = client
        .get(guild.id)
        .context("Must be in a voice channel to play music!")?;

    let mut handler = handler.lock().await;

    let input = songbird::ytdl(query).await?;
    let title = input.metadata.title.clone().unwrap_or_default();

    handler.enqueue_source(input);

    reply
        .edit(ctx, |e| e.content(format!("Tocando `{title}`")))
        .await?;

    Ok(())
}
