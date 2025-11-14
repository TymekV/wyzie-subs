use thiserror::Error;

#[derive(Debug, Error)]
pub enum WyzieError {
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
