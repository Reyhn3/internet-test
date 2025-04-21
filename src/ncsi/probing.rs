use anyhow::bail;
use log::{debug, trace};
use reqwest::{StatusCode, Url};
use crate::ncsi::Check;

//TODO: Remove when done debugging
pub(crate) fn url_lookup(succeed: bool, error: bool) -> anyhow::Result<Check> {
    if error {
        bail!("Something went bad")
    }

    if succeed {
        return Ok(Check::Success);
    }

    Ok(Check::Failure)
}

pub(crate) async fn invoke_web_request() -> anyhow::Result<Check> {
    const url: &str = "http://www.msftncsi.com/ncsi.txt";

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

    if content != "Microsoft NCSI" {
        debug!("NCSI response body not as expected");
        return Ok(Check::Failure);
    }

    debug!("NCSI response body as expected");
    Ok(Check::Success)
}