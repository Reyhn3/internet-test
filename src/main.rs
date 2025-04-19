mod input;
mod logger;
mod ncsi;

use clap::Parser;
use log::{error, info};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = input::Args::parse();
    let quiet = args.quiet;
    let verbose = args.verbose;

//TODO: Remove these when done developing
    let debug = args.debug;
    let error = args.error;

    logger::init(quiet, verbose);
    logger::log_debug(debug);

    return match ncsi::run_ncsi(error) {
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
    };
}
