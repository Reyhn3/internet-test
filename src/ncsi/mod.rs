use anyhow::{bail, Result};
use log::{debug, info, trace, warn};
use std::process::ExitCode;

#[derive(PartialEq)]
enum Check {
    Success,
    Failure,
}

const NCSI_OUT_OF_OPTIONS: u8 = 0x02;

pub fn run_ncsi(error: bool) -> Result<ExitCode> {
    trace!("Initiating first URL-check");
    let first = url_lookup(false, error)?;
    debug!("Completed first URL-check");

    if first == Check::Success {
        info!("Successful first URL-check");
        return Ok(ExitCode::SUCCESS);
    } else {
        warn!("Failed first URL-check");
    }

    trace!("Initiating second URL-check");
    let second = url_lookup(true, error)?;
    debug!("Completed second URL-check");

    if second == Check::Success {
        info!("Successful second URL-check");
        return Ok(ExitCode::SUCCESS);
    } else {
        warn!("Failed second URL-check");
    }

    Ok(ExitCode::from(NCSI_OUT_OF_OPTIONS))
}

fn url_lookup(succeed: bool, error: bool) -> Result<Check> {
    if error {
        bail!("Something went bad")
    }

    if succeed {
        return Ok(Check::Success);
    }

    Ok(Check::Failure)
}
