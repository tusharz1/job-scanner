use openrouter_rs::error::OpenRouterError;
// Application wide error type
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("API Request Failed: {0}")]
    ApiError(#[from] reqwest::Error),

    #[error("LLM Error: {0}")]
    LlmError(String),

    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Database Error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Openrouter Error: {0}")]
    OpenRouterProviderError(#[from] OpenRouterError),

    #[error("Url parser error: {0}")]
    UrlParseError(#[from] ParseError)
}

pub type ScannerResult<T> = std::result::Result<T, ScannerError>;
