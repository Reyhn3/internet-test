use crate::check_connectivity::checks::ConnectivityCheckResult;

pub(crate) struct Analyzer {
    results: Vec<ConnectivityCheckResult>,
}

impl Analyzer {
    pub(crate) fn new(results: Vec<ConnectivityCheckResult>) ->
     Self {
        Self {
            results
        }
    }

    pub(crate) fn analyze(&self) -> bool {
        if self.results.is_empty() {
            return false;
        }
        // Basic conclusion: all checks must succeed
        self.results.iter().all(|r| r.success)
    }
}
