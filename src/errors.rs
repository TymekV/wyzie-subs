use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WyzieError {
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("API Error: {0:?}")]
    ApiError(ApiError),
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    /// Response status code
    pub code: u32,
    pub message: String,
    pub details: String,
    pub notice: Option<String>,
}
