use teloxide::{ApiError, RequestError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Instagram API error: {0}")]
    InstagramApi(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Media not found: {0}")]
    MediaNotFound(String),

    #[error("Authentication required: {0}")]
    AuthRequired(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Dialogue error: {0}")]
    DialogueError(String),

    #[error("Cache error: {0}")]
    CacheError(String),
    #[error(transparent)]
    Other(anyhow::Error),
}

impl From<BotError> for RequestError {
    fn from(error: BotError) -> Self {
        RequestError::Api(ApiError::Unknown(error.to_string()))
    }
}

impl From<anyhow::Error> for BotError {
    fn from(error: anyhow::Error) -> Self {
        BotError::Other(error)
    }
}

pub type BotResult<T> = Result<T, BotError>;
