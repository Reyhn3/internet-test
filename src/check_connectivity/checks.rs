use std::net::IpAddr;
use crate::probing;

#[derive(Debug)]
pub struct NmCheck {
//TODO: Change to &str or Uri
    pub uri: String,
    pub expected_response: Option<String>
}

#[derive(Debug)]
pub struct NcsiCheck {
//TODO: Change to &str or Uri
    pub dns_first_host: String,
//TODO: Change to &str or Uri
    pub web_uri: String,
    pub web_expected_response: String,
//TODO: Change to &str or Uri
    pub dns_second_host: String,
//TODO: Change to IpAddr
    pub dns_second_expected_ip: String
}

#[derive(Debug)]
pub enum Check {
    Nm(NmCheck, Strategy),
    Ncsi(NcsiCheck, Strategy)
}

#[derive(clap::ValueEnum, Clone, Debug, Default, PartialEq, Eq)]
pub enum Strategy {
    /// The Arch Linux strategy
    Arch,

    /// The Fedora strategy.
    Fedora,

    /// The NetworkManager strategy.
    Nm,

    #[default]
    #[value(alias("microsoft"))]
    /// The Microsoft connectivity strategy (alias: microsoft)
    Ncsi,

    /// The Ubuntu strategy.
    Ubuntu,

    /// Run all checks
    All
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
        Check::Nm(NmCheck {
            uri: String::from("https://ping.archlinux.org/"),
            expected_response: Some(String::from("This domain is used for connectivity checking"))
        }, Strategy::Arch),
        Check::Nm(NmCheck {
            uri: String::from("http://nmcheck.gnome.org/check_network_status.txt"),
            expected_response: Some(String::from("NetworkManager is online"))
        }, Strategy::Nm),
        Check::Nm(NmCheck {
            uri: String::from("https://fedoraproject.org/static/hotspot.txt"),
            expected_response: Some(String::from("OK"))
        }, Strategy::Fedora),
        Check::Nm(NmCheck {
            uri: String::from("http://connectivity-check.ubuntu.com/"),
            expected_response: None
        }, Strategy::Ubuntu),
        Check::Ncsi(NcsiCheck {
            dns_first_host: String::from("www.msftconnecttest.com:80"),
            web_uri: String::from("http://www.msftconnecttest.com/connecttest.txt"),
            web_expected_response: String::from("Microsoft Connect Test"),
            dns_second_host: String::from("dns.msftncsi.com:80"),
            dns_second_expected_ip: String::from("131.107.255.255")
        }, Strategy::Ncsi)
    ]
}

pub(crate) fn get_strategy(check: &Check) -> &Strategy {
    match check {
        Check::Nm(_, s) => s,
        Check::Ncsi(_, s) => s
    }
}

pub(crate) fn get_name(check: &Check) -> String {
    match check {
        Check::Nm(c, _) => probing::fqdn(c.uri.as_str()).unwrap_or(String::from("unknown")),
        Check::Ncsi(c, _) => probing::fqdn(c.web_uri.as_str()).unwrap_or(String::from("unknown"))
    }
}