// Application wide error type
use thiserror::Error;

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
}

pub type ScannerResult<T> = std::result::Result<T, ScannerError>;
