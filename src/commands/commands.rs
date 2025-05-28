use crate::bot::{Context, Error};
use crate::components::embed::base_embed;

#[poise::command(prefix_command, slash_command)]
pub async fn commands(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.serenity_context().http.get_current_user().await?;
    let mut embed = base_embed(&user, "Commands")
        .description("Here are the available commands:");

    let mut count = 0;
    for command in &ctx.framework().options().commands {
        if count >= 25 {
            embed = embed.field("...", "Too many commands to show in one message.", false);
            break;
        }

        let name = match (
            command.prefix_action.is_some(),
            command.slash_action.is_some(),
        ) {
            (true, true) => format!(". / {}", command.name),
            (true, false) => format!(". {}", command.name),
            (false, true) => format!("/ {}", command.name),
            (false, false) => continue,
        };

        let description = command
            .description
            .as_deref()
            .unwrap_or("No description provided");

        embed = embed.field(name, description, false);
        count += 1;
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
