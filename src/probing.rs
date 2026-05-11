use anyhow::{anyhow, Context, Result};
use log::{debug, warn};
use log::trace;
use reqwest::StatusCode;
use std::net::{IpAddr, ToSocketAddrs};

pub(crate) fn resolve_dns(url: &str) -> Result<IpAddr> {
    trace!("Resolving DNS address {}", url);
    url.to_socket_addrs()?
        .find(|addr| addr.is_ipv4())
        .map(|addr| addr.ip())
        .map(|ip| {
            trace!("DNS address resolved to {}", ip);
            Ok(ip)
        })
        .ok_or(anyhow!("Failed to resolve DNS address"))?
}

pub(crate) async fn request_web_content(url: &str) -> Result<Option<String>> {
    trace!("Invoking GET request to {}", url);
    let result = reqwest::get(url).await?;
    debug!("Received response {}", result.status());

    if result.status() != StatusCode::OK && result.status() != StatusCode::NO_CONTENT {
        return Err(anyhow!("Received NOK status code {}", result.status()));
    }

    if result.status() == StatusCode::NO_CONTENT {
        trace!("Received NO_CONTENT status code, returning empty string");
        return Ok(None);
    }

    let content = result.text().await?;
    if content.is_empty() {
        warn!("Received empty content body");
        return Err(anyhow!("Web request body was empty"));
    }

//TODO: Get only the first 50 chars (if more)
    trace!("Received content '{}'", content);
    Ok(Some(content))
}

pub(crate) fn fqdn(input: &str) -> Result<String> {
    let url = reqwest::Url::parse(input)
        .with_context(|| format!("Invalid URL: {input}"))?;

    let host = url
        .host_str()
        .context("URL does not contain a host")?;

    Ok(host.to_string())
}

pub(crate) fn fqdn_with_port80(input: &str) -> Result<String> {
    let host = fqdn(input)?;

    Ok(format!("{host}:80"))
}
