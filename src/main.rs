use anyhow::Result;
use clap::Parser;
use std::process::ExitCode;

/// Checks whether there is a working Internet connection.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Disable all output to the terminal.
    #[clap(long, short, action)]
    quiet: bool,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    let quiet = args.quiet;

    return match url_lookup(quiet).await {
        Ok(_) => {
            println!("Working Internet connection detected");
            ExitCode::SUCCESS
        },
        Err(e) => {
            println!("Error: {}", e);
            ExitCode::SUCCESS
        }
    }
}

async fn url_lookup(quiet: bool) -> Result<()> {
    return Err(anyhow::anyhow!("URL lookup failed"));

    Ok(())
}