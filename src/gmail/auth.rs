use std::collections::HashMap;
use std::path::PathBuf;

use anyhow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    DeviceAuthorizationResponse, DeviceAuthorizationUrl, ExtraDeviceAuthorizationFields,
    PkceCodeChallenge, RedirectUrl, RevocableToken, RevocationUrl, Scope, StandardRevocableToken,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::time::sleep;
use url::Url;

use crate::Error;

pub async fn dance(
    client_secret: &str,
) -> Result<
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    Error,
> {
    let google_client_id = ClientId::new(
        "1029342291235-httptjo81jen9k1ftv4enscrdr58l4d8.apps.googleusercontent.com".to_string(),
    );
    let google_client_secret = ClientSecret::new(client_secret.to_string());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    )
    // Google supports OAuth 2.0 Token Revocation (RFC-7009)
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    );

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("https://mail.google.com/".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("tcp listener failed");

    let (stream, addr) = listener.accept().await.expect("failed to get request");

    {
        let code;
        let state;

        let (mut in_stream, mut out_stream) = stream.into_split();
        {
            let mut reader = BufReader::new(in_stream);

            let mut request_line = String::new();
            reader
                .read_line(&mut request_line)
                .await
                .expect("failed to read line");

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

            let code_pair = url
                .query_pairs()
                .find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                })
                .unwrap();

            let (_, value) = code_pair;
            code = AuthorizationCode::new(value.into_owned());

            let state_pair = url
                .query_pairs()
                .find(|pair| {
                    let &(ref key, _) = pair;
                    key == "state"
                })
                .unwrap();

            let (_, value) = state_pair;
            state = CsrfToken::new(value.into_owned());
        }

        let message = "Go back to your terminal :)";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        out_stream
            .write_all(response.as_bytes())
            .await
            .expect("failed to write response");

        println!("Google returned the following code:\n{}\n", code.secret());
        println!(
            "Google returned the following state:\n{} (expected `{}`)\n",
            state.secret(),
            csrf_token.secret()
        );

        // Exchange the code with a token.
        let token_response = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await;

        println!(
            "Google returned the following token:\n{:?}\n",
            token_response
        );

        return Ok(token_response?);
    }

    Err(Error::NoGcpAuthCode)
}

pub(crate) async fn break_dance(
    client_secret: &str,
    token: oauth2::StandardTokenResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
) {
    let google_client_id = ClientId::new(
        "1029342291235-h8tafdrogur0od1pjl96bs0dkrkql29q.apps.googleusercontent.com".to_string(),
    );
    let google_client_secret = ClientSecret::new(client_secret.to_string());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    // Revoke the obtained token
    let token_to_revoke: StandardRevocableToken = match token.refresh_token() {
        Some(token) => token.into(),
        None => token.access_token().into(),
    };

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    .set_redirect_uri(
        RedirectUrl::new(
            "https://github.com/radical-bike-lobby/switrs-courier/blob/main/README.md".to_string(),
        )
        .expect("Invalid redirect URL"),
    )
    // Google supports OAuth 2.0 Token Revocation (RFC-7009)
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    );

    client
        .revoke_token(token_to_revoke)
        .unwrap()
        .request_async(async_http_client)
        .await
        .expect("Failed to revoke token");
}

pub(crate) async fn service_dance() -> Result<gcp_auth::Token, Error> {
    use gcp_auth::{AuthenticationManager, CustomServiceAccount};

    // `credentials_path` variable is the path for the credentials `.json` file.
    let credentials_path =
        PathBuf::from("/Users/benjaminfry/Downloads/switrs-courier-7a44636842ba.json");
    let service_account = CustomServiceAccount::from_file(credentials_path)?;
    let authentication_manager = AuthenticationManager::from(service_account);
    let scopes = &["https://www.googleapis.com/auth/gmail.readonly"];
    let token = authentication_manager.get_token(scopes).await?;

    println!("token: {token:?}");

    Ok(token)
}
