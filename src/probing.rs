use anyhow::{anyhow, Context, Result};
use log::{debug, warn};
use log::trace;
use reqwest::{Response, StatusCode};
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
    let mut response = reqwest::get(url).await?;
    debug!("Received response {}", response.status());

    if response.status() != StatusCode::OK && response.status() != StatusCode::NO_CONTENT {
        return Err(anyhow!("Received NOK status code {}", response.status()));
    }

    if response.status() == StatusCode::NO_CONTENT {
        trace!("Received NO_CONTENT status code, returning empty string");
        return Ok(None);
    }

    let content = download_leading_body(&mut response).await?;
    if content.is_empty() {
        warn!("Received empty content body");
        return Err(anyhow!("Web request body was empty"));
    }

    trace!("Received content '{}'", content);
    Ok(Some(content))
}

async fn download_leading_body(response: &mut Response) -> Result<String> {
    let mut content = String::new();
    while let Some(chunk) = response.chunk().await? {
        let chunk_str = String::from_utf8_lossy(&chunk);
        for c in chunk_str.chars() {
            content.push(c);
            if content.chars().count() >= 50 {
                break;
            }
        }
        if content.chars().count() >= 50 {
            break;
        }
    }
    Ok(content)
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
