//! SWITRS Courier is a binary for retrieving the current SWITRS DB from i-SWITRS CA CHP

#![allow(clippy::print_stdout)]

use std::{
    error::Error as StdError,
    path::{Path, PathBuf},
    time::Duration,
};

use futures::future;
use tokio::fs;

use clap::Parser;

use reqwest::redirect::Policy;
use switrs_courier::{
    calodp::{Ccrs, Resource},
    Error,
};

const I_SWITRS: &str = "https://iswitrs.chp.ca.gov/Reports/jsp/CollisionReports.jsp";
const TERMS_OF_SERVICE: &str = include_str!("../SWITRS-TERMS-OF-USE.txt");
const CPRA_ADVISEMENT: &str = include_str!("../CPRA-ADVISEMENT.txt");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print the terms of service from I-SWITRS
    #[arg(long)]
    terms_of_service: bool,

    /// Print the CPRA Advisement from I-SWITRS
    #[arg(long)]
    cpra_advisement: bool,

    /// Output path for any downloaded content
    #[arg(short, long, default_value = "target/ccrs_cachedir")]
    output_dir: PathBuf,
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

    let ccrs = Ccrs::new();
    let package = ccrs.fetch_ccrs().await?;
    let resources = package.resources()?;

    std::fs::create_dir_all(&args.output_dir)?;

    let num_resources = fetch_all_resources(resources, &args.output_dir).await?;
    println!("Fetched {} resources", num_resources);

    Ok(())
}

async fn fetch_all_resources(resources: &[Resource], output_dir: &Path) -> Result<usize, Error> {
    let client = reqwest::Client::builder()
        .hickory_dns(true)
        .redirect(Policy::limited(5))
        .https_only(true)
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(360))
        .build()?;

    // Create tasks for each resource
    let tasks: Vec<_> = resources
        .iter()
        .map(|resource| {
            let client = client.clone();
            let resource_id = resource.id.clone();
            let resource_name = resource.name.clone();
            let resource_url = resource.url.clone();
            let resource_format = resource.format.clone();
            let output_path = output_dir.to_path_buf();

            tokio::spawn(async move {
                fetch_single_resource(
                    client,
                    resource_id,
                    resource_name,
                    resource_url,
                    resource_format,
                    output_path,
                )
                .await
            })
        })
        .collect();

    // Wait for all tasks to complete
    let results = future::join_all(tasks).await;

    // Check for errors
    for result in &results {
        match result {
            Ok(Ok(())) => continue,
            Ok(Err(e)) => eprintln!("Error fetching resource: {}", e),
            Err(e) => eprintln!("Task join error: {}", e),
        }
    }

    Ok(results.len())
}

async fn fetch_single_resource(
    client: reqwest::Client,
    resource_id: String,
    resource_name: String,
    resource_url: String,
    resource_format: String,
    output_dir: PathBuf,
) -> Result<(), Error> {
    // Create filename from resource name and format
    let mut filename = output_dir.join(resource_name.replace(" ", "_").replace("/", "_"));
    filename.set_extension(resource_format.to_lowercase());

    if filename.exists() {
        println!("Resource is in cache: {} ({})", resource_name, resource_id);
        return Ok(());
    }

    println!("Fetching resource: {} ({})", resource_name, resource_id);

    let response = client.get(&resource_url).send().await?;

    if !response.status().is_success() {
        return Err(Error::HttpError(format!(
            "Failed to fetch resource {}: HTTP {}",
            resource_name,
            response.status()
        )));
    }

    let bytes = response.bytes().await?;

    fs::write(&filename, &bytes).await?;

    println!("Saved resource '{}' to {:?}", resource_name, filename);

    Ok(())
}
