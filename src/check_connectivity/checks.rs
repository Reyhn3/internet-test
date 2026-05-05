use std::net::IpAddr;
use crate::ncsi;

pub struct Check {
    pub uri: String,
    pub expected_response: Option<String>,
//TODO: Remove or use
    pub proceed_on_error: bool
}

pub struct ConnectivityCheckResult {
    pub uri: String,
    pub dns_resolved: bool,
    pub get_succeeded: bool,
    pub content_matched: bool,
    pub ip: Option<IpAddr>
}

pub fn get_default_check_list() -> Vec<Check> {
    vec![
//TODO: Set these to real values
        Check { uri: ncsi::ms::MS_DNS_IPV4_HOST_AND_PORT.to_owned(), expected_response: Some("hello".to_string()), proceed_on_error: true },
        Check { uri: ncsi::ms::MS_WEB_IPV4_URL.to_owned(), expected_response: None, proceed_on_error: true }
    ]
}