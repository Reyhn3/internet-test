pub(crate) mod checks;
pub(crate) mod analysis;

use log::info;
use anyhow::Result;
use crate::check_connectivity::checks::{Check, ConnectivityCheckResult};
use crate::check_connectivity::analysis::{Analyzer};

pub async fn check_internet_connectivity(checks: Vec<Check>) -> Result<analysis::Verdict> {
    let mut results = Vec::new();

    for check in checks {
        let result = execute_strategy(&check).await?;
        results.push(result);
    }

    let analyzer = Analyzer::new(results);
    Ok(analyzer.analyze())
}

async fn execute_strategy(check: &Check) -> Result<ConnectivityCheckResult> {
    info!("Executing check: URI={}, Expected Response='{}'", check.uri, check.expected_response.as_deref().unwrap_or("None"));

    // For now, the strategy only logs and returns a success indicator.
    // Based on the requirement "return an Error-object that is failed in case of errors, 
    // or a boolean that indicates whether the check was successful or not."
    // We'll default to true for now since it's a mock implementation.
    Ok(ConnectivityCheckResult {
        uri: check.uri.clone(),
//TODO: Set these values
        dns_resolved: true,
        get_succeeded: true,
        content_matched: true,
        ip: Some("fake".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

//TODO: Refactor this test to actually test something
    #[tokio::test]
    async fn test_connectivity_check() {
        let checks = vec![
            Check {
                uri: "http://example.com".to_string(),
                expected_response: Option::from("OK".to_string()),
                proceed_on_error: true
            },
        ];
        let result = check_internet_connectivity(checks).await.unwrap();
        assert_eq!(result, analysis::Verdict::None);
    }
}
