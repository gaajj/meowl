use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Missing or invalid environment variable: {0}")]
    MissingOrInvalid(String),
}
