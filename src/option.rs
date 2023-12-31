use std::net::Ipv4Addr;
use std::str::from_utf8;

use crate::error::DhcpError;

#[derive(Debug, PartialEq)]
pub enum DhcpOption {
    // Pad Option
    //
    // The pad option can be used to cause subsequent fields to align on
    // word boundaries.
    //
    // The code for the pad option is 0, and its length is 1 octet.
    //
    //  Code
    // +-----+
    // |  0  |
    // +-----+
    Pad,
    // End Option
    //
    // The end option marks the end of valid information in the vendor
    // field. Subsequent octets should be filled with pad options.
    //
    // The code for the end option is 255, and its length is 1 octet.
    //
    //  Code
    // +-----+
    // | 255 |
    // +-----+
    End,
    // Subnet Mask
    //
    // The subnet mask option specifies the client's subnet mask as per RFC
    // 950.
    //
    // If both the subnet mask and the router option are specified in a DHCP
    // reply, the subnet mask option MUST be first.
    //
    // The code for the subnet mask option is 1, and its length is 4 octets.
    //
    //  Code   Len        Subnet Mask
    // +-----+-----+-----+-----+-----+-----+
    // |  1  |  4  |  m1 |  m2 |  m3 |  m4 |
    // +-----+-----+-----+-----+-----+-----+
    SubnetMask(Ipv4Addr),
    // Time Offset
    //
    // The time offset field specifies the offset of the client's subnet in
    // seconds from Coordinated Universal Time (UTC). The offset is
    // expressed as a two's complement 32-bit integer. A positive offset
    // indicates a location east of the zero meridian and a negative offset
    // indicates a location west of the zero meridian.
    //
    // The code for the time offset option is 2, and its length is 4 octets.
    //
    //  Code   Len        Time Offset
    // +-----+-----+-----+-----+-----+-----+
    // |  2  |  4  |  n1 |  n2 |  n3 |  n4 |
    // +-----+-----+-----+-----+-----+-----+
    TimeOffset(u32),
    // Router Option
    //
    // The router option specifies a list of IP addresses for routers on the
    // client's subnet.  Routers SHOULD be listed in order of preference.
    //
    // The code for the router option is 3.  The minimum length for the
    // router option is 4 octets, and the length MUST always be a multiple
    // of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  3  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    Router(Vec<Ipv4Addr>),
    // Time Server Option
    //
    // The time server option specifies a list of RFC 868 time servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    // The code for the time server option is 4. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  4  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    TimeServer(Vec<Ipv4Addr>),
    // Name Server Option
    //
    // The name server option specifies a list of IEN 116 name servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    // The code for the name server option is 5. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  5  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    NameServer(Vec<Ipv4Addr>),
    // Domain Name Server Option
    //
    // The domain name server option specifies a list of Domain Name System
    // (STD 13, RFC 1035) name servers available to the client. Servers
    // SHOULD be listed in order of preference.
    //
    // The code for the domain name server option is 6. The minimum length
    // for this option is 4 octets, and the length MUST always be a multiple
    // of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  6  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    DomainNameServer(Vec<Ipv4Addr>),
    // Log Server Option
    //
    // The log server option specifies a list of MIT-LCS UDP log servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    // The code for the log server option is 7. The minimum length for this
    // option is 4 octets, and the length MUST always be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  7  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    LogServer(Vec<Ipv4Addr>),
    // Cookie Server Option
    //
    // The cookie server option specifies a list of RFC 865 cookie
    // servers available to the client. Servers SHOULD be listed in order
    // of preference.
    //
    // The code for the log server option is 8. The minimum length for this
    // option is 4 octets, and the length MUST always be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  8  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    CookieServer(Vec<Ipv4Addr>),
    // LPR Server Option
    //
    // The LPR server option specifies a list of RFC 1179 line printer
    // servers available to the client. Servers SHOULD be listed in order
    // of preference.
    //
    // The code for the LPR server option is 9. The minimum length for this
    // option is 4 octets, and the length MUST always be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  9  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    LprServer(Vec<Ipv4Addr>),
    // Impress Server Option
    //
    // The Impress server option specifies a list of Imagen Impress servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    // The code for the Impress server option is 10. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  10 |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    ImpressServer(Vec<Ipv4Addr>),
    // Resource Location Server Option
    //
    // This option specifies a list of RFC 887 Resource Location
    // servers available to the client. Servers SHOULD be listed in order
    // of preference.
    //
    // The code for this option is 11. The minimum length for this option
    // is 4 octets, and the length MUST always be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  11 |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    ResourceLocationServer(Vec<Ipv4Addr>),
    // Host Name Option
    //
    // This option specifies the name of the client. The name may or may
    // not be qualified with the local domain name. See RFC 1035 for
    // character set restrictions.
    //
    // The code for this option is 12, and its minimum length is 1.
    //
    //  Code   Len                 Host Name
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  12 |  n  |  h1 |  h2 |  h3 |  h4 |  h5 |  h6 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    HostName(String),
    // Boot File Size Option
    //
    // This option specifies the length in 512-octet blocks of the default
    // boot image for the client.  The file length is specified as an
    // unsigned 16-bit integer.
    //
    // The code for this option is 13, and its length is 2.
    //
    //  Code   Len   File Size
    // +-----+-----+-----+-----+
    // |  13 |  2  |  l1 |  l2 |
    // +-----+-----+-----+-----+
    BootFileSize(u16),
    // Merit Dump File
    //
    // This option specifies the path-name of a file to which the client's
    // core image should be dumped in the event the client crashes. The
    // path is formatted as a character string consisting of characters from
    // the NVT ASCII character set.
    //
    // The code for this option is 14. Its minimum length is 1.
    //
    //  Code   Len      Dump File Pathname
    // +-----+-----+-----+-----+-----+-----+---
    // |  14 |  n  |  n1 |  n2 |  n3 |  n4 | ...
    // +-----+-----+-----+-----+-----+-----+---
    MeritDumpFile(String),
    // Domain Name
    //
    // This option specifies the domain name that client should use when
    // resolving hostnames via the Domain Name System.
    //
    // The code for this option is 15. Its minimum length is 1.
    //
    //  Code   Len        Domain Name
    // +-----+-----+-----+-----+-----+-----+--
    // |  15 |  n  |  d1 |  d2 |  d3 |  d4 |  ...
    // +-----+-----+-----+-----+-----+-----+--
    DomainName(String),
    // Swap Server
    //
    // This specifies the IP address of the client's swap server.
    //
    // The code for this option is 16 and its length is 4.
    //
    //  Code   Len    Swap Server Address
    // +-----+-----+-----+-----+-----+-----+
    // |  16 |  n  |  a1 |  a2 |  a3 |  a4 |
    // +-----+-----+-----+-----+-----+-----+
    SwapServer(Ipv4Addr),
    // Root Path
    //
    // This option specifies the path-name that contains the client's root
    // disk. The path is formatted as a character string consisting of
    // characters from the NVT ASCII character set.
    //
    // The code for this option is 17. Its minimum length is 1.
    //
    //  Code   Len      Root Disk Pathname
    // +-----+-----+-----+-----+-----+-----+---
    // |  17 |  n  |  n1 |  n2 |  n3 |  n4 | ...
    // +-----+-----+-----+-----+-----+-----+---
    RootPath(String),
    // Extensions Path
    //
    // A string to specify a file, retrievable via TFTP, which contains
    // information which can be interpreted in the same way as the 64-octet
    // vendor-extension field within the BOOTP response, with the following
    // exceptions:
    //
    //  - the length of the file is unconstrained;
    //  - all references to Tag 18 (i.e., instances of the
    //    BOOTP Extensions Path field) within the file are
    //    ignored.
    //
    // The code for this option is 18. Its minimum length is 1.
    //
    //  Code   Len      Extensions Pathname
    // +-----+-----+-----+-----+-----+-----+---
    // |  18 |  n  |  n1 |  n2 |  n3 |  n4 | ...
    // +-----+-----+-----+-----+-----+-----+---
    ExtensionsPath(String),
    // IP Forwarding Enable/Disable Option
    //
    // This option specifies whether the client should configure its IP
    // layer for packet forwarding. A value of 0 means disable IP
    // forwarding, and a value of 1 means enable IP forwarding.
    //
    // The code for this option is 19, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  19 |  1  | 0/1 |
    // +-----+-----+-----+
    IpForwarding(bool),
    // Non-Local Source Routing Enable/Disable Option
    //
    // This option specifies whether the client should configure its IP
    // layer to allow forwarding of datagrams with non-local source routes.
    // A value of 0 means disallow forwarding of such datagrams, and a value
    // of 1 means allow forwarding.
    //
    // The code for this option is 20, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  20 |  1  | 0/1 |
    // +-----+-----+-----+
    NonLocalSourceRouting(bool),
    // Policy Filter Option
    //
    // This option specifies policy filters for non-local source routing.
    // The filters consist of a list of IP addresses and masks which specify
    // destination/mask pairs with which to filter incoming source routes.
    //
    // Any source routed datagram whose next-hop address does not match one
    // of the filters should be discarded by the client.
    //
    // The code for this option is 21. The minimum length of this option is
    // 8, and the length MUST be a multiple of 8.
    //
    //  Code   Len         Address 1                  Mask 1
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
    // |  21 |  n  |  a1 |  a2 |  a3 |  a4 |  m1 |  m2 |  m3 |  m4 |
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
    //         Address 2                  Mask 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    // |  a1 |  a2 |  a3 |  a4 |  m1 |  m2 |  m3 |  m4 | ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    PolicyFilter(Vec<(Ipv4Addr, Ipv4Addr)>),
    // Maximum Datagram Reassembly Size
    //
    // This option specifies the maximum size datagram that the client
    // should be prepared to reassemble. The size is specified as a 16-bit
    // unsigned integer. The minimum value legal value is 576.
    //
    // The code for this option is 22, and its length is 2.
    //
    //  Code   Len      Size
    // +-----+-----+-----+-----+
    // |  22 |  2  |  s1 |  s2 |
    // +-----+-----+-----+-----+
    MaximumDatagramReassemblySize(u16),
    // Default IP Time-to-live
    //
    // This option specifies the default time-to-live that the client should
    // use on outgoing datagrams. The TTL is specified as an octet with a
    // value between 1 and 255.
    //
    // The code for this option is 23, and its length is 1.
    //
    //  Code   Len   TTL
    // +-----+-----+-----+
    // |  23 |  1  | ttl |
    // +-----+-----+-----+
    DefaultIpTimeToLive(u8),
    // Path MTU Aging Timeout Option
    //
    // This option specifies the timeout (in seconds) to use when aging Path
    // MTU values discovered by the mechanism defined in RFC 1191. The
    // timeout is specified as a 32-bit unsigned integer.
    //
    // The code for this option is 24, and its length is 4.
    //
    //  Code   Len           Timeout
    // +-----+-----+-----+-----+-----+-----+
    // |  24 |  4  |  t1 |  t2 |  t3 |  t4 |
    // +-----+-----+-----+-----+-----+-----+
    PathMtuAgingTimeout(u32),
    // Path MTU Plateau Table Option
    //
    // This option specifies a table of MTU sizes to use when performing
    // Path MTU Discovery as defined in RFC 1191. The table is formatted as
    // a list of 16-bit unsigned integers, ordered from smallest to largest.
    // The minimum MTU value cannot be smaller than 68.
    //
    // The code for this option is 25. Its minimum length is 2, and the
    // length MUST be a multiple of 2.
    //
    //  Code   Len     Size 1      Size 2
    // +-----+-----+-----+-----+-----+-----+---
    // |  25 |  n  |  s1 |  s2 |  s1 |  s2 | ...
    // +-----+-----+-----+-----+-----+-----+---
    PathMtuPlateauTable(Vec<u16>),
    // Interface MTU Option
    //
    // This option specifies the MTU to use on this interface.  The MTU is
    // specified as a 16-bit unsigned integer.  The minimum legal value for
    // the MTU is 68.
    //
    // The code for this option is 26, and its length is 2.
    //
    //  Code   Len      MTU
    // +-----+-----+-----+-----+
    // |  26 |  2  |  m1 |  m2 |
    // +-----+-----+-----+-----+
    InterfaceMtu(u16),
    // All Subnets are Local Option
    //
    // This option specifies whether or not the client may assume that all
    // subnets of the IP network to which the client is connected use the
    // same MTU as the subnet of that network to which the client is
    // directly connected. A value of 1 indicates that all subnets share
    // the same MTU. A value of 0 means that the client should assume that
    // some subnets of the directly connected network may have smaller MTUs.
    //
    // The code for this option is 27, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  27 |  1  | 0/1 |
    // +-----+-----+-----+
    AllSubnetsAreLocal(bool),
    // Broadcast Address Option
    //
    // This option specifies the broadcast address in use on the client's
    // subnet.
    //
    // The code for this option is 28, and its length is 4.
    //
    //  Code   Len     Broadcast Address
    // +-----+-----+-----+-----+-----+-----+
    // |  28 |  4  |  b1 |  b2 |  b3 |  b4 |
    // +-----+-----+-----+-----+-----+-----+
    BroadcastAddress(Ipv4Addr),
    // Perform Mask Discovery Option
    //
    // This option specifies whether or not the client should perform subnet
    // mask discovery using ICMP. A value of 0 indicates that the client
    // should not perform mask discovery. A value of 1 means that the
    // client should perform mask discovery.
    //
    // The code for this option is 29, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  29 |  1  | 0/1 |
    // +-----+-----+-----+
    PerformMaskDiscovery(bool),
    // Mask Supplier Option
    //
    // This option specifies whether or not the client should respond to
    // subnet mask requests using ICMP.  A value of 0 indicates that the
    // client should not respond.  A value of 1 means that the client should
    // respond.
    //
    // The code for this option is 30, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  30 |  1  | 0/1 |
    // +-----+-----+-----+
    MaskSupplier(bool),
    // Perform Router Discovery Option
    //
    // This option specifies whether or not the client should solicit
    // routers using the Router Discovery mechanism defined in RFC 1256.
    // A value of 0 indicates that the client should not perform
    // router discovery. A value of 1 means that the client should perform
    // router discovery.
    //
    // The code for this option is 31, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  31 |  1  | 0/1 |
    // +-----+-----+-----+
    PerformRouterDiscovery(bool),
    // Router Solicitation Address Option
    //
    // This option specifies the address to which the client should transmit
    // router solicitation requests.
    //
    // The code for this option is 32, and its length is 4.
    //
    //  Code   Len            Address
    // +-----+-----+-----+-----+-----+-----+
    // |  32 |  4  |  a1 |  a2 |  a3 |  a4 |
    // +-----+-----+-----+-----+-----+-----+
    RouterSolicitationAddress(Ipv4Addr),
    // Static Route Option
    //
    // This option specifies a list of static routes that the client should
    // install in its routing cache. If multiple routes to the same
    // destination are specified, they are listed in descending order of
    // priority.
    //
    // The routes consist of a list of IP address pairs. The first address
    // is the destination address, and the second address is the router for
    // the destination.
    //
    // The default route (0.0.0.0) is an illegal destination for a static
    // route.
    //
    // The code for this option is 33.  The minimum length of this option is
    // 8, and the length MUST be a multiple of 8.
    //
    //  Code   Len         Destination 1           Router 1
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
    // |  33 |  n  |  d1 |  d2 |  d3 |  d4 |  r1 |  r2 |  r3 |  r4 |
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
    //         Destination 2           Router 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    // |  d1 |  d2 |  d3 |  d4 |  r1 |  r2 |  r3 |  r4 | ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    StaticRoute(Vec<(Ipv4Addr, Ipv4Addr)>),
    // Trailer Encapsulation Option
    //
    // This option specifies whether or not the client should negotiate the
    // use of trailers (RFC 893 [14]) when using the ARP protocol.  A value
    // of 0 indicates that the client should not attempt to use trailers.  A
    // value of 1 means that the client should attempt to use trailers.
    //
    // The code for this option is 34, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  34 |  1  | 0/1 |
    // +-----+-----+-----+
    TrailerEncapsulation(bool),
    // ARP Cache Timeout Option
    //
    // This option specifies the timeout in seconds for ARP cache entries.
    // The time is specified as a 32-bit unsigned integer.
    //
    // The code for this option is 35, and its length is 4.
    //
    //  Code   Len           Time
    // +-----+-----+-----+-----+-----+-----+
    // |  35 |  4  |  t1 |  t2 |  t3 |  t4 |
    // +-----+-----+-----+-----+-----+-----+
    ArpCacheTimeout(u32),
    // Ethernet Encapsulation Option
    //
    // This option specifies whether or not the client should use Ethernet
    // Version 2 (RFC 894) or IEEE 802.3 (RFC 1042) encapsulation
    // if the interface is an Ethernet. A value of 0 indicates that the
    // client should use RFC 894 encapsulation. A value of 1 means that the
    // client should use RFC 1042 encapsulation.
    //
    // The code for this option is 36, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  36 |  1  | 0/1 |
    // +-----+-----+-----+
    EthernetEncapsulation(bool),
    // TCP Default TTL Option
    //
    // This option specifies the default TTL that the client should use when
    // sending TCP segments. The value is represented as an 8-bit unsigned
    // integer. The minimum value is 1.
    //
    // The code for this option is 37, and its length is 1.
    //
    //  Code   Len   TTL
    // +-----+-----+-----+
    // |  37 |  1  |  n  |
    // +-----+-----+-----+
    TcpDefaultTtl(u8),
    // TCP Keepalive Interval Option
    //
    // This option specifies the interval (in seconds) that the client TCP
    // should wait before sending a keepalive message on a TCP connection.
    // The time is specified as a 32-bit unsigned integer. A value of zero
    // indicates that the client should not generate keepalive messages on
    // connections unless specifically requested by an application.
    //
    // The code for this option is 38, and its length is 4.
    //
    //  Code   Len           Time
    // +-----+-----+-----+-----+-----+-----+
    // |  38 |  4  |  t1 |  t2 |  t3 |  t4 |
    // +-----+-----+-----+-----+-----+-----+
    TcpKeepaliveInterval(u32),
    // TCP Keepalive Garbage Option
    //
    // This option specifies the whether or not the client should send TCP
    // keepalive messages with a octet of garbage for compatibility with
    // older implementations. A value of 0 indicates that a garbage octet
    // should not be sent. A value of 1 indicates that a garbage octet
    // should be sent.
    //
    // The code for this option is 39, and its length is 1.
    //
    //  Code   Len  Value
    // +-----+-----+-----+
    // |  39 |  1  | 0/1 |
    // +-----+-----+-----+
    TcpKeepaliveGarbage(bool),
    // Network Information Service Domain Option
    //
    // This option specifies the name of the client's NIS domain. The
    // domain is formatted as a character string consisting of characters
    // from the NVT ASCII character set.
    //
    // The code for this option is 40. Its minimum length is 1.
    //
    //     Code   Len      NIS Domain Name
    // +-----+-----+-----+-----+-----+-----+---
    // |  40 |  n  |  n1 |  n2 |  n3 |  n4 | ...
    // +-----+-----+-----+-----+-----+-----+---
    NetworkInformationServiceDomain(String),
    // Network Information Servers Option
    //
    // This option specifies a list of IP addresses indicating NIS servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    // The code for this option is 41. Its minimum length is 4, and the
    // length MUST be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  41 |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    NetworkInformationServers(Vec<Ipv4Addr>),
    // Network Time Protocol Servers Option
    //
    // This option specifies a list of IP addresses indicating NTP
    // servers available to the client. Servers SHOULD be listed in order
    // of preference.
    //
    // The code for this option is 42. Its minimum length is 4, and the
    // length MUST be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  42 |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    NetworkTimeProtocolServers(Vec<Ipv4Addr>),
    // Vendor Specific Information
    //
    // This option is used by clients and servers to exchange vendor-
    // specific information. The information is an opaque object of n
    // octets, presumably interpreted by vendor-specific code on the clients
    // and servers. The definition of this information is vendor specific.
    // The vendor is indicated in the vendor class identifier option.
    // Servers not equipped to interpret the vendor-specific information
    // sent by a client MUST ignore it (although it may be reported).
    // Clients which do not receive desired vendor-specific information
    // SHOULD make an attempt to operate without it, although they may do so
    // (and announce they are doing so) in a degraded mode.
    //
    // If a vendor potentially encodes more than one item of information in
    // this option, then the vendor SHOULD encode the option using
    // "Encapsulated vendor-specific options" as described below:
    //
    // The Encapsulated vendor-specific options field SHOULD be encoded as a
    // sequence of code/length/value fields of identical syntax to the DHCP
    // options field with the following exceptions:
    //
    //     1) There SHOULD NOT be a "magic cookie" field in the encapsulated
    //         vendor-specific extensions field.
    //
    //     2) Codes other than 0 or 255 MAY be redefined by the vendor within
    //         the encapsulated vendor-specific extensions field, but SHOULD
    //         conform to the tag-length-value syntax defined in section 2.
    //
    //     3) Code 255 (END), if present, signifies the end of the
    //         encapsulated vendor extensions, not the end of the vendor
    //         extensions field. If no code 255 is present, then the end of
    //         the enclosing vendor-specific information field is taken as the
    //         end of the encapsulated vendor-specific extensions field.
    //
    // The code for this option is 43 and its minimum length is 1.
    //
    //  Code   Len   Vendor-specific information
    // +-----+-----+-----+-----+---
    // |  43 |  n  |  i1 |  i2 | ...
    // +-----+-----+-----+-----+---
    //
    // When encapsulated vendor-specific extensions are used, the
    // information bytes 1-n have the following format:
    //
    //  Code   Len   Data item        Code   Len   Data item       Code
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
    // |  T1 |  n  |  d1 |  d2 | ... |  T2 |  n  |  D1 |  D2 | ... | ... |
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
    VendorSpecificInformation(Vec<u8>),
    // NetBIOS over TCP/IP Name Server Option
    //
    // The NetBIOS name server (NBNS) option specifies a list of RFC
    // 1001/1002 NBNS name servers listed in order of preference.
    //
    // The code for this option is 44. The minimum length of the option is
    // 4 octets, and the length must always be a multiple of 4.
    //
    //  Code   Len           Address 1              Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+----
    // |  44 |  n  |  a1 |  a2 |  a3 |  a4 |  b1 |  b2 |  b3 |  b4 | ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+----
    NetBiosOverTcpIpNameServer(Vec<Ipv4Addr>),
    // NetBIOS over TCP/IP Datagram Distribution Server Option
    //
    // The NetBIOS datagram distribution server (NBDD) option specifies a
    // list of RFC 1001/1002 NBDD servers listed in order of preference. The
    // code for this option is 45. The minimum length of the option is 4
    // octets, and the length must always be a multiple of 4.
    //
    //  Code   Len           Address 1              Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+----
    // |  45 |  n  |  a1 |  a2 |  a3 |  a4 |  b1 |  b2 |  b3 |  b4 | ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+----
    NetBiosOverTcpIpDatagramDistributionServer(Vec<Ipv4Addr>),
    // NetBIOS over TCP/IP Node Type Option
    //
    // The NetBIOS node type option allows NetBIOS over TCP/IP clients which
    // are configurable to be configured as described in RFC 1001/1002. The
    // value is specified as a single octet which identifies the client type
    // as follows:
    //
    //     Value         Node Type
    //     -----         ---------
    //     0x1           B-node
    //     0x2           P-node
    //     0x4           M-node
    //     0x8           H-node
    //
    // In the above chart, the notation '0x' indicates a number in base-16
    // (hexadecimal).
    //
    // The code for this option is 46. The length of this option is always
    // 1.
    //
    //  Code   Len  Node Type
    // +-----+-----+-----------+
    // |  46 |  1  | see above |
    // +-----+-----+-----------+
    NetBiosOverTcpIpNodeType(NetBiosOverTcpIpNodeType),
    // NetBIOS over TCP/IP Scope Option
    //
    // The NetBIOS scope option specifies the NetBIOS over TCP/IP scope
    // parameter for the client as specified in RFC 1001/1002.
    //
    // The code for this option is 47. The minimum length of this option is
    // 1.
    //
    //  Code   Len       NetBIOS Scope
    // +-----+-----+-----+-----+-----+-----+----
    // |  47 |  n  |  s1 |  s2 |  s3 |  s4 | ...
    // +-----+-----+-----+-----+-----+-----+----
    NetBiosOverTcpIpScope(Vec<u8>),
    // X Window System Font Server Option
    //
    // This option specifies a list of X Window System Font servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    // The code for this option is 48. The minimum length of this option is
    // 4 octets, and the length MUST be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    // |  48 |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |   ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    XWindowSystemFontServer(Vec<Ipv4Addr>),
    // X Window System Display Manager Option
    //
    // This option specifies a list of IP addresses of systems that are
    // running the X Window System Display Manager and are available to the
    // client.
    //
    // Addresses SHOULD be listed in order of preference.
    //
    // The code for the this option is 49. The minimum length of this option
    // is 4, and the length MUST be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    // |  49 |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |   ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+---
    XWindowSystemDisplayManager(Vec<Ipv4Addr>),
    // Network Information Service+ Domain Option
    //
    // This option specifies the name of the client's NIS+ domain. The
    // domain is formatted as a character string consisting of characters
    // from the NVT ASCII character set.
    //
    // The code for this option is 64. Its minimum length is 1.
    //
    //  Code   Len      NIS Client Domain Name
    // +-----+-----+-----+-----+-----+-----+---
    // |  64 |  n  |  n1 |  n2 |  n3 |  n4 | ...
    // +-----+-----+-----+-----+-----+-----+---
    NetworkInformationServicePlusDomain(String),
    // Network Information Service+ Servers Option
    //
    // This option specifies a list of IP addresses indicating NIS+ servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    // The code for this option is 65. Its minimum length is 4, and the
    // length MUST be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // |  65 |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    NetworkInformationServicePlusServers(Vec<Ipv4Addr>),
    // Mobile IP Home Agent option
    //
    // This option specifies a list of IP addresses indicating mobile IP
    // home agents available to the client. Agents SHOULD be listed in
    // order of preference.
    //
    // The code for this option is 68. Its minimum length is 0 (indicating
    // no home agents are available) and the length MUST be a multiple of 4.
    // It is expected that the usual length will be four octets, containing
    // a single home agent's address.
    //
    //  Code Len    Home Agent Addresses (zero or more)
    // +-----+-----+-----+-----+-----+-----+--
    // | 68  |  n  | a1  | a2  | a3  | a4  | ...
    // +-----+-----+-----+-----+-----+-----+--
    MobileIpHomeAgent(Vec<Ipv4Addr>),
    // Simple Mail Transport Protocol (SMTP) Server Option
    //
    // The SMTP server option specifies a list of SMTP servers available to
    // the client. Servers SHOULD be listed in order of preference.
    //
    // The code for the SMTP server option is 69. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 69  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    SimpleMailTransportProtocolServer(Vec<Ipv4Addr>),
    // Post Office Protocol (POP3) Server Option
    //
    // The POP3 server option specifies a list of POP3 available to the
    // client. Servers SHOULD be listed in order of preference.
    //
    // The code for the POP3 server option is 70. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 70  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    PostOfficeProtocolServer(Vec<Ipv4Addr>),
    // Network News Transport Protocol (NNTP) Server Option
    //
    // The NNTP server option specifies a list of NNTP available to the
    // client. Servers SHOULD be listed in order of preference.
    //
    // The code for the NNTP server option is 71. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 71  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    NetworkNewsTransportProtocolServer(Vec<Ipv4Addr>),
    // Default World Wide Web (WWW) Server Option
    //
    // The WWW server option specifies a list of WWW available to the
    // client. Servers SHOULD be listed in order of preference.
    //
    // The code for the WWW server option is 72. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 72  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    DefaultWorldWideWebServer(Vec<Ipv4Addr>),
    // Default Finger Server Option
    //
    // The Finger server option specifies a list of Finger available to the
    // client. Servers SHOULD be listed in order of preference.
    //
    // The code for the Finger server option is 73. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 73  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    DefaultFingerServer(Vec<Ipv4Addr>),
    // Default Internet Relay Chat (IRC) Server Option
    //
    // The IRC server option specifies a list of IRC available to the
    // client. Servers SHOULD be listed in order of preference.
    //
    // The code for the IRC server option is 74. The minimum length for
    // this option is 4 octets, and the length MUST always be a multiple of
    // 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 74  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    DefaultInternetRelayChatServer(Vec<Ipv4Addr>),
    // StreetTalk Server Option
    //
    // The StreetTalk server option specifies a list of StreetTalk servers
    // available to the client. Servers SHOULD be listed in order of
    // preference.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 75  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    StreetTalkServer(Vec<Ipv4Addr>),
    // StreetTalk Directory Assistance (STDA) Server Option

    // The StreetTalk Directory Assistance (STDA) server option specifies a
    // list of STDA servers available to the client. Servers SHOULD be
    // listed in order of preference.
    //
    // The code for the StreetTalk Directory Assistance server option is 76.
    // The minimum length for this option is 4 octets, and the length MUST
    // always be a multiple of 4.
    //
    //  Code   Len         Address 1               Address 2
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    // | 76  |  n  |  a1 |  a2 |  a3 |  a4 |  a1 |  a2 |  ...
    // +-----+-----+-----+-----+-----+-----+-----+-----+--
    StreetTalkDirectoryAssistanceServer(Vec<Ipv4Addr>),
    // Requested IP Address
    //
    // This option is used in a client request (DHCPDISCOVER) to allow the
    // client to request that a particular IP address be assigned.
    //
    // The code for this option is 50, and its length is 4.
    //
    //  Code   Len          Address
    // +-----+-----+-----+-----+-----+-----+
    // |  50 |  4  |  a1 |  a2 |  a3 |  a4 |
    // +-----+-----+-----+-----+-----+-----+
    RequestedIpAddress(Ipv4Addr),
    // IP Address Lease Time
    //
    // This option is used in a client request (DHCPDISCOVER or DHCPREQUEST)
    // to allow the client to request a lease time for the IP address. In a
    // server reply (DHCPOFFER), a DHCP server uses this option to specify
    // the lease time it is willing to offer.
    //
    // The time is in units of seconds, and is specified as a 32-bit
    // unsigned integer.
    //
    // The code for this option is 51, and its length is 4.
    //
    //  Code   Len         Lease Time
    // +-----+-----+-----+-----+-----+-----+
    // |  51 |  4  |  t1 |  t2 |  t3 |  t4 |
    // +-----+-----+-----+-----+-----+-----+
    IpAddressLeaseTime(u32),
}

impl DhcpOption {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            DhcpOption::Pad => vec![0],
            DhcpOption::End => vec![255],
            DhcpOption::SubnetMask(subnet_mask) => {
                let mut result = Vec::new();
                result.push(1);
                result.push(4);
                result.extend_from_slice(&subnet_mask.octets());
                result
            }
            DhcpOption::TimeOffset(time_offset) => {
                let mut result = Vec::new();
                result.push(2);
                result.push(4);
                result.push(((time_offset >> 24) & 0xFF) as u8);
                result.push(((time_offset >> 16) & 0xFF) as u8);
                result.push(((time_offset >> 8) & 0xFF) as u8);
                result.push((time_offset & 0xFF) as u8);
                result
            }
            DhcpOption::Router(routers) => {
                let mut result = Vec::new();
                result.push(3);
                result.push((routers.len() * 4) as u8);
                for router in routers {
                    result.extend_from_slice(&router.octets());
                }
                result
            }
            DhcpOption::TimeServer(time_servers) => {
                let mut result = Vec::new();
                result.push(4);
                result.push((time_servers.len() * 4) as u8);
                for time_server in time_servers {
                    result.extend_from_slice(&time_server.octets());
                }
                result
            }
            DhcpOption::NameServer(name_servers) => {
                let mut result = Vec::new();
                result.push(5);
                result.push((name_servers.len() * 4) as u8);
                for name_server in name_servers {
                    result.extend_from_slice(&name_server.octets());
                }
                result
            }
            DhcpOption::DomainNameServer(domain_name_servers) => {
                let mut result = Vec::new();
                result.push(6);
                result.push((domain_name_servers.len() * 4) as u8);
                for domain_name_server in domain_name_servers {
                    result.extend_from_slice(&domain_name_server.octets());
                }
                result
            }
            DhcpOption::LogServer(log_servers) => {
                let mut result = Vec::new();
                result.push(7);
                result.push((log_servers.len() * 4) as u8);
                for log_server in log_servers {
                    result.extend_from_slice(&log_server.octets());
                }
                result
            }
            DhcpOption::CookieServer(cookie_servers) => {
                let mut result = Vec::new();
                result.push(8);
                result.push((cookie_servers.len() * 4) as u8);
                for cookie_server in cookie_servers {
                    result.extend_from_slice(&cookie_server.octets());
                }
                result
            }
            DhcpOption::LprServer(lpr_servers) => {
                let mut result = Vec::new();
                result.push(9);
                result.push((lpr_servers.len() * 4) as u8);
                for lpr_server in lpr_servers {
                    result.extend_from_slice(&lpr_server.octets());
                }
                result
            }
            DhcpOption::ImpressServer(impress_servers) => {
                let mut result = Vec::new();
                result.push(10);
                result.push((impress_servers.len() * 4) as u8);
                for impress_server in impress_servers {
                    result.extend_from_slice(&impress_server.octets());
                }
                result
            }
            DhcpOption::ResourceLocationServer(resource_location_servers) => {
                let mut result = Vec::new();
                result.push(11);
                result.push((resource_location_servers.len() * 4) as u8);
                for resource_location_server in resource_location_servers {
                    result.extend_from_slice(&resource_location_server.octets());
                }
                result
            }
            DhcpOption::HostName(host_name) => {
                let mut result = Vec::new();
                result.push(12);
                result.push(host_name.len() as u8);
                result.extend_from_slice(host_name.as_bytes());
                result
            }
            DhcpOption::BootFileSize(boot_file_size) => {
                let mut result = Vec::new();
                result.push(13);
                result.push(2);
                result.push(((boot_file_size >> 8) & 0xFF) as u8);
                result.push((boot_file_size & 0xFF) as u8);
                result
            }
            DhcpOption::MeritDumpFile(merit_dump_file) => {
                let mut result = Vec::new();
                result.push(14);
                result.push(merit_dump_file.len() as u8);
                result.extend_from_slice(merit_dump_file.as_bytes());
                result
            }
            DhcpOption::DomainName(domain_name) => {
                let mut result = Vec::new();
                result.push(15);
                result.push(domain_name.len() as u8);
                result.extend_from_slice(domain_name.as_bytes());
                result
            }
            DhcpOption::SwapServer(swap_server) => {
                let mut result = Vec::new();
                result.push(16);
                result.push(4);
                result.extend_from_slice(&swap_server.octets());
                result
            }
            DhcpOption::RootPath(root_path) => {
                let mut result = Vec::new();
                result.push(17);
                result.push(root_path.len() as u8);
                result.extend_from_slice(root_path.as_bytes());
                result
            }
            DhcpOption::ExtensionsPath(extensions_path) => {
                let mut result = Vec::new();
                result.push(18);
                result.push(extensions_path.len() as u8);
                result.extend_from_slice(extensions_path.as_bytes());
                result
            }
            DhcpOption::IpForwarding(ip_forwarding) => {
                let mut result = Vec::new();
                result.push(19);
                result.push(1);
                result.push(if *ip_forwarding { 1 } else { 0 });
                result
            }
            DhcpOption::NonLocalSourceRouting(non_local_source_routing) => {
                let mut result = Vec::new();
                result.push(20);
                result.push(1);
                result.push(if *non_local_source_routing { 1 } else { 0 });
                result
            }
            DhcpOption::PolicyFilter(policy_filter) => {
                let mut result = Vec::new();
                result.push(21);
                result.push((policy_filter.len() * 8) as u8);
                for policy_filter in policy_filter {
                    result.push(policy_filter.0.octets()[0]);
                    result.push(policy_filter.0.octets()[1]);
                    result.push(policy_filter.0.octets()[2]);
                    result.push(policy_filter.0.octets()[3]);
                    result.push(policy_filter.1.octets()[0]);
                    result.push(policy_filter.1.octets()[1]);
                    result.push(policy_filter.1.octets()[2]);
                    result.push(policy_filter.1.octets()[3]);
                }
                result
            }
            DhcpOption::MaximumDatagramReassemblySize(maximum_datagram_reassembly_size) => {
                let mut result = Vec::new();
                result.push(22);
                result.push(2);
                result.push(((maximum_datagram_reassembly_size >> 8) & 0xFF) as u8);
                result.push((maximum_datagram_reassembly_size & 0xFF) as u8);
                result
            }
            DhcpOption::DefaultIpTimeToLive(default_ip_ttl) => {
                let mut result = Vec::new();
                result.push(23);
                result.push(1);
                result.push(*default_ip_ttl);
                result
            }
            DhcpOption::PathMtuAgingTimeout(path_mtu_aging_timeout) => {
                let mut result = Vec::new();
                result.push(24);
                result.push(4);
                result.push(((path_mtu_aging_timeout >> 24) & 0xFF) as u8);
                result.push(((path_mtu_aging_timeout >> 16) & 0xFF) as u8);
                result.push(((path_mtu_aging_timeout >> 8) & 0xFF) as u8);
                result.push((path_mtu_aging_timeout & 0xFF) as u8);
                result
            }
            DhcpOption::PathMtuPlateauTable(path_mtu_plateau_table) => {
                let mut result = Vec::new();
                result.push(25);
                result.push((path_mtu_plateau_table.len() * 2) as u8);
                for path_mtu_plateau in path_mtu_plateau_table {
                    result.push(((path_mtu_plateau >> 8) & 0xFF) as u8);
                    result.push((path_mtu_plateau & 0xFF) as u8);
                }
                result
            }
            DhcpOption::InterfaceMtu(interface_mtu) => {
                let mut result = Vec::new();
                result.push(26);
                result.push(2);
                result.push(((interface_mtu >> 8) & 0xFF) as u8);
                result.push((interface_mtu & 0xFF) as u8);
                result
            }
            DhcpOption::AllSubnetsAreLocal(all_subnets_are_local) => {
                let mut result = Vec::new();
                result.push(27);
                result.push(1);
                result.push(if *all_subnets_are_local { 1 } else { 0 });
                result
            }
            DhcpOption::BroadcastAddress(broadcast_address) => {
                let mut result = Vec::new();
                result.push(28);
                result.push(4);
                result.extend_from_slice(&broadcast_address.octets());
                result
            }
            DhcpOption::PerformMaskDiscovery(perform_mask_discovery) => {
                let mut result = Vec::new();
                result.push(29);
                result.push(1);
                result.push(if *perform_mask_discovery { 1 } else { 0 });
                result
            }
            DhcpOption::MaskSupplier(mask_supplier) => {
                let mut result = Vec::new();
                result.push(30);
                result.push(1);
                result.push(if *mask_supplier { 1 } else { 0 });
                result
            }
            DhcpOption::PerformRouterDiscovery(perform_router_discovery) => {
                let mut result = Vec::new();
                result.push(31);
                result.push(1);
                result.push(if *perform_router_discovery { 1 } else { 0 });
                result
            }
            DhcpOption::RouterSolicitationAddress(router_solicitation_address) => {
                let mut result = Vec::new();
                result.push(32);
                result.push(4);
                result.extend_from_slice(&router_solicitation_address.octets());
                result
            }
            DhcpOption::StaticRoute(static_route) => {
                let mut result = Vec::new();
                result.push(33);
                result.push((static_route.len() * 8) as u8);
                for static_route in static_route {
                    result.push(static_route.0.octets()[0]);
                    result.push(static_route.0.octets()[1]);
                    result.push(static_route.0.octets()[2]);
                    result.push(static_route.0.octets()[3]);
                    result.push(static_route.1.octets()[0]);
                    result.push(static_route.1.octets()[1]);
                    result.push(static_route.1.octets()[2]);
                    result.push(static_route.1.octets()[3]);
                }
                result
            }
            DhcpOption::TrailerEncapsulation(trailer_encapsulation) => {
                let mut result = Vec::new();
                result.push(34);
                result.push(1);
                result.push(if *trailer_encapsulation { 1 } else { 0 });
                result
            }
            DhcpOption::ArpCacheTimeout(arp_cache_timeout) => {
                let mut result = Vec::new();
                result.push(35);
                result.push(4);
                result.push(((arp_cache_timeout >> 24) & 0xFF) as u8);
                result.push(((arp_cache_timeout >> 16) & 0xFF) as u8);
                result.push(((arp_cache_timeout >> 8) & 0xFF) as u8);
                result.push((arp_cache_timeout & 0xFF) as u8);
                result
            }
            DhcpOption::EthernetEncapsulation(ethernet_encapsulation) => {
                let mut result = Vec::new();
                result.push(36);
                result.push(1);
                result.push(if *ethernet_encapsulation { 1 } else { 0 });
                result
            }
            DhcpOption::TcpDefaultTtl(tcp_default_ttl) => {
                let mut result = Vec::new();
                result.push(37);
                result.push(1);
                result.push(*tcp_default_ttl);
                result
            }
            DhcpOption::TcpKeepaliveInterval(tcp_keepalive_interval) => {
                let mut result = Vec::new();
                result.push(38);
                result.push(4);
                result.push(((tcp_keepalive_interval >> 24) & 0xFF) as u8);
                result.push(((tcp_keepalive_interval >> 16) & 0xFF) as u8);
                result.push(((tcp_keepalive_interval >> 8) & 0xFF) as u8);
                result.push((tcp_keepalive_interval & 0xFF) as u8);
                result
            }
            DhcpOption::TcpKeepaliveGarbage(tcp_keepalive_garbage) => {
                let mut result = Vec::new();
                result.push(39);
                result.push(1);
                result.push(if *tcp_keepalive_garbage { 1 } else { 0 });
                result
            }
            DhcpOption::NetworkInformationServiceDomain(network_information_service_domain) => {
                let mut result = Vec::new();
                result.push(40);
                result.push(network_information_service_domain.len() as u8);
                result.extend_from_slice(network_information_service_domain.as_bytes());
                result
            }
            DhcpOption::NetworkInformationServers(network_information_servers) => {
                let mut result = Vec::new();
                result.push(41);
                result.push((network_information_servers.len() * 4) as u8);
                for network_information_server in network_information_servers {
                    result.push(network_information_server.octets()[0]);
                    result.push(network_information_server.octets()[1]);
                    result.push(network_information_server.octets()[2]);
                    result.push(network_information_server.octets()[3]);
                }
                result
            }
            DhcpOption::NetworkTimeProtocolServers(network_time_protocol_servers) => {
                let mut result = Vec::new();
                result.push(42);
                result.push((network_time_protocol_servers.len() * 4) as u8);
                for network_time_protocol_server in network_time_protocol_servers {
                    result.push(network_time_protocol_server.octets()[0]);
                    result.push(network_time_protocol_server.octets()[1]);
                    result.push(network_time_protocol_server.octets()[2]);
                    result.push(network_time_protocol_server.octets()[3]);
                }
                result
            }
            DhcpOption::VendorSpecificInformation(vendor_specific_information) => {
                let mut result = Vec::new();
                result.push(43);
                result.push(vendor_specific_information.len() as u8);
                result.extend_from_slice(vendor_specific_information);
                result
            }
            DhcpOption::NetBiosOverTcpIpNameServer(netbios_over_tcpip_name_server) => {
                let mut result = Vec::new();
                result.push(44);
                result.push((netbios_over_tcpip_name_server.len() * 4) as u8);
                for netbios_over_tcpip_name_server in netbios_over_tcpip_name_server {
                    result.push(netbios_over_tcpip_name_server.octets()[0]);
                    result.push(netbios_over_tcpip_name_server.octets()[1]);
                    result.push(netbios_over_tcpip_name_server.octets()[2]);
                    result.push(netbios_over_tcpip_name_server.octets()[3]);
                }
                result
            }
            DhcpOption::NetBiosOverTcpIpDatagramDistributionServer(
                netbios_over_tcpip_datagram_distribution_server,
            ) => {
                let mut result = Vec::new();
                result.push(45);
                result.push((netbios_over_tcpip_datagram_distribution_server.len() * 4) as u8);
                for netbios_over_tcpip_datagram_distribution_server in
                    netbios_over_tcpip_datagram_distribution_server
                {
                    result.push(netbios_over_tcpip_datagram_distribution_server.octets()[0]);
                    result.push(netbios_over_tcpip_datagram_distribution_server.octets()[1]);
                    result.push(netbios_over_tcpip_datagram_distribution_server.octets()[2]);
                    result.push(netbios_over_tcpip_datagram_distribution_server.octets()[3]);
                }
                result
            }
            DhcpOption::NetBiosOverTcpIpNodeType(netbios_over_tcpip_node_type) => {
                let mut result = Vec::new();
                result.push(46);
                result.push(1);
                match netbios_over_tcpip_node_type {
                    NetBiosOverTcpIpNodeType::BNode => result.push(1),
                    NetBiosOverTcpIpNodeType::PNode => result.push(2),
                    NetBiosOverTcpIpNodeType::MNode => result.push(4),
                    NetBiosOverTcpIpNodeType::HNode => result.push(8),
                }
                result
            }
            DhcpOption::NetBiosOverTcpIpScope(netbios_over_tcpip_scope) => {
                let mut result = Vec::new();
                result.push(47);
                result.push(netbios_over_tcpip_scope.len() as u8);
                result.extend_from_slice(&netbios_over_tcpip_scope);
                result
            }
            DhcpOption::XWindowSystemFontServer(x_window_system_font_server) => {
                let mut result = Vec::new();
                result.push(48);
                result.push((x_window_system_font_server.len() * 4) as u8);
                for x_window_system_font_server in x_window_system_font_server {
                    result.push(x_window_system_font_server.octets()[0]);
                    result.push(x_window_system_font_server.octets()[1]);
                    result.push(x_window_system_font_server.octets()[2]);
                    result.push(x_window_system_font_server.octets()[3]);
                }
                result
            }
            DhcpOption::XWindowSystemDisplayManager(x_window_system_display_manager) => {
                let mut result = Vec::new();
                result.push(49);
                result.push((x_window_system_display_manager.len() * 4) as u8);
                for x_window_system_display_manager in x_window_system_display_manager {
                    result.push(x_window_system_display_manager.octets()[0]);
                    result.push(x_window_system_display_manager.octets()[1]);
                    result.push(x_window_system_display_manager.octets()[2]);
                    result.push(x_window_system_display_manager.octets()[3]);
                }
                result
            }
            DhcpOption::NetworkInformationServicePlusDomain(
                network_information_service_plus_domain,
            ) => {
                let mut result = Vec::new();
                result.push(64);
                result.push(network_information_service_plus_domain.len() as u8);
                result.extend_from_slice(network_information_service_plus_domain.as_bytes());
                result
            }
            DhcpOption::NetworkInformationServicePlusServers(
                network_information_service_plus_servers,
            ) => {
                let mut result = Vec::new();
                result.push(65);
                result.push((network_information_service_plus_servers.len() * 4) as u8);
                for network_information_service_plus_server in
                    network_information_service_plus_servers
                {
                    result.push(network_information_service_plus_server.octets()[0]);
                    result.push(network_information_service_plus_server.octets()[1]);
                    result.push(network_information_service_plus_server.octets()[2]);
                    result.push(network_information_service_plus_server.octets()[3]);
                }
                result
            }
            DhcpOption::MobileIpHomeAgent(mobile_ip_home_agent) => {
                let mut result = Vec::new();
                result.push(68);
                result.push((mobile_ip_home_agent.len() * 4) as u8);
                for mobile_ip_home_agent in mobile_ip_home_agent {
                    result.push(mobile_ip_home_agent.octets()[0]);
                    result.push(mobile_ip_home_agent.octets()[1]);
                    result.push(mobile_ip_home_agent.octets()[2]);
                    result.push(mobile_ip_home_agent.octets()[3]);
                }
                result
            }
            DhcpOption::SimpleMailTransportProtocolServer(
                simple_mail_transport_protocol_server,
            ) => {
                let mut result = Vec::new();
                result.push(69);
                result.push((simple_mail_transport_protocol_server.len() * 4) as u8);
                for simple_mail_transport_protocol_server in simple_mail_transport_protocol_server {
                    result.push(simple_mail_transport_protocol_server.octets()[0]);
                    result.push(simple_mail_transport_protocol_server.octets()[1]);
                    result.push(simple_mail_transport_protocol_server.octets()[2]);
                    result.push(simple_mail_transport_protocol_server.octets()[3]);
                }
                result
            }
            DhcpOption::PostOfficeProtocolServer(post_office_protocol_server) => {
                let mut result = Vec::new();
                result.push(70);
                result.push((post_office_protocol_server.len() * 4) as u8);
                for post_office_protocol_server in post_office_protocol_server {
                    result.push(post_office_protocol_server.octets()[0]);
                    result.push(post_office_protocol_server.octets()[1]);
                    result.push(post_office_protocol_server.octets()[2]);
                    result.push(post_office_protocol_server.octets()[3]);
                }
                result
            }
            DhcpOption::NetworkNewsTransportProtocolServer(
                network_news_transport_protocol_server,
            ) => {
                let mut result = Vec::new();
                result.push(71);
                result.push((network_news_transport_protocol_server.len() * 4) as u8);
                for network_news_transport_protocol_server in network_news_transport_protocol_server
                {
                    result.push(network_news_transport_protocol_server.octets()[0]);
                    result.push(network_news_transport_protocol_server.octets()[1]);
                    result.push(network_news_transport_protocol_server.octets()[2]);
                    result.push(network_news_transport_protocol_server.octets()[3]);
                }
                result
            }
            DhcpOption::DefaultWorldWideWebServer(default_world_wide_web_server) => {
                let mut result = Vec::new();
                result.push(72);
                result.push((default_world_wide_web_server.len() * 4) as u8);
                for default_world_wide_web_server in default_world_wide_web_server {
                    result.push(default_world_wide_web_server.octets()[0]);
                    result.push(default_world_wide_web_server.octets()[1]);
                    result.push(default_world_wide_web_server.octets()[2]);
                    result.push(default_world_wide_web_server.octets()[3]);
                }
                result
            }
            DhcpOption::DefaultFingerServer(default_finger_server) => {
                let mut result = Vec::new();
                result.push(73);
                result.push((default_finger_server.len() * 4) as u8);
                for default_finger_server in default_finger_server {
                    result.push(default_finger_server.octets()[0]);
                    result.push(default_finger_server.octets()[1]);
                    result.push(default_finger_server.octets()[2]);
                    result.push(default_finger_server.octets()[3]);
                }
                result
            }
            DhcpOption::DefaultInternetRelayChatServer(default_internet_relay_chat_server) => {
                let mut result = Vec::new();
                result.push(74);
                result.push((default_internet_relay_chat_server.len() * 4) as u8);
                for default_internet_relay_chat_server in default_internet_relay_chat_server {
                    result.push(default_internet_relay_chat_server.octets()[0]);
                    result.push(default_internet_relay_chat_server.octets()[1]);
                    result.push(default_internet_relay_chat_server.octets()[2]);
                    result.push(default_internet_relay_chat_server.octets()[3]);
                }
                result
            }
            DhcpOption::StreetTalkServer(street_talk_server) => {
                let mut result = Vec::new();
                result.push(75);
                result.push((street_talk_server.len() * 4) as u8);
                for street_talk_server in street_talk_server {
                    result.push(street_talk_server.octets()[0]);
                    result.push(street_talk_server.octets()[1]);
                    result.push(street_talk_server.octets()[2]);
                    result.push(street_talk_server.octets()[3]);
                }
                result
            }
            DhcpOption::StreetTalkDirectoryAssistanceServer(
                street_talk_directory_assistance_server,
            ) => {
                let mut result = Vec::new();
                result.push(76);
                result.push((street_talk_directory_assistance_server.len() * 4) as u8);
                for street_talk_directory_assistance_server in
                    street_talk_directory_assistance_server
                {
                    result.push(street_talk_directory_assistance_server.octets()[0]);
                    result.push(street_talk_directory_assistance_server.octets()[1]);
                    result.push(street_talk_directory_assistance_server.octets()[2]);
                    result.push(street_talk_directory_assistance_server.octets()[3]);
                }
                result
            }
            DhcpOption::RequestedIpAddress(requested_ip_address) => {
                let mut result = Vec::new();
                result.push(50);
                result.push(4);
                result.push(requested_ip_address.octets()[0]);
                result.push(requested_ip_address.octets()[1]);
                result.push(requested_ip_address.octets()[2]);
                result.push(requested_ip_address.octets()[3]);
                result
            }
            DhcpOption::IpAddressLeaseTime(ip_address_lease_time) => {
                let mut result = Vec::new();
                result.push(51);
                result.push(4);
                result.push(((ip_address_lease_time >> 24) & 0xFF) as u8);
                result.push(((ip_address_lease_time >> 16) & 0xFF) as u8);
                result.push(((ip_address_lease_time >> 8) & 0xFF) as u8);
                result.push((ip_address_lease_time & 0xFF) as u8);
                result
            }
        }
    }

    pub fn deserialize(data: &[u8]) -> Result<(DhcpOption, &[u8]), DhcpError> {
        // Retrieve the option code.
        let (code, data) = match data.split_first() {
            Some((code, data)) => (*code, data),
            None => return Err(DhcpError::ParsingError("No option code found".to_string())),
        };

        //
        match code {
            0 => Ok((DhcpOption::Pad, data)),
            255 => Ok((DhcpOption::End, data)),
            1 => {
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse subnet mask".to_string(),
                    ));
                }

                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse subnet mask".to_string(),
                        ))
                    }
                };

                let (subnet_mask, data) = data.split_at(4);
                let subnet_mask = Ipv4Addr::new(
                    subnet_mask[0],
                    subnet_mask[1],
                    subnet_mask[2],
                    subnet_mask[3],
                );

                Ok((DhcpOption::SubnetMask(subnet_mask), data))
            }
            2 => {
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse time offset".to_string(),
                    ));
                }

                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse time offset".to_string(),
                        ))
                    }
                };

                let (time_offset, data) = data.split_at(4);
                let time_offset = ((time_offset[0] as u32) << 24)
                    + ((time_offset[1] as u32) << 16)
                    + ((time_offset[2] as u32) << 8)
                    + (time_offset[3] as u32);

                Ok((DhcpOption::TimeOffset(time_offset), data))
            }
            3 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse router".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse router".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse router".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::Router(addresses), data))
            }
            4 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse time servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse time servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse time servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::TimeServer(addresses), data))
            }
            5 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse name servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse name servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse name servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::NameServer(addresses), data))
            }
            6 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse domain name servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse domain name servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse domain name servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::DomainNameServer(addresses), data))
            }
            7 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse log servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse log servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse log servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::LogServer(addresses), data))
            }
            8 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse cookie servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse cookie servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse cookie servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::CookieServer(addresses), data))
            }
            9 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse lpr servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse lpr servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse lpr servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::LprServer(addresses), data))
            }
            10 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse impress servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse impress servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse impress servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::ImpressServer(addresses), data))
            }
            11 => {
                // Check that the data is long enough to contain the length and at least one address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse resource location servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse resource location servers".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse resource location servers".to_string(),
                    ));
                }

                // Retreive the addresses.
                let (addresses, data) = data.split_at(len as usize);
                let addresses = addresses
                    .chunks_exact(4)
                    .map(|address| Ipv4Addr::new(address[0], address[1], address[2], address[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::ResourceLocationServer(addresses), data))
            }
            12 => {
                // Check that the data is long enough to contain a name with at least 1 character.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse host name".to_string(),
                    ));
                }

                // Retrieve the length of the name.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse host name".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if len > data.len() as u8 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse host name".to_string(),
                    ));
                }

                // Retrieve the name.
                let (hostname, data) = data.split_at(len as usize);

                // Convert the name to a string.
                let hostname = match from_utf8(hostname) {
                    Ok(hostname) => hostname,
                    Err(_) => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse host name".to_string(),
                        ))
                    }
                };

                Ok((DhcpOption::HostName(hostname.to_string()), data))
            }
            13 => {
                // Check that the data is long enough to contain a short.
                if data.len() < 3 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse boot file size".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse boot file size".to_string(),
                        ))
                    }
                };

                // Retrieve the size.
                let (size, data) = match data.split_at(2) {
                    (size, data) => (u16::from_be_bytes([size[0], size[1]]), data),
                };

                Ok((DhcpOption::BootFileSize(size), data))
            }
            14 => {
                // Check that the data is long enough to contain at least a character.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse merit dump file".to_string(),
                    ));
                }

                // Retrieve the length of the name.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse merit dump file".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if len > data.len() as u8 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse merit dump file".to_string(),
                    ));
                }

                // Retrieve the name.
                let (filename, data) = data.split_at(len as usize);

                // Convert the name to a string.
                let filename = match from_utf8(filename) {
                    Ok(filename) => filename,
                    Err(_) => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse merit dump file".to_string(),
                        ))
                    }
                };

                Ok((DhcpOption::MeritDumpFile(filename.to_string()), data))
            }
            15 => {
                // Check that the data is long enough to contain at least a character.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse domain name".to_string(),
                    ));
                }

                // Retrieve the length of the name.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse domain name".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if len > data.len() as u8 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse domain name".to_string(),
                    ));
                }

                // Retrieve the name.
                let (domain, data) = data.split_at(len as usize);

                // Convert the name to a string.
                let domain = match from_utf8(domain) {
                    Ok(domain) => domain,
                    Err(_) => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse domain name".to_string(),
                        ))
                    }
                };

                Ok((DhcpOption::DomainName(domain.to_string()), data))
            }
            16 => {
                // Check that the data is long enough to contain the address.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse swap server".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse swap server".to_string(),
                        ))
                    }
                };

                // Retrieve the address.
                let (address, data) = data.split_at(4);
                let address = Ipv4Addr::new(address[0], address[1], address[2], address[3]);

                Ok((DhcpOption::SwapServer(address), data))
            }
            17 => {
                // Check that the data has at least one byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse root path".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse root path".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if len > data.len() as u8 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse root path".to_string(),
                    ));
                }

                // Retrieve the path.
                let (path, data) = data.split_at(len as usize);

                // Convert the path to a string.
                let path = match from_utf8(path) {
                    Ok(path) => path,
                    Err(_) => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse root path".to_string(),
                        ))
                    }
                };

                Ok((DhcpOption::RootPath(path.to_string()), data))
            }
            18 => {
                // Check that the data has at least one byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse extension path".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse extension path".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if len > data.len() as u8 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse extension path".to_string(),
                    ));
                }

                // Retrieve the path.
                let (path, data) = data.split_at(len as usize);

                // Convert the path to a string.
                let path = match from_utf8(path) {
                    Ok(path) => path,
                    Err(_) => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse extension path".to_string(),
                        ))
                    }
                };

                Ok((DhcpOption::ExtensionsPath(path.to_string()), data))
            }
            19 => {
                // Check that the data has at least one byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse IP forwarding".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse IP forwarding".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(1);

                Ok((DhcpOption::IpForwarding(value[0] == 1), data))
            }
            20 => {
                // Check that the data has at least one byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse non-local source routing".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse non-local source routing".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(1);

                Ok((DhcpOption::NonLocalSourceRouting(value[0] == 1), data))
            }
            21 => {
                // Check that the data cans at least hold a filter.
                if data.len() < 9 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse policy filter".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse policy filter".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if len > data.len() as u8 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse policy filter".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 8.
                if len % 8 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse policy filter".to_string(),
                    ));
                }

                // Retrieve the filters.
                let (filters, data) = data.split_at(len as usize);
                let filters = filters
                    .chunks_exact(8)
                    .map(|filter| {
                        (
                            Ipv4Addr::new(filter[0], filter[1], filter[2], filter[3]),
                            Ipv4Addr::new(filter[4], filter[5], filter[6], filter[7]),
                        )
                    })
                    .collect::<Vec<(Ipv4Addr, Ipv4Addr)>>();

                Ok((DhcpOption::PolicyFilter(filters), data))
            }
            22 => {
                // Check that the data has at least 2 bytes.
                if data.len() < 3 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse maximum datagram reassembly size".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse maximum datagram reassembly size".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(2);

                Ok((
                    DhcpOption::MaximumDatagramReassemblySize(u16::from_be_bytes([
                        value[0], value[1],
                    ])),
                    data,
                ))
            }
            23 => {
                // Check that the data has at least one byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse default IP TTL".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse default IP TTL".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(1);

                Ok((DhcpOption::DefaultIpTimeToLive(value[0]), data))
            }
            24 => {
                // Check that the data has at least 5 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse path MTU aging timeout".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse path MTU aging timeout".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(4);

                Ok((
                    DhcpOption::PathMtuAgingTimeout(u32::from_be_bytes([
                        value[0], value[1], value[2], value[3],
                    ])),
                    data,
                ))
            }
            25 => {
                // Check that the data has at least 2 bytes.
                if data.len() < 3 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse path MTU plateau table".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse path MTU plateau table".to_string(),
                        ))
                    }
                };

                let (mtu_sizes, data) = data.split_at(len as usize);
                let mtu_sizes = mtu_sizes
                    .chunks_exact(2)
                    .map(|filters| u16::from_be_bytes([filters[0], filters[1]]))
                    .collect::<Vec<u16>>();

                Ok((DhcpOption::PathMtuPlateauTable(mtu_sizes), data))
            }
            26 => {
                // Check that the data has at least 2 bytes.
                if data.len() < 3 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse interface MTU".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse interface MTU".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(2);

                Ok((
                    DhcpOption::InterfaceMtu(u16::from_be_bytes([value[0], value[1]])),
                    data,
                ))
            }
            27 => {
                // Check that the data has at least 1 byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse all subnets are local".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse all subnets are local".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(1);

                Ok((DhcpOption::AllSubnetsAreLocal(value[0] != 0), data))
            }
            28 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse broadcast address".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse broadcast address".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (address, data) = data.split_at(4);

                Ok((
                    DhcpOption::BroadcastAddress(Ipv4Addr::new(
                        address[0], address[1], address[2], address[3],
                    )),
                    data,
                ))
            }
            29 => {
                // Check that the data has at least 1 byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse perform mask discovery".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse perform mask discovery".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (address, data) = data.split_at(1);

                Ok((DhcpOption::PerformMaskDiscovery(address[0] != 0), data))
            }
            30 => {
                // Check that the data has at least 1 byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse mask supplier".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse mask supplier".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (address, data) = data.split_at(1);

                Ok((DhcpOption::MaskSupplier(address[0] != 0), data))
            }
            31 => {
                // Check that the data has at least 1byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse perform router discovery".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse perform router discovery".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (address, data) = data.split_at(1);

                Ok((DhcpOption::PerformRouterDiscovery(address[0] != 0), data))
            }
            32 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse router solicitation address".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse router solicitation address".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (address, data) = data.split_at(4);

                Ok((
                    DhcpOption::RouterSolicitationAddress(Ipv4Addr::new(
                        address[0], address[1], address[2], address[3],
                    )),
                    data,
                ))
            }
            33 => {
                // Check that the data has at least 8 bytes.
                if data.len() < 9 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse static route".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse static route".to_string(),
                        ))
                    }
                };

                // Check that the length is a multiple of 8.
                if len % 8 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse static route".to_string(),
                    ));
                }

                // Retrieve the value.
                let (routes, data) = data.split_at(len as usize);
                let routes = routes
                    .chunks_exact(8)
                    .map(|route| {
                        (
                            Ipv4Addr::new(route[0], route[1], route[2], route[3]),
                            Ipv4Addr::new(route[4], route[5], route[6], route[7]),
                        )
                    })
                    .collect::<Vec<(Ipv4Addr, Ipv4Addr)>>();

                Ok((DhcpOption::StaticRoute(routes), data))
            }
            34 => {
                // Check that the data has at least 1 bytes.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse trailer encapsulation".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse trailer encapsulation".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(1);

                Ok((DhcpOption::TrailerEncapsulation(value[0] != 0), data))
            }
            35 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse arp cache timeout".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse arp cache timeout".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (timeout, data) = data.split_at(4);

                Ok((
                    DhcpOption::ArpCacheTimeout(u32::from_be_bytes([
                        timeout[0], timeout[1], timeout[2], timeout[3],
                    ])),
                    data,
                ))
            }
            36 => {
                // Check that the data has at least 1 bytes.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse ethernet encapsulation".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse ethernet encapsulation".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (value, data) = data.split_at(1);

                Ok((DhcpOption::EthernetEncapsulation(value[0] != 0), data))
            }
            37 => {
                // Check that the data has at least 1 bytes.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse tcp default ttl".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse tcp default ttl".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (ttl, data) = data.split_at(1);

                Ok((DhcpOption::TcpDefaultTtl(ttl[0]), data))
            }
            38 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse tcp keepalive interval".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse tcp keepalive interval".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (interval, data) = data.split_at(4);

                Ok((
                    DhcpOption::TcpKeepaliveInterval(u32::from_be_bytes([
                        interval[0],
                        interval[1],
                        interval[2],
                        interval[3],
                    ])),
                    data,
                ))
            }
            39 => {
                // Check that the data has at least 1 bytes.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse tcp keepalive garbage".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse tcp keepalive garbage".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (garbage, data) = data.split_at(1);

                Ok((DhcpOption::TcpKeepaliveGarbage(garbage[0] != 0), data))
            }
            40 => {
                // Check that the data has at least 1 bytes.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network information service domain domain".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse network information service domain domain".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network information service domain domain".to_string(),
                    ));
                }

                // Retrieve the value.
                let (domain, data) = data.split_at(len as usize);

                Ok((
                    DhcpOption::NetworkInformationServiceDomain(
                        String::from_utf8_lossy(domain).to_string(),
                    ),
                    data,
                ))
            }
            41 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network information service servers server address"
                            .to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) =
                    match data.split_first() {
                        Some((len, data)) => (*len, data),
                        None => return Err(DhcpError::ParsingError(
                            "Could not parse network information service servers server address"
                                .to_string(),
                        )),
                    };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network information service servers server address"
                            .to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network information service servers server address"
                            .to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::NetworkInformationServers(servers), data))
            }
            42 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network time protocol servers server address".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse network time protocol servers server address"
                                .to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network time protocol servers server address".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse network time protocol servers server address".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::NetworkTimeProtocolServers(servers), data))
            }
            43 => {
                // Check that the data has at least 1 bytes.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse vendor specific information".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse vendor specific information".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse vendor specific information".to_string(),
                    ));
                }

                // Retrieve the value.
                let (info, data) = data.split_at(len as usize);

                Ok((DhcpOption::VendorSpecificInformation(info.to_vec()), data))
            }
            44 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip name servers server address"
                            .to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse netbios over tcp/ip name servers server address"
                                .to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip name servers server address"
                            .to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip name servers server address"
                            .to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::NetBiosOverTcpIpNameServer(servers), data))
            }
            45 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip datagram distribution server address"
                            .to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip datagram distribution server address"
                            .to_string(),
                    )),
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip datagram distribution server address"
                            .to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip datagram distribution server address"
                            .to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((
                    DhcpOption::NetBiosOverTcpIpDatagramDistributionServer(servers),
                    data,
                ))
            }
            46 => {
                // Check that the data has at least 1 byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip node type".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (_len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse netbios over tcp/ip node type".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (node_type, data) = data.split_at(1);
                let node_type = match node_type[0] {
                    1 => NetBiosOverTcpIpNodeType::BNode,
                    2 => NetBiosOverTcpIpNodeType::PNode,
                    4 => NetBiosOverTcpIpNodeType::MNode,
                    8 => NetBiosOverTcpIpNodeType::HNode,
                    _ => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse netbios over tcp/ip node type".to_string(),
                        ))
                    }
                };

                Ok((DhcpOption::NetBiosOverTcpIpNodeType(node_type), data))
            }
            47 => {
                // Check that the data has at least 1 byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip scope".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse netbios over tcp/ip scope".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse netbios over tcp/ip scope".to_string(),
                    ));
                }

                // Retrieve the value.
                let (scope, data) = data.split_at(len as usize);

                Ok((DhcpOption::NetBiosOverTcpIpScope(scope.to_vec()), data))
            }
            48 => {
                // Check that the data has at least 4 byte.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse X Window System Font server".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse X Window System Font server".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse X Window System Font server".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::XWindowSystemFontServer(servers), data))
            }
            49 => {
                // Check that the data has at least 4 byte.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse X Window System Display Manager".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse X Window System Display Manager".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse X Window System Display Manager".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::XWindowSystemDisplayManager(servers), data))
            }
            64 => {
                // Check that the data has at least 1 byte.
                if data.len() < 2 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Network Information Service+ domain".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Network Information Service+ domain".to_string(),
                        ))
                    }
                };

                // Retrieve the value.
                let (domain, data) = data.split_at(len as usize);

                Ok((
                    DhcpOption::NetworkInformationServicePlusDomain(
                        String::from_utf8_lossy(domain).to_string(),
                    ),
                    data,
                ))
            }
            65 => {
                // Check that the data has at least 4 byte.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Network Information Service+ servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Network Information Service+ servers".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Network Information Service+ servers".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Network Information Service+ servers".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((
                    DhcpOption::NetworkInformationServicePlusServers(servers),
                    data,
                ))
            }
            68 => {
                // Check that the data has at least the length.
                if data.len() < 1 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Mobile Ip Home Agent".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Mobile Ip Home Agent".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Mobile Ip Home Agent".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Mobile Ip Home Agent".to_string(),
                    ));
                }

                // Retrieve the value.
                if len != 0 {
                    let (servers, data) = data.split_at(len as usize);
                    let servers = servers
                        .chunks_exact(4)
                        .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                        .collect::<Vec<Ipv4Addr>>();

                    Ok((DhcpOption::MobileIpHomeAgent(servers), data))
                } else {
                    Ok((DhcpOption::MobileIpHomeAgent(Vec::new()), data))
                }
            }
            69 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Simple Mail Transport Protocol Server servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Simple Mail Transport Protocol Server servers"
                                .to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Simple Mail Transport Protocol Server servers".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Simple Mail Transport Protocol Server servers".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);
                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::SimpleMailTransportProtocolServer(servers), data))
            }
            70 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Post Office Protocol Server servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Post Office Protocol Server servers".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Post Office Protocol Server servers".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Post Office Protocol Server servers".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);

                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::PostOfficeProtocolServer(servers), data))
            }
            71 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Network News Transport Protocol Server servers"
                            .to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Network News Transport Protocol Server servers"
                                .to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Network News Transport Protocol Server servers"
                            .to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Network News Transport Protocol Server servers"
                            .to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);

                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((
                    DhcpOption::NetworkNewsTransportProtocolServer(servers),
                    data,
                ))
            }
            72 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default World Wide Web Server servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Default World Wide Web Server servers".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default World Wide Web Server servers".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default World Wide Web Server servers".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);

                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::DefaultWorldWideWebServer(servers), data))
            }
            73 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default Finger Server servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Default Finger Server servers".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default Finger Server servers".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default Finger Server servers".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);

                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::DefaultFingerServer(servers), data))
            }
            74 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default Internet Relay Chat Server servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Default Internet Relay Chat Server servers"
                                .to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default Internet Relay Chat Server servers".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Default Internet Relay Chat Server servers".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);

                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::DefaultInternetRelayChatServer(servers), data))
            }
            75 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse StreetTalk Server servers".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse StreetTalk Server servers".to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse StreetTalk Server servers".to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse StreetTalk Server servers".to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);

                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((DhcpOption::StreetTalkServer(servers), data))
            }
            76 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse StreetTalk Directory Assistance Server servers"
                            .to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse StreetTalk Directory Assistance Server servers"
                                .to_string(),
                        ))
                    }
                };

                // Verify that the length is possible.
                if data.len() < len as usize {
                    return Err(DhcpError::ParsingError(
                        "Could not parse StreetTalk Directory Assistance Server servers"
                            .to_string(),
                    ));
                }

                // Verify that the length is a multiple of 4.
                if len % 4 != 0 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse StreetTalk Directory Assistance Server servers"
                            .to_string(),
                    ));
                }

                // Retrieve the value.
                let (servers, data) = data.split_at(len as usize);

                let servers = servers
                    .chunks_exact(4)
                    .map(|server| Ipv4Addr::new(server[0], server[1], server[2], server[3]))
                    .collect::<Vec<Ipv4Addr>>();

                Ok((
                    DhcpOption::StreetTalkDirectoryAssistanceServer(servers),
                    data,
                ))
            }
            50 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Requested IP Address".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse Requested IP Address".to_string(),
                        ))
                    }
                };

                // Check that the length is 4.
                if len != 4 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse Requested IP Address".to_string(),
                    ));
                }

                // Retrieve the value.
                let (addr, data) = data.split_at(4);

                let addr = Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3]);

                Ok((DhcpOption::RequestedIpAddress(addr), data))
            }
            51 => {
                // Check that the data has at least 4 bytes.
                if data.len() < 5 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse IP Address Lease Time".to_string(),
                    ));
                }

                // Retrieve the length of the option.
                let (len, data) = match data.split_first() {
                    Some((len, data)) => (*len, data),
                    None => {
                        return Err(DhcpError::ParsingError(
                            "Could not parse IP Address Lease Time".to_string(),
                        ))
                    }
                };

                // Check that the length is 4.
                if len != 4 {
                    return Err(DhcpError::ParsingError(
                        "Could not parse IP Address Lease Time".to_string(),
                    ));
                }

                // Retrieve the value.
                let (time, data) = data.split_at(4);

                let time = u32::from_be_bytes([time[0], time[1], time[2], time[3]]);

                Ok((DhcpOption::IpAddressLeaseTime(time), data))
            }
            _ => Err(DhcpError::ParsingError(format!(
                "Unknown option code: {}",
                code
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NetBiosOverTcpIpNodeType {
    BNode,
    PNode,
    MNode,
    HNode,
}
