use crate::check_connectivity::checks::ConnectivityCheckResult;

#[derive(PartialEq, Debug)]
pub(crate) enum Verdict {
    Error,
    None,
    Limited,
    Full,
}

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

    pub(crate) fn analyze(&self) -> Verdict {
        if self.results.is_empty() {
            return Verdict::Error;
        }

//TODO: Implement analysis
        // self.results.iter()
        //     .all(|r| r.dns_resolved && r.get_succeeded && r.content_matched)
        Verdict::None
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
        assert_eq!(verdict, Verdict::None);
    }
}