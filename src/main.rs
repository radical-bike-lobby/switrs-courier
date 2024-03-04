//! SWITRS Courier is a binary for retrieving the current SWITRS DB from i-SWITRS CA CHP

use std::error::Error as StdError;

use clap::Parser;

use switrs_courier::{gmail, Error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 't',
        env = "SWITRS_GMAIL_CLIENT_SECRET",
        hide_env_values = true
    )]
    gmail_client_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();

    let client_secret = &args.gmail_client_token;

    gmail::fetch(client_secret).await?;

    Ok(())
}
