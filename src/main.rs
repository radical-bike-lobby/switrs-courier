//! SWITRS Courier is a binary for retrieving the current SWITRS DB from i-SWITRS CA CHP

#![allow(clippy::print_stdout)]

use std::error::Error as StdError;

use clap::Parser;

use switrs_courier::{
    gmail,
    iswitrs::{self, IswitrsLogin},
    Error,
};

const I_SWITRS: &str = "https://iswitrs.chp.ca.gov/Reports/jsp/CollisionReports.jsp";
const TERMS_OF_SERVICE: &str = include_str!("../SWITRS-TERMS-OF-USE.txt");
const CPRA_ADVISEMENT: &str = include_str!("../CPRA-ADVISEMENT.txt");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GMail client secret
    #[arg(
        short = 't',
        env = "SWITRS_GMAIL_CLIENT_SECRET",
        hide_env_values = true
    )]
    gmail_client_token: String,

    /// Username for the iswitrs site
    #[arg(short = 'u', env = "ISWITRS_USERNAME")]
    iswitrs_username: String,

    /// Password for the iswitrs site
    #[arg(short = 'p', env = "ISWITRS_PASSWORD", hide_env_values = true)]
    iswitrs_password: String,

    /// Print the terms of service from I-SWITRS
    #[arg(long)]
    terms_of_service: bool,

    /// Print the CPRA Advisement from I-SWITRS
    #[arg(long)]
    cpra_advisement: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();

    if args.terms_of_service {
        println!("Terms of Service from {I_SWITRS}:");
        println!("{TERMS_OF_SERVICE}");
        return Ok(());
    }

    if args.cpra_advisement {
        println!("CPRA Advisement from {I_SWITRS}:");
        println!("{CPRA_ADVISEMENT}");
        return Ok(());
    }

    iswitrs(&args.iswitrs_username, &args.iswitrs_password).await?;
    //    gmail(&args.gmail_client_token).await;

    Ok(())
}

async fn gmail(client_secret: &str) {
    gmail::fetch(client_secret).await;
}

async fn iswitrs(iswitrs_username: &str, iswitrs_password: &str) -> Result<(), Box<dyn StdError>> {
    let client = iswitrs::client().build()?;
    let login = IswitrsLogin::new(iswitrs_username.to_string(), iswitrs_password.to_string());

    login.login(&client).await?;

    Ok(())
}
