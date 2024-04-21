use reqwest::{
    self,
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder, RequestBuilder,
};
use std::{fmt, time::Duration};

use crate::Error;

struct Password(String);

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Password").field(&"******").finish()
    }
}

pub fn client() -> ClientBuilder {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );

    reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .cookie_store(true)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .timeout(Duration::from_secs(120))
        .https_only(true)
        .hickory_dns(true)
        .default_headers(headers)
}

pub struct IswitrsLogin {
    username: String,
    password: Password,
}

impl IswitrsLogin {
    pub fn new(username: String, password: String) -> Self {
        IswitrsLogin {
            username,
            password: Password(password),
        }
    }

    pub async fn login(&self, client: &reqwest::Client) -> Result<Iswitrs, Error> {
        // GET /Reports/jsp/index.jsp HTTP/1.1
        // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
        // Accept-Encoding: gzip, deflate, br
        // Accept-Language: en-US,en;q=0.9
        // Connection: keep-alive
        // Host: iswitrs.chp.ca.gov
        // Referer: https://www.google.com/
        // Sec-Fetch-Dest: document
        // Sec-Fetch-Mode: navigate
        // Sec-Fetch-Site: same-origin
        // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15

        eprintln!("calling switrs");

        let content = client
            .get("https://iswitrs.chp.ca.gov/Reports/jsp/index.jsp")
            .send()
            .await?
            .text()
            .await?;

        eprintln!("initial request: {content}");

        Ok(Iswitrs {})
    }
}

pub struct Iswitrs {}
