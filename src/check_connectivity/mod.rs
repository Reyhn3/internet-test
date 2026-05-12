pub(crate) mod checks;
pub(crate) mod analysis;
mod strategy;

use anyhow::Result;
use log::debug;
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

        debug!("Beginning checking target {}", checks::get_name(&check));
        let start = Instant::now();

        let result = execute_strategy(&check).await;
        results.push(result);

        let duration = start.elapsed();
        debug!("Finished checking target {} in {:?}", checks::get_name(&check), duration);

        if scoped {
            clear_scope();
        }
    }

    let analyzer = Analyzer::new(results);
    Ok(analyzer.analyze())
}
