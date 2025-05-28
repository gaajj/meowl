use crate::bot::{Context, Error};
use crate::components::embed::base_embed;

#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.serenity_context().http.get_current_user().await?;
    let embed = base_embed(&user, "Help").description(
        "Use `/commands` to see a full list of commands.\nNeed help? Use `/ticket`.",
    );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
