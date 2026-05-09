pub(crate) mod checks;
pub(crate) mod analysis;
mod strategy;

use anyhow::Result;
use log::debug;
use crate::check_connectivity::analysis::Analyzer;
use crate::check_connectivity::checks::Check;
use crate::check_connectivity::strategy::execute_strategy;
use crate::logging::{clear_scope, set_scope};

pub async fn check_internet_connectivity(checks: Vec<Check>) -> Result<analysis::Verdict> {
    let mut results = Vec::new();

    for (index, check) in checks.into_iter().enumerate() {
        set_scope(index);
//TODO: Log begin and end of check execution
//        debug!("Executing check: {}", &check);
        
        let result = execute_strategy(&check).await?;
        results.push(result);
        
        clear_scope();
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
                expected_response: Option::from("OK".to_string())
            }),
        ];
        let result = check_internet_connectivity(checks).await.unwrap();
        assert_eq!(result, analysis::Verdict::None);
    }
}
