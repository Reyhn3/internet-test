use anyhow::Result;
use log::{debug, error, trace, warn};
use crate::check_connectivity::checks::{Check, ConnectivityCheckResult, NmCheck, NcsiCheck};
use crate::probing;

pub(crate) trait Strategy {
    async fn execute(&self) -> Result<ConnectivityCheckResult>;
}

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
            uri: self.check.uri.clone(),
            dns_resolved: false,
            get_succeeded: false,
            content_matched: false,
            ip: None,
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

pub(crate) struct NcsiStrategy<'a> {
    check: &'a NcsiCheck,
}

impl<'a> NcsiStrategy<'a> {
    pub fn new(check: &'a NcsiCheck) -> Self {
        Self { check }
    }
}

impl<'a> Strategy for NcsiStrategy<'a> {
    async fn execute(&self) -> Result<ConnectivityCheckResult> {
        let mut result = ConnectivityCheckResult {
            uri: self.check.web_uri.clone(),
            dns_resolved: false,
            get_succeeded: false,
            content_matched: false,
            ip: None,
        };

        trace!("DNS resolution of web host started");
        match probing::resolve_dns(self.check.dns_first_host.as_str()) {
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

        trace!("NCSI Web request started");
        let content = match probing::request_web_content(self.check.web_uri.as_str()).await {
            Ok(c) => {
                debug!("NCSI Web request succeeded");
                result.get_succeeded = true;
                c
            }
            Err(e) => {
                error!("NCSI Web request failed: {}", e);
                return Ok(result);
            }
        };

        trace!("NCSI Checking content match");
        if content.is_none() {
            error!("NCSI content was expected but is missing");
            return Ok(result);
        } else {
            if let Some(actual) = content && actual.starts_with(&self.check.web_expected_response) {
                debug!("NCSI content matched");
                result.content_matched = true;
            } else {
                debug!("NCSI content did NOT match");
                result.content_matched = false;
                // In NCSI, if web content doesn't match, it might be limited connectivity.
                // But for ConnectivityCheckResult, we just report what happened.
                return Ok(result);
            }
        }

        trace!("NCSI DNS resolution of DNS host started");
        match probing::resolve_dns(self.check.dns_second_host.as_str()) {
            Ok(dns_ip) => {
                debug!("NCSI DNS resolution of DNS host succeeded and found IP {}", dns_ip);
                if dns_ip.to_string().eq(&self.check.dns_second_expected_ip) {
                    debug!("NCSI DNS IP matches expected IP");
                } else {
                    debug!("NCSI DNS IP does not match expected IP");
                    // We don't have a specific field for this in ConnectivityCheckResult yet,
                    // but we could say it failed if we want to be strict.
                    // For now, let's keep it as is.
                }
            }
            Err(e) => {
                error!("NCSI DNS resolution of DNS host failed: {}", e);
                // Should we fail the whole check?
            }
        }

        Ok(result)
    }
}

pub(crate) async fn execute_strategy(check: &Check) -> Result<ConnectivityCheckResult> {
    match check {
        Check::Nm(nm_check) => {
            let strategy = NmStrategy::new(nm_check);
            strategy.execute().await
        }
        Check::Ncsi(ncsi_check) => {
            let strategy = NcsiStrategy::new(ncsi_check);
            strategy.execute().await
        }
    }
}
