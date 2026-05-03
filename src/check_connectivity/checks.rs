pub struct Check {
    pub uri: String,
    pub expected_response: Option<String>,
    pub proceed_on_error: bool
}

pub struct ConnectivityCheckResult {
    pub uri: String,
    pub ip: Option<String>,
    pub dns: bool,
    pub success: bool,
}

pub fn get_default_check_list() -> Vec<Check> {
    vec![
        Check { uri: "first".to_string(), expected_response: Some("hello".to_string()), proceed_on_error: true },
        Check { uri: "second".to_string(), expected_response: None, proceed_on_error: true }
    ]
}