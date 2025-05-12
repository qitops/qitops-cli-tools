use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Test execution failed: {0}")]
    TestError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Schema validation error: {0}")]
    SchemaValidationError(String),

    #[error("Data source error: {0}")]
    DataSourceError(String),

    #[error("AI model error: {0}")]
    AiModelError(String),

    #[error("AI error: {0}")]
    AiError(String),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeatureError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
