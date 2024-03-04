//! Error types from the Telraam API

use mailparse::MailParseError;
use thiserror::Error;

/// Various error types from the Telraam API
#[derive(Error, Debug)]
pub enum Error {
    /// An error occured on the request
    #[error("parse_error: {0}")]
    BadEmail(#[from] MailParseError),
    /// Error authenticating for gcp_auth
    #[error("gcp auth error: {0}")]
    GcpAuth(#[from] gcp_auth::Error),
    /// No data was found in the email message
    #[error("no data found")]
    NoDataFound(),
    /// No gcp auth code was received
    #[error("no gcp auth code")]
    NoGcpAuthCode,
    /// Url parse error
    #[error("error parsing url: {0}")]
    UrlParseError(#[from] url::ParseError),
    /// Bad Oauth configuration
    #[error("oauth2 config error: {0}")]
    OauthConfigError(#[from] oauth2::ConfigurationError),
    /// Oauth failed
    #[error("oauth2 exchange failed: {0}")]
    Ouath2Device(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::DeviceCodeErrorResponseType>,
        >,
    ),
    /// Oauth failed
    #[error("oauth2 exchange failed: {0}")]
    Ouath2Web(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),
}
