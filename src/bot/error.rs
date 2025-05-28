use poise::serenity_prelude as serenity;

#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("Missing or invalid environment variable: {0}")]
    EnvVar(String),

    #[error("Failed to parse GUILD_ID: {0}")]
    InvalidGuildId(String),

    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] serenity::http::HttpError),

    #[error("Client error: {0}")]
    Client(#[from] serenity::ClientError),

    #[error("Serenity error: {0}")]
    Serenity(#[from] serenity::Error),

    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Error = BotError;
pub type Context<'a> = poise::Context<'a, crate::bot::Data, BotError>;
