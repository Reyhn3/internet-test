mod input;
mod logging;
mod ncsi;

use clap::Parser;
use log::{error, info};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args = input::Args::parse();
    let quiet = args.quiet;
    let verbose = args.verbose;

    logging::init(quiet, verbose);

//TODO: Remove when done learning
    let debug = args.debug;
    logging::log_debug(debug);

    match ncsi::run_ncsi().await {
        Ok(result) => {
            if result == ExitCode::SUCCESS {
                info!("Working Internet connection detected")
            } else {
                error!("No working Internet connection detected")
            }
            result
        }
        Err(e) => {
            error!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
