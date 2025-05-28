#![warn(clippy::str_to_string)]

mod bot;
mod commands;
mod components;
mod startup;

use startup::{init_tracing, load_env, start_bot};
use bot::create_framework;
use bot::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let (token, guild_id) = load_env()?;
    let framework = create_framework(guild_id).await;

    start_bot(token, framework).await
}
