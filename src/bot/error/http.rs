use poise::serenity_prelude as serenity;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] serenity::http::HttpError),

    #[error("Client error: {0}")]
    Client(#[from] serenity::ClientError),
}
