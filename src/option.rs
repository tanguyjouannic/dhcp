use std::net::Ipv4Addr;

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
            _ => Err(DhcpError::ParsingError(format!(
                "Unknown option code: {}",
                code
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_pad_serialize() {
        let option = DhcpOption::Pad;
        let serialized = option.serialize();
        assert_eq!(serialized, vec![0]);
    }

    #[test]
    fn option_pad_deserialize() {
        let data = vec![0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::Pad);
        assert_eq!(data, &[]);

        let data = vec![0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::Pad);
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_end_serialize() {
        let option = DhcpOption::End;
        let serialized = option.serialize();
        assert_eq!(serialized, vec![255]);
    }

    #[test]
    fn option_end_deserialize() {
        let data = vec![255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::End);
        assert_eq!(data, &[]);

        let data = vec![255, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::End);
        assert_eq!(data, &[0]);
    }

    #[test]
    fn option_subnet_mask_serialize() {
        let option = DhcpOption::SubnetMask(Ipv4Addr::new(255, 255, 255, 0));
        let serialized = option.serialize();
        assert_eq!(serialized, vec![1, 4, 255, 255, 255, 0]);
    }

    #[test]
    fn option_subnet_mask_deserialize() {
        let data = vec![1, 4, 255, 255, 255, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::SubnetMask(Ipv4Addr::new(255, 255, 255, 0))
        );
        assert_eq!(data, &[]);

        let data = vec![1, 4, 255, 255, 255, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::SubnetMask(Ipv4Addr::new(255, 255, 255, 0))
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_time_offset_serialize() {
        let option = DhcpOption::TimeOffset(0x12345678);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![2, 4, 0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn option_time_offset_deserialize() {
        let data = vec![2, 4, 0x12, 0x34, 0x56, 0x78];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TimeOffset(0x12345678));
        assert_eq!(data, &[]);

        let data = vec![2, 4, 0x12, 0x34, 0x56, 0x78, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TimeOffset(0x12345678));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_router_serialize() {
        let option = DhcpOption::Router(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![3, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_router_deserialize() {
        let data = vec![3, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::Router(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![3, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::Router(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_time_server_serialize() {
        let option = DhcpOption::TimeServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![4, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_time_server_deserialize() {
        let data = vec![4, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::TimeServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![4, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::TimeServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_domain_name_server_serialize() {
        let option = DhcpOption::NameServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![5, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_domain_name_server_deserialize() {
        let data = vec![5, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::NameServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![5, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::NameServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_log_server_serialize() {
        let option = DhcpOption::LogServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![6, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_log_server_deserialize() {
        let data = vec![6, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::LogServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![6, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::LogServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_cookie_server_serialize() {
        let option = DhcpOption::CookieServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![7, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_cookie_server_deserialize() {
        let data = vec![7, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::CookieServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![7, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::CookieServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }
}
