use std::net::{IpAddr, Ipv4Addr};
use crate::probing;

#[derive(Debug)]
pub struct NmCheck {
    pub uri: String,
    pub expected_response: Option<String>
}

#[derive(Debug)]
pub struct NcsiCheck {
    pub dns_first_host: String,
    pub web_uri: String,
    pub web_expected_response: String,
    pub dns_second_host: String,
    pub dns_second_expected_ip: IpAddr
}

#[derive(Debug)]
pub enum Check {
    Nm(NmCheck, Target),
    Ncsi(NcsiCheck, Target)
}

#[derive(clap::ValueEnum, Clone, Debug, Default, PartialEq, Eq)]
pub enum Target {
    /// The Arch Linux connectivity target.
    Arch,

    /// The Fedora connectivity target.
    Fedora,

    /// The NetworkManager connectivity target.
    Nm,

    #[default]
    #[value(alias("microsoft"))]
    /// The Microsoft connectivity target (alias: microsoft)
    Ncsi,

    /// The Ubuntu connectivity target.
    Ubuntu,

    /// Check all targets.
    All
}

pub struct ConnectivityCheckResult {
    pub dns_resolved: bool,
    pub get_succeeded: bool,
    pub content_matched: bool,
    pub ip: Option<IpAddr>
}

pub fn get_default_targets() -> Vec<Check> {
    vec![
        Check::Nm(NmCheck {
            uri: String::from("https://ping.archlinux.org/"),
            expected_response: Some(String::from("This domain is used for connectivity checking"))
        }, Target::Arch),
        Check::Nm(NmCheck {
            uri: String::from("http://nmcheck.gnome.org/check_network_status.txt"),
            expected_response: Some(String::from("NetworkManager is online"))
        }, Target::Nm),
        Check::Nm(NmCheck {
            uri: String::from("https://fedoraproject.org/static/hotspot.txt"),
            expected_response: Some(String::from("OK"))
        }, Target::Fedora),
        Check::Nm(NmCheck {
            uri: String::from("http://connectivity-check.ubuntu.com/"),
            expected_response: None
        }, Target::Ubuntu),
        Check::Ncsi(NcsiCheck {
            dns_first_host: String::from("www.msftconnecttest.com:80"),
            web_uri: String::from("http://www.msftconnecttest.com/connecttest.txt"),
            web_expected_response: String::from("Microsoft Connect Test"),
            dns_second_host: String::from("dns.msftncsi.com:80"),
            dns_second_expected_ip: IpAddr::from(Ipv4Addr::new(131, 107, 255, 255))
        }, Target::Ncsi)
    ]
}

pub(crate) fn get_target(check: &Check) -> &Target {
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