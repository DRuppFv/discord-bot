use crate::primitives::Context;
use anyhow::{Context as _, Result};
use poise::{
    futures_util::{stream, Stream, StreamExt},
    serenity_prelude::UserId,
};
use std::future::ready;

async fn autocomplete_bans<'a>(
    cx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = poise::AutocompleteChoice<UserId>> + 'a {
    stream::iter(cx.guild().unwrap().bans(cx).await.unwrap())
        .filter(move |ban| ready(ban.user.name.contains(partial)))
        .map(|ban| poise::AutocompleteChoice {
            name: ban.user.name,
            value: ban.user.id,
        })
}

///〔🛡️ Administração〕 Bana um usuário
#[poise::command(slash_command, prefix_command)]
pub async fn unban(
    cx: Context<'_>,
    #[description = "Usuário"]
    #[autocomplete = "autocomplete_bans"]
    user: UserId,
) -> Result<()> {
    let guild = cx.guild().context("No Guild!")?;
    guild.unban(cx, user).await?;

    cx.say("Usuário desbanido!").await?;

    Ok(())
}
