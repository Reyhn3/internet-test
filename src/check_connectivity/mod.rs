pub(crate) mod checks;
pub(crate) mod analysis;
mod strategy;

use anyhow::Result;
use log::{debug, trace};
use tokio::time::Instant;
use crate::check_connectivity::analysis::Analyzer;
use crate::check_connectivity::checks::Check;
use crate::check_connectivity::strategy::execute_strategy;
use crate::logging::{clear_scope, set_scope};

pub async fn check_internet_connectivity(checks: Vec<Check>) -> Result<analysis::Verdict> {
    let mut results = Vec::new();
    let scoped = checks.len() > 1;

    for (index, check) in checks.into_iter().enumerate() {
        if scoped {
            set_scope(index);
        }

        debug!("Executing check: {}", checks::get_name(&check));
        let start = Instant::now();

        let result = execute_strategy(&check).await;
        results.push(result);

        let duration = start.elapsed();
        trace!("Executed check: {} in {:?}", checks::get_name(&check), duration);

        if scoped {
            clear_scope();
        }
    }

    let analyzer = Analyzer::new(results);
    Ok(analyzer.analyze())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_connectivity::checks::{NmCheck, Target};

//TODO: Refactor this test to actually test something
    #[tokio::test]
    async fn test_connectivity_check() {
        let checks = vec![
            Check::Nm(NmCheck {
                uri: "http://example.com".to_string(),
                expected_response: Option::from("OK".to_string())
            }, Target::Nm),
        ];
        let result = check_internet_connectivity(checks).await.unwrap();
        assert_eq!(result, analysis::Verdict::None);
    }
}
