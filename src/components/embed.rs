use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter, Timestamp, User};

pub fn base_embed(user: &User, title: &str) -> CreateEmbed {
    let avatar = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());

    CreateEmbed::new()
        .title(title)
        .footer(CreateEmbedFooter::new("Meowl").icon_url(avatar))
        .timestamp(Timestamp::now())
        .color(0x00FF00)
}
