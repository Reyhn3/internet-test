use anyhow::Result;
use log::{debug, error, trace};
use crate::check_connectivity::checks::{Check, ConnectivityCheckResult};
use crate::probing;

pub(crate) struct Strategy;

impl Strategy {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn execute(&self, check: &Check) -> Result<ConnectivityCheckResult> {
        let mut result = ConnectivityCheckResult {
            uri: check.uri.clone(),
            dns_resolved: false,
            get_succeeded: false,
            content_matched: false,
            ip: None,
        };

        trace!("DNS resolution of web host started");
        match probing::resolve_dns(check.uri.as_str()) {
            Ok(ip) => {
                debug!("DNS resolution of web host succeeded: {}", ip);
                result.dns_resolved = true;
                result.ip = Some(ip);
            }
            Err(e) => {
                error!("DNS resolution of web host failed: {}", e);
                return Ok(result);
            }
        };

        trace!("Web request started");
        let content = match probing::request_web_content(check.uri.as_str()).await {
            Ok(c) => {
                debug!("Web request succeeded");
                result.get_succeeded = true;
                c
            }
            Err(e) => {
                error!("Web request failed: {}", e);
                return Ok(result);
            }
        };

        trace!("Checking content match");
        if let Some(expected) = &check.expected_response {
            if content.starts_with(expected) {
                result.content_matched = true;
            }
        } else {
            result.content_matched = true; 
        }

        debug!("Content matched: {}", result.content_matched);
        Ok(result)
    }
}