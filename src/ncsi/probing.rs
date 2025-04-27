use anyhow::anyhow;
use log::warn;
use log::trace;
use reqwest::StatusCode;
use std::net::{IpAddr, ToSocketAddrs};

pub(crate) fn resolve_dns(url: &str) -> anyhow::Result<IpAddr> {
    trace!("Resolving DNS address {}", url);

    let mut addrs = url.to_socket_addrs()?;
    addrs
        .find(|addr| addr.is_ipv4())
        .map(|addr| addr.ip())
        .map(|ip| {
            trace!("DNS address resolved to {}", ip);
            Ok(ip)
        })
        .ok_or(anyhow!("Failed to resolve DNS address"))
        .unwrap()
}

pub(crate) async fn request_web_content(url: &str) -> anyhow::Result<String> {
    trace!("Invoking GET request to {}", url);
    let result = reqwest::get(url).await?;
    trace!("Received response {}", result.status());

    if result.status() != StatusCode::OK {
        return Err(anyhow!("Received NOK status code {}", result.status()));
    }

    let content = result.text().await?;
    if content.is_empty() {
        warn!("Received empty content body");
        return Err(anyhow!("Web request body was empty"));
    }

    trace!("Received content '{}'", content);
    Ok(content)
}
