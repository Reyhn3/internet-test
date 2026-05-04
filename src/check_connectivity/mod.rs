pub(crate) mod checks;
pub(crate) mod analysis;
mod strategy;

use log::info;
use anyhow::Result;
use crate::check_connectivity::analysis::Analyzer;
use crate::check_connectivity::checks::{Check, ConnectivityCheckResult};
use crate::check_connectivity::strategy::Strategy;

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
    info!(
        "Executing check: URI={}, Expected Response='{}'",
        check.uri,
        check.expected_response.as_deref().unwrap_or("None")
    );

    let strategy = Strategy::new();
    strategy.execute(check).await
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
