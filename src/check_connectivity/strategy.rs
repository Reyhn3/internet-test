use anyhow::Result;

use crate::check_connectivity::checks::{Check, ConnectivityCheckResult};
use crate::check_connectivity::strategy::ncsi::NcsiStrategy;
use crate::check_connectivity::strategy::nm::NmStrategy;

mod ncsi;
mod nm;

pub(crate) trait Strategy {
    async fn execute(&self) -> Result<ConnectivityCheckResult>;
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
