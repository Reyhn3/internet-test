use anyhow::Result;
use log::{debug, error, trace, warn};

use crate::check_connectivity::checks::{ConnectivityCheckResult, NmCheck};
use crate::check_connectivity::strategy::Strategy;
use crate::probing;

pub(crate) struct NmStrategy<'a> {
    check: &'a NmCheck,
}

impl<'a> NmStrategy<'a> {
    pub fn new(check: &'a NmCheck) -> Self {
        Self { check }
    }
}

impl<'a> Strategy for NmStrategy<'a> {
    async fn execute(&self) -> Result<ConnectivityCheckResult> {
        let dns_uri = probing::fqdn_with_port80(self.check.uri.as_str())?;

        let mut result = ConnectivityCheckResult {
            dns_resolved: false,
            get_succeeded: false,
            content_matched: false,
            ip: None
        };

        trace!("DNS resolution of web host started");
        match probing::resolve_dns(dns_uri.as_str()) {
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
        let content = match probing::request_web_content(self.check.uri.as_str()).await {
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
        if let Some(expected) = &self.check.expected_response {
            if content.is_none() {
                warn!("Expected content '{}' but none was received", expected);
            } else {
                if let Some(actual) = content && actual.starts_with(expected) {
                    debug!("Content matched");
                    result.content_matched = true;
                } else {
                    error!("Content did not match");
                }
            }
        } else {
            debug!("No content expected");
            result.content_matched = true;
        }

        Ok(result)
    }
}
