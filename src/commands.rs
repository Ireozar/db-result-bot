use crate::{winner, Context, Error};

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

#[poise::command(prefix_command, slash_command)]
pub async fn winner(
    ctx: Context<'_>,
    #[description = "DuelingBook replay link"] url: String,
) -> Result<(), Error> {
    let response = format!("The winner is: {}", winner::get_winner(url).await?);
    ctx.say(response).await?;
    Ok(())
}
