use constcat::concat;

/*
 * According to Microsoft,
 * the host and path were changed starting with Windows 10 build 14393 (1607).
 * Both hosts are still active, though.
 *
 * Source:
 * https://learn.microsoft.com/en-us/windows-server/networking/ncsi/ncsi-frequently-asked-questions#where-is-the-http-web-probe-server-path-found-in-the-registry
 */

pub(super) const MS_DNS_IPV4_IP: &str = "131.107.255.255";
pub(super) const MS_DNS_IPV6_IP: &str = "fd3e:4f5a:5b81::1";

const MS_DNS_IPV4_HOST: &str = "dns.msftncsi.com";
const MS_DNS_IPV6_HOST: &str = "dns.msftncsi.com";

pub(super) const MS_WEB_IPV4_CONTENT: &str = "Microsoft Connect Test";
pub(super) const MS_WEB_IPV6_CONTENT: &str = "Microsoft Connect Test";
pub(super) const MS_WEB_IPV4_CONTENT_W10PRE1607: &str = "Microsoft NCSI";

const MS_WEB_IPV4_HOST: &str = "www.msftconnecttest.com";
const MS_WEB_IPV6_HOST: &str = "ipv6.msftconnecttest.com";
const MS_WEB_IPV4_HOST_W10PRE1607: &str = "www.msftncsi.com";
const MS_WEB_IPV6_HOST_W10PRE1607: &str = "ipv6.msftncsi.com";

const MS_WEB_IPV4_PATH: &str = "connecttest.txt";
const MS_WEB_IPV6_PATH: &str = "connecttest.txt";
const MS_WEB_IPV4_PATH_W10PRE1607: &str = "ncsi.txt";
const MS_WEB_IPV6_PATH_W10PRE1607: &str = "ncsi.txt";

pub(super) const MS_DNS_IPV4_HOST_AND_PORT: &str = concat!(
    MS_DNS_IPV4_HOST, ":80");
pub(super) const MS_DNS_IPV6_HOST_AND_PORT: &str = concat!(
    MS_DNS_IPV6_HOST, ":80");

pub(super) const MS_WEB_IPV4_HOST_AND_PORT: &str = concat!(
    MS_WEB_IPV4_HOST, ":80");
pub(super) const MS_WEB_IPV6_HOST_AND_PORT: &str = concat!(
    MS_WEB_IPV6_HOST, ":80");

pub(super) const MS_WEB_IPV4_URL: &str = concat!(
    "http://", MS_WEB_IPV4_HOST, "/", MS_WEB_IPV4_PATH);
pub(super) const MS_WEB_IPV6_URL: &str = concat!(
    "http://", MS_WEB_IPV6_HOST, "/", MS_WEB_IPV6_PATH);
