mod auth;

use google_gmail1::{hyper, hyper_rustls, Gmail};
use oauth2::TokenResponse;

use crate::Error;

pub async fn fetch(client_secret: &str) -> Result<(), Error> {
    let token = auth::dance(client_secret).await?;

    let mut hub = Gmail::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        token.access_token().secret().to_string(),
    );

    dbg!(hub.users().get_profile("me").doit().await);
    //hub.users().messages_list(user_id);

    auth::break_dance(client_secret, token).await;

    Ok(())
}
