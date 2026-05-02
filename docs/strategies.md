# Internet Connectivity Checking Strategies


## NCSI

Microsoft has developed a strategy called Network Connectivity Status Indicator (NCSI) to check internet connectivity. This method involves sending a request to a known website and checking the response to determine if the device is connected to the internet.

It performs a DNS lookup and a plain HTTP GET request to [http://www.msftconnecttest.com/connecttest.txt](http://www.msftconnecttest.com/connecttest.txt), checking for the text `Microsoft Connect Test`. A failure results in a "No Internet" icon.


### Procedure

1. NCSI sends a DNS request to resolve the address of the `www.msftconnecttest.com` FQDN.
1. If NCSI receives a valid response from a DNS server, NCSI sends a plain HTTP GET request to `http://www.msftconnecttest.com/connecttest.txt`.
1. If NCSI successfully downloads the text file, it makes sure that the file contains `Microsoft Connect Test`.
1. NCSI sends another DNS request to resolve the address of the `dns.msftncsi.com` FQDN.

If any of these requests fails, the network alert appears in the Task Bar with a message such as "No connectivity" or "Limited Internet access" (depending on which requests failed).

If all of these requests succeed, the Task Bar shows the usual network icon with a message such as "Internet access."

For the procedure used by Windows prior to Windows 10 version 1607, see the references below.


### Manual Verification

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


### References

- [Microsoft NCSI Overview](https://learn.microsoft.com/en-us/windows-server/networking/ncsi/ncsi-overview)
- [Network Connection Status Indicator](https://learn.microsoft.com/en-us/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services#14-network-connection-status-indicator)
- [NCSI active probes and the network status alert](https://learn.microsoft.com/en-us/troubleshoot/windows-client/networking/internet-explorer-edge-open-connect-corporate-public-network#ncsi-active-probes-and-the-network-status-alert)
