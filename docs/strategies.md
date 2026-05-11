# Internet Connectivity Checking Strategies


# NCSI

Microsoft has developed a strategy called Network Connectivity Status Indicator (NCSI) to check internet connectivity. This method involves sending a request to a known website and checking the response to determine if the device is connected to the internet.

It performs a DNS lookup and a plain HTTP GET request to [http://www.msftconnecttest.com/connecttest.txt](http://www.msftconnecttest.com/connecttest.txt), checking for the text `Microsoft Connect Test`. A failure results in a "No Internet" icon.


## Procedure

1. NCSI sends a DNS request to resolve the address of the `www.msftconnecttest.com` FQDN.
1. If NCSI receives a valid response from a DNS server, NCSI sends a plain HTTP GET request to `http://www.msftconnecttest.com/connecttest.txt`.
1. If NCSI successfully downloads the text file, it makes sure that the file contains `Microsoft Connect Test`.
1. NCSI sends another DNS request to resolve the address of the `dns.msftncsi.com` FQDN.

If any of these requests fails, the network alert appears in the Task Bar with a message such as "No connectivity" or "Limited Internet access" (depending on which requests failed).

If all of these requests succeed, the Task Bar shows the usual network icon with a message such as "Internet access."

For the procedure used by Windows prior to Windows 10 version 1607, see the references below.


## Manual Verification

DNS lookup:
```bash
curl -vH "accept: application/dns-json" "https://cloudflare-dns.com/dns-query?name=www.msftconnecttest.com&type=AAAA"

# ...or...

dig www.msftconnecttest.com
```

HTTP request:
```bash
curl -iX GET http://www.msftconnecttest.com/connecttest.txt
```


## References

- [Microsoft NCSI Overview](https://learn.microsoft.com/en-us/windows-server/networking/ncsi/ncsi-overview)
- [Network Connection Status Indicator](https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#14-network-connection-status-indicator)
- [NCSI active probes and the network status alert](https://learn.microsoft.com/en-us/troubleshoot/windows-client/networking/internet-explorer-edge-open-connect-corporate-public-network#ncsi-active-probes-and-the-network-status-alert)


# NetworkManager

In Linux, the NetworkManager performs similar checks to Windows to determine network connectivity. It uses a combination of DNS and HTTP requests to verify internet access. The NetworkManager service is responsible for managing network connections and ensuring that the system has a stable and functional network connection.


## Procedure

1. Check if a default route exists.
   If no default route is found, it might be because of a misconfigured routing or a network without a gateway. The result is a "limited" connection.
1. Send an HTTP GET request to the configured URI.
   If this fails, a network exists but the Internet is not reachable. The result is a "limited" connection.
1. Analyze the response.
   * If the request was redirected, it indicates that there is a captive portal.
   * If the request was not redirected, but the HTTP response code is 200, it is assumed to be a portal unless the body matches an expected content.
   * If the HTTP response code is 204, connectivity is "full".
   * Any other response means that the connection is "limited".

The URI to check is configurable and is typically overridden by the distro. These are some of the URIs:

| Distro | URI | Status |
|--------|--|-----|
| Arch   | https://ping.archlinux.org/ | 200 |
| Gnome  | http://nmcheck.gnome.org/check_network_status.txt | 200 |
| Fedora | https://fedoraproject.org/static/hotspot.txt | 200 |
| Ubuntu | http://connectivity-check.ubuntu.com/ | 204 |


## Manual Verification

HTTP requests:
```bash
curl -iX GET https://ping.archlinux.org/
curl -iX GET http://nmcheck.gnome.org/check_network_status.txt
curl -iX GET https://fedoraproject.org/static/hotspot.txt
curl -iX GET http://connectivity-check.ubuntu.com/
```


## References

* [nm-connectivity.c](https://gitlab.freedesktop.org/NetworkManager/NetworkManager/-/blob/main/src/core/nm-connectivity.c)
* [Arch Linux documentation](https://wiki.archlinux.org/title/NetworkManager#Checking_connectivity)
