mod input;
mod logging;
mod ncsi;
mod probing;
mod check_connectivity;

pub(crate) mod codes;

use clap::Parser;
use log::{error, info};
use std::process::ExitCode;
use crate::check_connectivity::checks::Check;

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
        Ok(result) => {
            if result == true {
                info!("Working Internet connection detected")
            } else {
                error!("No working Internet connection detected")
            }
//TODO: Map all values
            if result {
                ExitCode::from(codes::INTERNET_ACCESS_FULL)
            } else {
                ExitCode::from(codes::INTERNET_ACCESS_NONE)
            }
        }
        Err(e) => {
            error!("Error: {}", e);
            ExitCode::from(codes::GENERAL_ERROR)
        }
    }

    // match ncsi::run_ncsi().await {
    //     Ok(result) => {
    //         if result == ExitCode::SUCCESS {
    //             info!("Working Internet connection detected")
    //         } else {
    //             error!("No working Internet connection detected")
    //         }
    //         result
    //     }
    //     Err(e) => {
    //         error!("Error: {}", e);
    //         ExitCode::from(codes::GENERAL_ERROR)
    //     }
    // }
}
