use std::net::{IpAddr, ToSocketAddrs};
use crate::ncsi::{ms, Check};
use anyhow::{anyhow, bail};
use log::{debug, trace};
use reqwest::{StatusCode, Url};

pub(crate) async fn invoke_web_request(url: Url) -> anyhow::Result<Check> {
    trace!("Invoking web request: {}", url);
    let result = reqwest::get(url).await?;
    debug!("Received response {}", result.status());

    if result.status() != StatusCode::OK {
        return Ok(Check::Failure);
    }

    let content = result.text().await?;
    if content.is_empty() {
        debug!("Received empty content body");
        return Ok(Check::Failure);
    }
    trace!("Received content '{}'", content);

    if !content.eq(ms::MS_WEB_IPV4_CONTENT) {
        debug!("NCSI response body not as expected");
        return Ok(Check::Failure);
    }

    debug!("NCSI response body as expected");
    Ok(Check::Success)
}

pub(crate) fn resolve_dns(url: &str) -> anyhow::Result<()> {
    trace!("Resolving DNS address: {}", url);
    
    // match url.to_socket_addrs() {
    //     Ok(_) => {
    //         debug!("DNS address resolved");       
    //         Ok(())
    //     },
    //     Err(_) => {
    //         debug!("DNS address was not resolved");       
    //         bail!("Failed to resolve DNS address")
    //     }
    // }

    let mut addrs = url.to_socket_addrs()?;
    addrs
        .find(|addr| addr.is_ipv4())
        .map(|addr| addr.ip())
        .map(|ip| {
            debug!("DNS address resolved to: {}", ip);
            Ok(())
        })
        .ok_or(anyhow!("Failed to resolve DNS address"))
        .unwrap()
}

pub(crate) async fn request_web_content(url: &str) -> anyhow::Result<String> {
    trace!("Invoking web request: {}", url);
    let result = reqwest::get(url).await?;
    debug!("Received response {}", result.status());

    if result.status() != StatusCode::OK {
        return Err(anyhow!("Received status code {}", result.status()));       
    }

    let content = result.text().await?;
    if content.is_empty() {
        debug!("Received empty content body");
        return Err(anyhow!("Received empty content body"));       
    }
    
    trace!("Received content '{}'", content);
    Ok(content)
}