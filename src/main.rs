mod input;
mod logging;
mod probing;
mod check_connectivity;

pub(crate) mod codes;

use clap::Parser;
use log::{error, info, warn};
use std::process::ExitCode;
use crate::check_connectivity::analysis::{Verdict};

#[tokio::main]
async fn main() -> ExitCode {
    let args = input::Args::parse();
    let quiet = args.quiet;
    let verbose = args.verbose;

    logging::init(quiet, verbose);

//TODO: Remove when done learning
    let debug = args.debug;
    logging::log_debug(debug);

    let checks = check_connectivity::checks::get_default_check_list();
    let result = check_connectivity::check_internet_connectivity(checks);

    match result.await {
        Ok(result) => match result {
            Verdict::Error => {
                error!("An error occurred");
                ExitCode::from(codes::GENERAL_ERROR)
            },
            Verdict::None => {
                error!("No working Internet connection detected");
                ExitCode::from(codes::INTERNET_ACCESS_NONE)
            },
            Verdict::Limited => {
                warn!("Limited Internet connection detected");
                ExitCode::from(codes::INTERNET_ACCESS_LIMITED)
            },
            Verdict::Full => {
                info!("Working Internet connection detected");
                ExitCode::from(codes::INTERNET_ACCESS_FULL)
            },
        },
        Err(e) => {
            error!("Error: {}", e);
            ExitCode::from(codes::GENERAL_ERROR)
        }
    }
}
