mod input;
mod logger;

use anyhow::Result;
use clap::Parser;
use log::{debug, error, info, trace, warn};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = input::Args::parse();
    let quiet = args.quiet;
    let verbose = args.verbose;
    let debug = args.debug;

    logger::init(quiet, verbose);
    logger::log_debug(debug);

    return match url_lookup().await {
        Ok(_) => {
            info!("Working Internet connection detected");
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("Error: {}", e);
            ExitCode::SUCCESS
        }
    };
}

async fn url_lookup() -> Result<()> {
    //TODO: Implement
    // return Err(anyhow::anyhow!("URL lookup failed"));

    Ok(())
}
