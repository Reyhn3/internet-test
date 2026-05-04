use anyhow::Result;

use crate::check_connectivity::checks::{Check, ConnectivityCheckResult};

pub(crate) struct Strategy;

impl Strategy {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn execute(&self, check: &Check) -> Result<ConnectivityCheckResult> {
        Ok(ConnectivityCheckResult {
            uri: check.uri.clone(),
            dns_resolved: !check.uri.is_empty(),
            get_succeeded: !check.uri.is_empty(),
            content_matched: !check.uri.is_empty(),
            ip: Some("fake".to_string()),
        })
    }
}