#![warn(clippy::str_to_string)]

mod bot;
mod commands;
mod components;

use bot::{create_framework, Data};
use bot::error::{env::EnvError, Error};
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;

fn get_env_var(key: &str) -> Result<String, EnvError> {
    std::env::var(key).map_err(|_| EnvError::MissingOrInvalid(key.to_string()))
}

fn load_env() -> Result<(String, serenity::GuildId), Error> {
    dotenv().ok();
    let token = get_env_var("DISCORD_TOKEN")?;
    let guild_id = get_env_var("GUILD_ID")?
        .parse::<u64>()
        .map_err(|_| EnvError::MissingOrInvalid("Invalid GUILD_ID".to_string()))?;
    Ok((token, serenity::GuildId::from(guild_id)))
}

fn init_tracing() {
    tracing_subscriber::fmt::init();
}

async fn start_bot(token: String, framework: poise::Framework<Data, Error>) -> Result<(), Error> {
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let (token, guild_id) = load_env()?;
    let framework = create_framework(guild_id).await;

    start_bot(token, framework).await
}
