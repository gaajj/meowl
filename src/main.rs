#![warn(clippy::str_to_string)]

mod bot;
mod commands;
mod components;
mod startup;

use startup::{init_tracing, load_config, start_bot};
use bot::create_framework;
use bot::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let config = load_config()?;
    let framework = create_framework(config.guild_id, config.prefix.clone()).await;

    start_bot(config.token, framework).await
}
