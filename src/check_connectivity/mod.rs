pub(crate) mod checks;
pub(crate) mod analysis;
mod strategy;

use anyhow::Result;
use crate::check_connectivity::analysis::Analyzer;
use crate::check_connectivity::checks::Check;
use crate::check_connectivity::strategy::execute_strategy;

pub async fn check_internet_connectivity(checks: Vec<Check>) -> Result<analysis::Verdict> {
    let mut results = Vec::new();

    for check in checks {
        let result = execute_strategy(&check).await?;
        results.push(result);
    }

    let analyzer = Analyzer::new(results);
    Ok(analyzer.analyze())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_connectivity::checks::NmCheck;

//TODO: Refactor this test to actually test something
    #[tokio::test]
    async fn test_connectivity_check() {
        let checks = vec![
            Check::Nm(NmCheck {
                uri: "http://example.com".to_string(),
                expected_response: Option::from("OK".to_string()),
                proceed_on_error: true
            }),
        ];
        let result = check_internet_connectivity(checks).await.unwrap();
        assert_eq!(result, analysis::Verdict::None);
    }
}
