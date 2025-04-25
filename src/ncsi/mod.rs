mod codes;
mod ms;
mod probing;

use anyhow::{bail, Result};
use log::{debug, info, trace, warn};
use reqwest::Url;
use std::process::ExitCode;
use crate::ncsi::ms::{MS_DNS_IPV4_HOST, MS_WEB_IPV4_HOST};

#[derive(PartialEq)]
#[derive(Debug)]
enum Check {
    Success,
    Failure,
}

/*
 * This method translates Result objects to an ExitCode.
 * Subsequent method calls are only allowed to return
 * Result with a Check status.
 */
pub async fn run_ncsi(error: bool) -> Result<ExitCode> {

    // Step 1: Send a DNS request to resolve the web host.
    // Step 2: If valid, do a plain HTTP GET.
    // Step 3: Validate the contents.
    // Step 4: Send a DNS request to resolve the dns host.
    // Step 5: Evaluate requests.
    
//TODO: Return Internet Access, Limited Connectivity or No Internet Access.
    
    
//TODO: Learn how to simplify these statements
    let web_check = active_web_probing().await?;
    if web_check ==  Check::Success {
        debug!("Active web probing succeeded");
        return Ok(ExitCode::SUCCESS);
    }
    debug!("Active web probing failed");

    let dns_check = active_dns_probing()?;
    if dns_check ==  Check::Success {
        return Ok(ExitCode::SUCCESS);
    }

    Ok(ExitCode::from(codes::NCSI_OUT_OF_OPTIONS))
}

async fn active_web_probing() -> Result<Check> {
//TODO: Implement

    // Probe IPv4 and IPv6 in parallel
    // If either succeeds, return success

//TODO: Get addresses
    let url_v4_raw = format!("http://{}", MS_WEB_IPV4_HOST);
    let url_v4_base = Url::parse(url_v4_raw.as_str())?;
    let url_v4 = url_v4_base.join(ms::MS_WEB_IPV4_PATH)?;

//TODO: Invoke in parallel

    let ipv4check = probing::invoke_web_request(url_v4).await?;
    debug!("Active web probe result is {:?}", ipv4check);
    if ipv4check == Check::Success {
        return Ok(Check::Success);
    }

    Ok(Check::Failure)
}

fn active_dns_probing() -> Result<Check> {
//TODO: Implement

    // Attempt DNS-resolution on IPv4 and IPv6 in parallel
    // If either succeeds, return success

    Ok(Check::Failure)
}

//TODO: Remove when done debugging
fn demo_probing(error : bool) -> Result<Check> {
    if !cfg!(debug_assertions) {
        return  Ok(Check::Success);
    }

    trace!("Initiating first URL-check");
    let first = probing::url_lookup(false, error)?;
    debug!("Completed first URL-check");

    if first == Check::Success {
        info!("Successful first URL-check");
        return Ok(Check::Success);
    } else {
        warn!("Failed first URL-check");
    }

    trace!("Initiating second URL-check");
    let second = probing::url_lookup(true, error)?;
    debug!("Completed second URL-check");

    if second == Check::Success {
        info!("Successful second URL-check");
        return Ok(Check::Success);
    } else {
        warn!("Failed second URL-check");
    }

    return Ok(Check::Failure);
}

