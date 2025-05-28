use poise::serenity_prelude as serenity;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Framework build failed: {0}")]
    Build(#[from] serenity::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
