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
    // Code
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
    // Code
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
    // Code   Len        Subnet Mask
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
    // Code   Len        Time Offset
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len         Address 1               Address 2
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
    // Code   Len                 Host Name
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
    // Code   Len   File Size
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
    // Code   Len      Dump File Pathname
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
    // Code   Len        Domain Name
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
    // Code   Len    Swap Server Address
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
    // Code   Len      Root Disk Pathname
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
    // Code   Len      Extensions Pathname
    // +-----+-----+-----+-----+-----+-----+---
    // |  18 |  n  |  n1 |  n2 |  n3 |  n4 | ...
    // +-----+-----+-----+-----+-----+-----+---
    ExtensionsPath(String),
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
            _ => Err(DhcpError::ParsingError(format!(
                "Unknown option code: {}",
                code
            ))),
        }
    }
}
