//! SWITRS Courier is a binary for retrieving the current SWITRS DB from i-SWITRS CA CHP

#![allow(clippy::print_stdout)]

use std::error::Error as StdError;

use clap::Parser;

use switrs_courier::{gmail, Error};

const I_SWITRS: &str = "https://iswitrs.chp.ca.gov/Reports/jsp/CollisionReports.jsp";
const TERMS_OF_SERVICE: &str = include_str!("../SWITRS-TERMS-OF-USE.txt");
const CPRA_ADVISEMENT: &str = include_str!("../CPRA-ADVISEMENT.txt");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 't',
        env = "SWITRS_GMAIL_CLIENT_SECRET",
        hide_env_values = true
    )]
    gmail_client_token: String,

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

    let client_secret = &args.gmail_client_token;

    gmail::fetch(client_secret).await?;

    Ok(())
}
