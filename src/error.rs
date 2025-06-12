//! All Error types from this crate

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
    Dyn(#[from] Box<dyn core::error::Error + Send + Sync>),

    /// CKAN Error
    #[error("ckan error: {0}")]
    CkanError(String),

    /// HTTP Error
    #[error("http error: {0}")]
    HttpError(String),

    /// IO Error
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Serde Error
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[test]
fn test_send_sync() {
    fn is_send_sync<T: Send + Sync>() {}

    is_send_sync::<Error>();
}
