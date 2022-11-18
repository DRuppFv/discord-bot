use std::time::Instant;

use anyhow::Result;

use crate::primitives::Context;

/// Envia um ping
#[poise::command(slash_command)]
pub async fn ping(cx: Context<'_>) -> Result<()> {
    let time = Instant::now();
    let handle = cx.say(":stopwatch: um momento...").await?;
    handle
        .edit(cx, |m| {
            m.content(format!(":butterfly: {:.2?}", time.elapsed()))
        })
        .await?;
    Ok(())
}
