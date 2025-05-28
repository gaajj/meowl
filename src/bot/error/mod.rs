pub mod env;
pub mod http;
pub mod framework;

use env::EnvError;
use framework::FrameworkError;
use http::HttpError;
use poise::serenity_prelude;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error(transparent)]
    Env(#[from] EnvError),

    #[error(transparent)]
    Http(#[from] HttpError),

    #[error(transparent)]
    Framework(#[from] FrameworkError),

    #[error("Serenity error: {0}")]
    Serenity(#[from] serenity_prelude::Error),
}

pub type Error = BotError;
pub type Context<'a> = poise::Context<'a, crate::bot::Data, BotError>;
