use crate::winner;
use crate::{Context, Error};

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn process(
    ctx: Context<'_>,
    #[description = "DuelingBook replay link"] url: String,
) -> Result<(), Error> {
    ctx.say("Processing...").await?;
    let result = winner::process(url).await?;
    ctx.say(result).await?;
    Ok(())
}
