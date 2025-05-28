use crate::bot::Data;
use crate::bot::error::{BotError, Error};
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
}

pub fn load_env() -> Result<(String, serenity::GuildId), Error> {
    dotenv().ok();
    let token = get_env_var("DISCORD_TOKEN")?;
    let guild_id = get_env_var("GUILD_ID")?
        .parse::<u64>()
        .map_err(|_| BotError::EnvVar("Invalid GUILD_ID".to_string()))?;
    Ok((token, serenity::GuildId::from(guild_id)))
}

pub fn get_env_var(key: &str) -> Result<String, Error> {
    std::env::var(key).map_err(|_| BotError::EnvVar(key.to_string()))
}

pub async fn start_bot(token: String, framework: poise::Framework<Data, Error>) -> Result<(), Error> {
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(|e| BotError::Other(anyhow::anyhow!("Failed to build Discord client: {}", e)))?;

    client.start().await
        .map_err(|e| BotError::Other(anyhow::anyhow!("Failed to start client: {}", e)))?;

    Ok(())
}
