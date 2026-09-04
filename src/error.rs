use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("broker api error [{code}]: {message}")]
    Api { code: String, message: String },
}