use std::net::IpAddr;
use crate::probing;

pub struct NmCheck {
//TODO: Change to &str or Uri
    pub uri: String,
    pub expected_response: Option<String>
}

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

pub enum Check {
    Nm(NmCheck),
    Ncsi(NcsiCheck)
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum Strategy {
    // The Linux NetworkManager strategy.
    #[value(alias("linux"))]
    /// The Linux NetworkManager strategy (alias: linux)
    Nm,

    // The Microsoft connectivity strategy.
    #[default]
    #[value(alias("microsoft"))]
    /// The Microsoft connectivity strategy (alias: microsoft)
    Ncsi,

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
        }),
        Check::Nm(NmCheck {
            uri: String::from("http://nmcheck.gnome.org/check_network_status.txt"),
            expected_response: Some(String::from("NetworkManager is online"))
        }),
        Check::Nm(NmCheck {
            uri: String::from("https://fedoraproject.org/static/hotspot.txt"),
            expected_response: Some(String::from("OK"))
        }),
        Check::Nm(NmCheck {
            uri: String::from("http://connectivity-check.ubuntu.com/"),
            expected_response: None
        }),
        Check::Ncsi(NcsiCheck {
            dns_first_host: String::from("www.msftconnecttest.com:80"),
            web_uri: String::from("http://www.msftconnecttest.com/connecttest.txt"),
            web_expected_response: String::from("Microsoft Connect Test"),
            dns_second_host: String::from("dns.msftncsi.com:80"),
            dns_second_expected_ip: String::from("131.107.255.255")
        })
    ]
}

pub fn get_nm_check_list() -> Vec<Check> {
    vec![
        Check::Nm(NmCheck {
            uri: String::from("https://ping.archlinux.org/"),
            expected_response: Some(String::from("This domain is used for connectivity checking"))
        }),
        Check::Nm(NmCheck {
            uri: String::from("http://nmcheck.gnome.org/check_network_status.txt"),
            expected_response: Some(String::from("NetworkManager is online"))
        }),
        Check::Nm(NmCheck {
            uri: String::from("https://fedoraproject.org/static/hotspot.txt"),
            expected_response: Some(String::from("OK"))
        }),
        Check::Nm(NmCheck {
            uri: String::from("http://connectivity-check.ubuntu.com/"),
            expected_response: None
        }),
    ]
}

pub fn get_ncsi_check_list() -> Vec<Check> {
    vec![
        Check::Ncsi(NcsiCheck {
            dns_first_host: String::from("www.msftconnecttest.com:80"),
            web_uri: String::from("http://www.msftconnecttest.com/connecttest.txt"),
            web_expected_response: String::from("Microsoft Connect Test"),
            dns_second_host: String::from("dns.msftncsi.com:80"),
            dns_second_expected_ip: String::from("131.107.255.255")
        })
    ]
}

pub(crate) fn get_name(check: &Check) -> String {
    match check {
        Check::Nm(c) => probing::fqdn(c.uri.as_str()).unwrap_or(String::from("unknown")),
        Check::Ncsi(c) => probing::fqdn(c.web_uri.as_str()).unwrap_or(String::from("unknown"))
    }
}