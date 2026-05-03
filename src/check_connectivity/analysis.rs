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

//TODO: Change return type to constant
    pub(crate) fn analyze(&self) -> bool {
        if self.results.is_empty() {
            return false;
        }

        // Basic conclusion: all checks must succeed
        self.results.iter().all(|r| r.dns_resolved && r.get_succeeded && r.content_matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_check_result_all_failed_returns_false() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                dns_resolved: false,
                get_succeeded: false,
                content_matched: false,
                ip: Option::from("test-ip".to_string())
            },
        ];
        let verdict = Analyzer::new(results).analyze();
        assert!(!verdict);
    }
}