use anyhow::Result;
use chrono::Local;
use clap::Parser;
use log::{debug, error, info, log_enabled, trace, warn, Level, LevelFilter};
use std::fmt::{Debug, Display};
use std::io::Write;
use std::process::ExitCode;
use std::time::SystemTime;

/// Checks whether there is a working Internet connection.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
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

    init_logger(quiet, verbose);

//TODO: Remove
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

    return match url_lookup(quiet).await {
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

async fn url_lookup(quiet: bool) -> Result<()> {
//TODO: Implement
    // return Err(anyhow::anyhow!("URL lookup failed"));

    Ok(())
}

//TODO: Move to another file
const DATE_FORMAT_STR: &'static str = "%H:%M:%S";

//TODO: Move to another file
fn init_logger(quiet: bool, verbose: bool) {
    if quiet {
        env_logger::builder().filter_level(LevelFilter::Off).init();
        return;
    }

    if !verbose {
        env_logger::builder()
            .format(|buf, record| {
                let style = buf.default_level_style(record.level());
                writeln!(buf, "{style}{}{style:#}", record.args())
            })
            .init();
        return;
    }

    env_logger::builder()
        .format(|buf, record| {
            let style = buf.default_level_style(record.level());
            let timestamp = Local::now().format(DATE_FORMAT_STR);
            let pad = match record.level() {
                Level::Info | Level::Warn => "  ",
                _ => " ",
            };

            writeln!(
                buf,
                "{timestamp} [{}]{pad}{style}{}{style:#}",
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::max())
        .format_target(false)
        .init();
}
