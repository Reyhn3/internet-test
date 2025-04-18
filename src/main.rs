mod logger;

use anyhow::Result;
use clap::Parser;
use log::{debug, error, info, log_enabled, trace, warn, Level};
use std::fmt::Debug;
use std::process::ExitCode;

/// Checks whether there is a working Internet connection.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Enable debug logs to the terminal.
    #[clap(long, short, action, hide(true))]
    debug: bool,

    /// Disable all output to the terminal.
    #[clap(long, short, action)]
    quiet: bool,

    /// Enable full output to the terminal.
    #[clap(long, short, action)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    let quiet = args.quiet;
    let verbose = args.verbose;
    let debug = args.debug;

    logger::init(quiet, verbose);

    //TODO: Remove
    if debug {
        error!("{}", "Its fleece was white as snow");
        warn!("{:#?}", "The lamb was sure to go");
        info!("{:?}", "And every where that Mary went");
        debug!("Mary has a little lamb");
        trace!("Mary has a fluffy lamb");

        debug!("this is a debug {}", "message");
        error!("this is printed by default");

        if log_enabled!(Level::Info) {
            let x = 3 * 4; // expensive computation
            info!("the answer was: {}", x);
        }
    }

    return match url_lookup().await {
        Ok(_) => {
            info!("Working Internet connection detected");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("Error: {}", e);
            ExitCode::SUCCESS
        }
    };
}

async fn url_lookup() -> Result<()> {
    //TODO: Implement
    // return Err(anyhow::anyhow!("URL lookup failed"));

    Ok(())
}
