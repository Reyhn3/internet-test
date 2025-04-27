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
    // Step 1: Send a DNS request to resolve the web host.
    // Step 2: If valid, do a plain HTTP GET.
    // Step 3: Validate the contents.
    // Step 4: Send a DNS request to resolve the dns host.
    // Step 5: Evaluate requests.

//TODO: Return Internet Access, Limited Connectivity or No Internet Access.

    debug!("Running NCSI for IPv4");
    let status = probe_ipv4().await;
    debug!("NCSI for IPv4 completed with status {:?}", status);
    if status.is_ok() {
        return Ok(ExitCode::SUCCESS);
    }
    Ok(ExitCode::from(codes::NCSI_OUT_OF_OPTIONS))

//TODO: Learn how to simplify these statements
//     let web_check = active_web_probing().await?;
//     if web_check == Check::Success {
//         debug!("Active web probing succeeded");
//         return Ok(ExitCode::SUCCESS);
//     }
//     debug!("Active web probing failed");

    // let dns_check = active_dns_probing()?;
    // if dns_check == Check::Success {
    //     return Ok(ExitCode::SUCCESS);
    // }
    //
    // Ok(ExitCode::from(codes::NCSI_OUT_OF_OPTIONS))
}

async fn active_web_probing() -> Result<Check> {
//TODO: Implement

    // Probe IPv4 and IPv6 in parallel
    // If either succeeds, return success

//TODO: Get addresses
//     let url_v4_raw = format!("http://{}", MS_WEB_IPV4_HOST);
//     let url_v4_base = Url::parse(url_v4_raw.as_str())?;
//     let url_v4 = url_v4_base.join(ms::MS_WEB_IPV4_PATH)?;
    let url_v4 = Url::parse(ms::MS_WEB_IPV4_URL)?;

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
