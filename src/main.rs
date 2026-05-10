mod input;
mod logging;
mod probing;
mod check_connectivity;

pub(crate) mod codes;

use std::fmt::Debug;
use clap::Parser;
use log::{debug, error, info, warn};
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
    
    let strategy = args.strategy;
    let checks: Vec<_> = check_connectivity::checks::get_default_check_list()
        .into_iter()
        .filter(|check| {
            let check_strategy = check_connectivity::checks::get_strategy(check);
            match strategy {
                check_connectivity::checks::Strategy::All => true,
                _ => strategy == *check_strategy,
            }
        })
        .collect();
    debug!("Filtered checks: {:?}",
        checks
        .iter()
        .map(|c| check_connectivity::checks::get_strategy(c))
        .collect::<Vec<_>>());

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
