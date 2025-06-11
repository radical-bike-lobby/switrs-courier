//! Error types from the Telraam API

use thiserror::Error;

/// Various error types from the Telraam API
#[derive(Error, Debug)]
pub enum Error {
    /// No data was found in the email message
    #[error("no data found")]
    NoDataFound(),
    /// No gcp auth code was received
    #[error("no gcp auth code")]
    NoGcpAuthCode,
    /// Url parse error
    #[error("error parsing url: {0}")]
    UrlParseError(#[from] url::ParseError),
    /// Reqwest, Http, errors
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Dynamic Error
    #[error("dyn error: {0}")]
    Dyn(#[from] Box<dyn core::error::Error>),

    /// Serde Error
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}
