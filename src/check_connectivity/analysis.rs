use crate::check_connectivity::checks::ConnectivityCheckResult;

#[derive(PartialEq, Debug)]
pub(crate) enum Verdict {
    Error,
    None,
    Limited,
    Full,
}

pub(crate) struct Analyzer {
    results: Vec<ConnectivityCheckResult>
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

        let verdicts = self.results.iter()
            .map(|r| Self::analyze_check(r))
            .collect::<Vec<Verdict>>();

        let all_are_full = verdicts.iter().all(|v| v == &Verdict::Full);
        if all_are_full {
            return Verdict::Full;
        }

        let all_are_none = verdicts.iter().all(|v| v == &Verdict::None);
        if all_are_none {
            return Verdict::None;
        }

        Verdict::Limited
    }

    fn analyze_check(check: &ConnectivityCheckResult) -> Verdict {
        if !check.dns_resolved {
            return Verdict::None
        }
        if !check.get_succeeded {
            return Verdict::None
        }
        if !check.content_matched {
            return Verdict::Limited
        }

        Verdict::Full
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_check_with_all_result_failed_shall_return_verdict_none() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: false,
                get_succeeded: false,
                content_matched: false
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::None);
    }

    #[test]
    fn single_check_with_dns_resolved_failed_shall_return_verdict_none() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: false,
                get_succeeded: true,
                content_matched: true
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::None);
    }

    #[test]
    fn single_check_with_get_failed_shall_return_verdict_none() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: true,
                get_succeeded: false,
                content_matched: true
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::None);
    }

    #[test]
    fn single_check_with_content_failed_shall_return_verdict_limited() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: true,
                get_succeeded: true,
                content_matched: false
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::Limited);
    }

    #[test]
    fn single_check_with_all_passed_shall_return_verdict_full() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: true,
                get_succeeded: true,
                content_matched: true
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::Full);
    }

    #[test]
    fn multiple_checks_with_all_verdict_none_shall_return_aggregated_verdict_none() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: false,
                get_succeeded: false,
                content_matched: false
            },
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: false,
                get_succeeded: false,
                content_matched: false
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::None);
    }

    #[test]
    fn multiple_checks_with_any_verdict_full_shall_return_aggregated_verdict_limited() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: false,
                get_succeeded: false,
                content_matched: false
            },
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: true,
                get_succeeded: true,
                content_matched: true
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::Limited);
    }

    #[test]
    fn multiple_checks_with_all_verdict_full_shall_return_aggregated_verdict_full() {
        let results = vec![
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: true,
                get_succeeded: true,
                content_matched: true
            },
            ConnectivityCheckResult {
                uri: "test-uri".to_string(),
                ip: Option::from("test-ip".to_string()),
                dns_resolved: true,
                get_succeeded: true,
                content_matched: true
            }
        ];
        let verdict = Analyzer::new(results).analyze();
        assert_eq!(verdict, Verdict::Full);
    }
}