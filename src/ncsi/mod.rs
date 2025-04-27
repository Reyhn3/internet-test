mod codes;
mod ms;
mod probing;

use anyhow::Result;
use log::{debug, error, trace, warn};
use reqwest::Url;
use std::process::ExitCode;

#[derive(PartialEq, Debug)]
enum Check {
    Success,
    Failure,
}

#[derive(PartialEq, Debug)]
enum NcsiStatus {
    InternetAccess,
    LimitedConnectivity,
    NoInternetAccess,
}

/*
 * This method translates Result objects to an ExitCode.
 * Subsequent method calls are only allowed to return
 * Result with a Check status.
 */
pub async fn run_ncsi() -> Result<ExitCode> {
//TODO: Return Internet Access, Limited Connectivity or No Internet Access.

    debug!("Running NCSI for IPv4");
    let status = probe_ipv4().await;
    debug!("NCSI for IPv4 completed with status {:?}", status);
    if status.is_ok() {
        return Ok(ExitCode::SUCCESS);
    }
    Ok(ExitCode::from(codes::NCSI_OUT_OF_OPTIONS))
}

async fn probe_ipv4() -> Result<NcsiStatus> {
    trace!("DNS resolution of web host started");
    probing::resolve_dns(ms::MS_WEB_IPV4_HOST_AND_PORT).or_else(|e| {
        error!("DNS resolution of web host failed: {}", e);
        Err(e)
    })?;
    debug!("DNS resolution of web host succeeded");

    trace!("Web request started");
    let get_content = probing::request_web_content(ms::MS_WEB_IPV4_URL).await?;
    debug!("Web request succeeded");

    let content_match = match get_content.as_str() {
        ms::MS_WEB_IPV4_CONTENT => {
            debug!("Received content matches expected content");
            Ok(())
        },
        _ => {
            warn!("Unexpected content detected");
            Err(anyhow::anyhow!("Unexpected content in web request"))
        }
    };

    trace!("DNS resolution of DNS host started");
    let dns_ip = probing::resolve_dns(ms::MS_DNS_IPV4_HOST_AND_PORT)
        .or_else(|e| {
            error!("DNS resolution of DNS host failed: {}", e);
            Err(e)
        })?;
    trace!("DNS resolution of DNS host succeeded and found IP {}", dns_ip);
    if dns_ip.to_string().eq(ms::MS_DNS_IPV4_IP) {
        debug!("DNS IP matches expected IP");
    } else {
        warn!("DNS IP does not match expected IP");
    }
    debug!("DNS resolution of DNS host succeeded");

    if content_match.is_ok() {
        debug!("Internet access detected");
        return Ok(NcsiStatus::InternetAccess);
    }

    debug!("Limited connectivity detected");
    Ok(NcsiStatus::LimitedConnectivity)
}
