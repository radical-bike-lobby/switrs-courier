//! Error types from the Telraam API

use mailparse::MailParseError;
use thiserror::Error;

/// Various error types from the Telraam API
#[derive(Error, Debug)]
pub enum Error {
    /// An error occured on the request
    #[error("parse_error:{0}")]
    BadEmail(#[from] MailParseError),
    /// No data was found in the email message
    #[error("no data found")]
    NoDataFound(),
}
