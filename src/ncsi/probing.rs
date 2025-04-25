use crate::ncsi::{ms, Check};
use anyhow::bail;
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
