use dhcp::option::DhcpOption;

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use dhcp::option::NetBiosOverTcpIpNodeType;

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
    fn option_name_server_serialize() {
        let option = DhcpOption::NameServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![5, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_name_server_deserialize() {
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
    fn option_domain_name_server_serialize() {
        let option = DhcpOption::DomainNameServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![6, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_domain_name_server_deserialize() {
        let data = vec![6, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::DomainNameServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![6, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::DomainNameServer(vec![
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
        assert_eq!(serialized, vec![7, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_log_server_deserialize() {
        let data = vec![7, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::LogServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![7, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
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
        assert_eq!(serialized, vec![8, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_cookie_server_deserialize() {
        let data = vec![8, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::CookieServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![8, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
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

    #[test]
    fn option_lpr_server_serialize() {
        let option = DhcpOption::LprServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![9, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_lpr_server_deserialize() {
        let data = vec![9, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::LprServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![9, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::LprServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_impress_server_serialize() {
        let option = DhcpOption::ImpressServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![10, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_impress_server_deserialize() {
        let data = vec![10, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::ImpressServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![10, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::ImpressServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_resource_location_server_serialize() {
        let option = DhcpOption::ResourceLocationServer(vec![
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 2),
        ]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![11, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_resource_location_server_deserialize() {
        let data = vec![11, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::ResourceLocationServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![11, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::ResourceLocationServer(vec![
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_host_name_serialize() {
        let option = DhcpOption::HostName("host".to_string());
        let serialized = option.serialize();
        assert_eq!(serialized, vec![12, 4, 104, 111, 115, 116]);
    }

    #[test]
    fn option_host_name_deserialize() {
        let data = vec![12, 4, 104, 111, 115, 116];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::HostName("host".to_string()));
        assert_eq!(data, &[]);

        let data = vec![12, 4, 104, 111, 115, 116, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::HostName("host".to_string()));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_boot_file_size_serialize() {
        let option = DhcpOption::BootFileSize(1024);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![13, 2, 4, 0]);
    }

    #[test]
    fn option_boot_file_size_deserialize() {
        let data = vec![13, 2, 4, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::BootFileSize(1024));
        assert_eq!(data, &[]);

        let data = vec![13, 2, 4, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::BootFileSize(1024));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_merit_dump_file_serialize() {
        let option = DhcpOption::MeritDumpFile("dump".to_string());
        let serialized = option.serialize();
        assert_eq!(serialized, vec![14, 4, 100, 117, 109, 112]);
    }

    #[test]
    fn option_merit_dump_file_deserialize() {
        let data = vec![14, 4, 100, 117, 109, 112];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MeritDumpFile("dump".to_string()));
        assert_eq!(data, &[]);

        let data = vec![14, 4, 100, 117, 109, 112, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MeritDumpFile("dump".to_string()));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_domain_name_serialize() {
        let option = DhcpOption::DomainName("domain".to_string());
        let serialized = option.serialize();
        assert_eq!(serialized, vec![15, 6, 100, 111, 109, 97, 105, 110]);
    }

    #[test]
    fn option_domain_name_deserialize() {
        let data = vec![15, 6, 100, 111, 109, 97, 105, 110];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::DomainName("domain".to_string()));
        assert_eq!(data, &[]);

        let data = vec![15, 6, 100, 111, 109, 97, 105, 110, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::DomainName("domain".to_string()));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_swap_server_serialize() {
        let option = DhcpOption::SwapServer(Ipv4Addr::new(192, 168, 0, 1));
        let serialized = option.serialize();
        assert_eq!(serialized, vec![16, 4, 192, 168, 0, 1]);
    }

    #[test]
    fn option_swap_server_deserialize() {
        let data = vec![16, 4, 192, 168, 0, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::SwapServer(Ipv4Addr::new(192, 168, 0, 1))
        );
        assert_eq!(data, &[]);

        let data = vec![16, 4, 192, 168, 0, 1, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::SwapServer(Ipv4Addr::new(192, 168, 0, 1))
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_root_path_serialize() {
        let option = DhcpOption::RootPath("path".to_string());
        let serialized = option.serialize();
        assert_eq!(serialized, vec![17, 4, 112, 97, 116, 104]);
    }

    #[test]
    fn option_root_path_deserialize() {
        let data = vec![17, 4, 112, 97, 116, 104];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::RootPath("path".to_string()));
        assert_eq!(data, &[]);

        let data = vec![17, 4, 112, 97, 116, 104, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::RootPath("path".to_string()));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_extension_path_serialize() {
        let option = DhcpOption::ExtensionsPath("path".to_string());
        let serialized = option.serialize();
        assert_eq!(serialized, vec![18, 4, 112, 97, 116, 104]);
    }

    #[test]
    fn option_extension_path_deserialize() {
        let data = vec![18, 4, 112, 97, 116, 104];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::ExtensionsPath("path".to_string()));
        assert_eq!(data, &[]);

        let data = vec![18, 4, 112, 97, 116, 104, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::ExtensionsPath("path".to_string()));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_ip_forwarding_serialize() {
        let option = DhcpOption::IpForwarding(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![19, 1, 1]);

        let option = DhcpOption::IpForwarding(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![19, 1, 0]);
    }

    #[test]
    fn option_ip_forwarding_deserialize() {
        let data = vec![19, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::IpForwarding(true));
        assert_eq!(data, &[]);

        let data = vec![19, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::IpForwarding(false));
        assert_eq!(data, &[]);

        let data = vec![19, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::IpForwarding(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_non_local_source_routing_serialize() {
        let option = DhcpOption::NonLocalSourceRouting(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![20, 1, 1]);

        let option = DhcpOption::NonLocalSourceRouting(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![20, 1, 0]);
    }

    #[test]
    fn option_non_local_source_routing_deserialize() {
        let data = vec![20, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NonLocalSourceRouting(true));
        assert_eq!(data, &[]);

        let data = vec![20, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NonLocalSourceRouting(false));
        assert_eq!(data, &[]);

        let data = vec![20, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NonLocalSourceRouting(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_policy_filter_serialize() {
        let option = DhcpOption::PolicyFilter(vec![
            (
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(255, 255, 255, 0),
            ),
            (
                Ipv4Addr::new(192, 168, 0, 2),
                Ipv4Addr::new(255, 255, 255, 0),
            ),
        ]);
        let serialized = option.serialize();
        assert_eq!(
            serialized,
            vec![21, 16, 192, 168, 0, 1, 255, 255, 255, 0, 192, 168, 0, 2, 255, 255, 255, 0]
        );
    }

    #[test]
    fn option_policy_filter_deserialize() {
        let data = vec![
            21, 16, 192, 168, 0, 1, 255, 255, 255, 0, 192, 168, 0, 2, 255, 255, 255, 0,
        ];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::PolicyFilter(vec![
                (
                    Ipv4Addr::new(192, 168, 0, 1),
                    Ipv4Addr::new(255, 255, 255, 0)
                ),
                (
                    Ipv4Addr::new(192, 168, 0, 2),
                    Ipv4Addr::new(255, 255, 255, 0)
                ),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![
            21, 16, 192, 168, 0, 1, 255, 255, 255, 0, 192, 168, 0, 2, 255, 255, 255, 0, 255,
        ];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::PolicyFilter(vec![
                (
                    Ipv4Addr::new(192, 168, 0, 1),
                    Ipv4Addr::new(255, 255, 255, 0)
                ),
                (
                    Ipv4Addr::new(192, 168, 0, 2),
                    Ipv4Addr::new(255, 255, 255, 0)
                ),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_max_datagram_reassembly_size_serialize() {
        let option = DhcpOption::MaximumDatagramReassemblySize(1500);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![22, 2, 5, 220]);
    }

    #[test]
    fn option_max_datagram_reassembly_size_deserialize() {
        let data = vec![22, 2, 5, 220];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MaximumDatagramReassemblySize(1500));
        assert_eq!(data, &[]);

        let data = vec![22, 2, 5, 220, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MaximumDatagramReassemblySize(1500));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_default_ip_ttl_serialize() {
        let option = DhcpOption::DefaultIpTimeToLive(64);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![23, 1, 64]);
    }

    #[test]
    fn option_default_ip_ttl_deserialize() {
        let data = vec![23, 1, 64];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::DefaultIpTimeToLive(64));
        assert_eq!(data, &[]);

        let data = vec![23, 1, 64, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::DefaultIpTimeToLive(64));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_path_mtu_aging_timeout_serialize() {
        let option = DhcpOption::PathMtuAgingTimeout(1500);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![24, 4, 0, 0, 5, 220]);
    }

    #[test]
    fn option_path_mtu_aging_timeout_deserialize() {
        let data = vec![24, 4, 0, 0, 5, 220];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PathMtuAgingTimeout(1500));
        assert_eq!(data, &[]);

        let data = vec![24, 4, 0, 0, 5, 220, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PathMtuAgingTimeout(1500));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_path_mtu_plateau_table_serialize() {
        let option = DhcpOption::PathMtuPlateauTable(vec![1500, 1499]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![25, 4, 5, 220, 5, 219]);
    }

    #[test]
    fn option_path_mtu_plateau_table_deserialize() {
        let data = vec![25, 4, 5, 220, 5, 219];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PathMtuPlateauTable(vec![1500, 1499]));
        assert_eq!(data, &[]);

        let data = vec![25, 4, 5, 220, 5, 219, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PathMtuPlateauTable(vec![1500, 1499]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_interface_mtu_serialize() {
        let option = DhcpOption::InterfaceMtu(1500);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![26, 2, 5, 220]);
    }

    #[test]
    fn option_interface_mtu_deserialize() {
        let data = vec![26, 2, 5, 220];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::InterfaceMtu(1500));
        assert_eq!(data, &[]);

        let data = vec![26, 2, 5, 220, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::InterfaceMtu(1500));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_all_subnets_are_local_serialize() {
        let option = DhcpOption::AllSubnetsAreLocal(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![27, 1, 1]);

        let option = DhcpOption::AllSubnetsAreLocal(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![27, 1, 0]);
    }

    #[test]
    fn option_all_subnets_are_local_deserialize() {
        let data = vec![27, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::AllSubnetsAreLocal(true));
        assert_eq!(data, &[]);

        let data = vec![27, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::AllSubnetsAreLocal(false));
        assert_eq!(data, &[]);

        let data = vec![27, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::AllSubnetsAreLocal(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_broadcast_address_serialize() {
        let option = DhcpOption::BroadcastAddress(Ipv4Addr::new(192, 168, 1, 255));
        let serialized = option.serialize();
        assert_eq!(serialized, vec![28, 4, 192, 168, 1, 255]);
    }

    #[test]
    fn option_broadcast_address_deserialize() {
        let data = vec![28, 4, 192, 168, 1, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::BroadcastAddress(Ipv4Addr::new(192, 168, 1, 255))
        );
        assert_eq!(data, &[]);

        let data = vec![28, 4, 192, 168, 1, 255, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::BroadcastAddress(Ipv4Addr::new(192, 168, 1, 255))
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_perform_mask_discovery_serialize() {
        let option = DhcpOption::PerformMaskDiscovery(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![29, 1, 1]);

        let option = DhcpOption::PerformMaskDiscovery(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![29, 1, 0]);
    }

    #[test]
    fn option_perform_mask_discovery_deserialize() {
        let data = vec![29, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PerformMaskDiscovery(true));
        assert_eq!(data, &[]);

        let data = vec![29, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PerformMaskDiscovery(false));
        assert_eq!(data, &[]);

        let data = vec![29, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PerformMaskDiscovery(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_mask_supplier_serialize() {
        let option = DhcpOption::MaskSupplier(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![30, 1, 1]);

        let option = DhcpOption::MaskSupplier(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![30, 1, 0]);
    }

    #[test]
    fn option_mask_supplier_deserialize() {
        let data = vec![30, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MaskSupplier(true));
        assert_eq!(data, &[]);

        let data = vec![30, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MaskSupplier(false));
        assert_eq!(data, &[]);

        let data = vec![30, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MaskSupplier(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_perform_router_discovery_serialize() {
        let option = DhcpOption::PerformRouterDiscovery(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![31, 1, 1]);

        let option = DhcpOption::PerformRouterDiscovery(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![31, 1, 0]);
    }

    #[test]
    fn option_perform_router_discovery_deserialize() {
        let data = vec![31, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PerformRouterDiscovery(true));
        assert_eq!(data, &[]);

        let data = vec![31, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PerformRouterDiscovery(false));
        assert_eq!(data, &[]);

        let data = vec![31, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PerformRouterDiscovery(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_router_solicitation_address_serialize() {
        let option = DhcpOption::RouterSolicitationAddress(Ipv4Addr::new(192, 168, 1, 1));
        let serialized = option.serialize();
        assert_eq!(serialized, vec![32, 4, 192, 168, 1, 1]);
    }

    #[test]
    fn option_router_solicitation_address_deserialize() {
        let data = vec![32, 4, 192, 168, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::RouterSolicitationAddress(Ipv4Addr::new(192, 168, 1, 1))
        );
        assert_eq!(data, &[]);

        let data = vec![32, 4, 192, 168, 1, 1, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::RouterSolicitationAddress(Ipv4Addr::new(192, 168, 1, 1))
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_static_route_serialize() {
        let option = DhcpOption::StaticRoute(vec![
            (
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 100),
            ),
            (
                Ipv4Addr::new(192, 168, 0, 2),
                Ipv4Addr::new(192, 168, 0, 200),
            ),
        ]);
        let serialized = option.serialize();
        assert_eq!(
            serialized,
            vec![
                33, 16, 192, 168, 0, 1, 192, 168, 0, 100, 192, 168, 0, 2, 192, 168, 0, 200
            ]
        );
    }

    #[test]
    fn option_static_route_deserialize() {
        let data = vec![
            33, 16, 192, 168, 0, 1, 192, 168, 0, 100, 192, 168, 0, 2, 192, 168, 0, 200,
        ];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::StaticRoute(vec![
                (
                    Ipv4Addr::new(192, 168, 0, 1),
                    Ipv4Addr::new(192, 168, 0, 100),
                ),
                (
                    Ipv4Addr::new(192, 168, 0, 2),
                    Ipv4Addr::new(192, 168, 0, 200),
                ),
            ])
        );
        assert_eq!(data, &[]);

        let data = vec![
            33, 16, 192, 168, 0, 1, 192, 168, 0, 100, 192, 168, 0, 2, 192, 168, 0, 200, 255,
        ];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(
            option,
            DhcpOption::StaticRoute(vec![
                (
                    Ipv4Addr::new(192, 168, 0, 1),
                    Ipv4Addr::new(192, 168, 0, 100),
                ),
                (
                    Ipv4Addr::new(192, 168, 0, 2),
                    Ipv4Addr::new(192, 168, 0, 200),
                ),
            ])
        );
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_trailer_encapsulation_serialize() {
        let option = DhcpOption::TrailerEncapsulation(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![34, 1, 1]);

        let option = DhcpOption::TrailerEncapsulation(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![34, 1, 0]);
    }

    #[test]
    fn option_trailer_encapsulation_deserialize() {
        let data = vec![34, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TrailerEncapsulation(true));
        assert_eq!(data, &[]);

        let data = vec![34, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TrailerEncapsulation(false));
        assert_eq!(data, &[]);

        let data = vec![34, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TrailerEncapsulation(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_arp_cache_timeout_serialize() {
        let option = DhcpOption::ArpCacheTimeout(1234);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![35, 4, 0, 0, 4, 210]);
    }

    #[test]
    fn option_arp_cache_timeout_deserialize() {
        let data = vec![35, 4, 0, 0, 4, 210];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::ArpCacheTimeout(1234));
        assert_eq!(data, &[]);

        let data = vec![35, 4, 0, 0, 4, 210, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::ArpCacheTimeout(1234));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_ethernet_encapsulation_serialize() {
        let option = DhcpOption::EthernetEncapsulation(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![36, 1, 1]);

        let option = DhcpOption::EthernetEncapsulation(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![36, 1, 0]);
    }

    #[test]
    fn option_ethernet_encapsulation_deserialize() {
        let data = vec![36, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::EthernetEncapsulation(true));
        assert_eq!(data, &[]);

        let data = vec![36, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::EthernetEncapsulation(false));
        assert_eq!(data, &[]);

        let data = vec![36, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::EthernetEncapsulation(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_tcp_default_ttl_serialize() {
        let option = DhcpOption::TcpDefaultTtl(123);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![37, 1, 123]);
    }

    #[test]
    fn option_tcp_default_ttl_deserialize() {
        let data = vec![37, 1, 123];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TcpDefaultTtl(123));
        assert_eq!(data, &[]);

        let data = vec![37, 1, 123, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TcpDefaultTtl(123));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_tcp_keepalive_interval_serialize() {
        let option = DhcpOption::TcpKeepaliveInterval(1234);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![38, 4, 0, 0, 4, 210]);
    }

    #[test]
    fn option_tcp_keepalive_interval_deserialize() {
        let data = vec![38, 4, 0, 0, 4, 210];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TcpKeepaliveInterval(1234));
        assert_eq!(data, &[]);

        let data = vec![38, 4, 0, 0, 4, 210, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TcpKeepaliveInterval(1234));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_tcp_keepalive_garbage_serialize() {
        let option = DhcpOption::TcpKeepaliveGarbage(true);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![39, 1, 1]);

        let option = DhcpOption::TcpKeepaliveGarbage(false);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![39, 1, 0]);
    }

    #[test]
    fn option_tcp_keepalive_garbage_deserialize() {
        let data = vec![39, 1, 1];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TcpKeepaliveGarbage(true));
        assert_eq!(data, &[]);

        let data = vec![39, 1, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TcpKeepaliveGarbage(false));
        assert_eq!(data, &[]);

        let data = vec![39, 1, 0, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::TcpKeepaliveGarbage(false));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_network_information_servers_serialize() {
        let option = DhcpOption::NetworkInformationServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![41, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_network_information_servers_deserialize() {
        let data = vec![41, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkInformationServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![41, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkInformationServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_network_time_protocol_servers_serialize() {
        let option = DhcpOption::NetworkTimeProtocolServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![42, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_network_time_protocol_servers_deserialize() {
        let data = vec![42, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkTimeProtocolServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![42, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkTimeProtocolServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_vendor_specific_information_serialize() {
        let option = DhcpOption::VendorSpecificInformation(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![43, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn option_vendor_specific_information_deserialize() {
        let data = vec![43, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::VendorSpecificInformation(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(data, &[]);

        let data = vec![43, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::VendorSpecificInformation(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_netbios_over_tcpip_name_server_serialize() {
        let option = DhcpOption::NetBiosOverTcpIpNameServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![44, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_netbios_over_tcpip_name_server_deserialize() {
        let data = vec![44, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpNameServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![44, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpNameServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_netbios_over_tcpip_datagram_distribution_server_serialize() {
        let option = DhcpOption::NetBiosOverTcpIpDatagramDistributionServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![45, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_netbios_over_tcpip_datagram_distribution_server_deserialize() {
        let data = vec![45, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpDatagramDistributionServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![45, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpDatagramDistributionServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_netbios_over_tcpip_node_type_serialize() {
        let option = DhcpOption::NetBiosOverTcpIpNodeType(NetBiosOverTcpIpNodeType::PNode);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![46, 1, 2]);
    }

    #[test]
    fn option_netbios_over_tcpip_node_type_deserialize() {
        let data = vec![46, 1, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpNodeType(NetBiosOverTcpIpNodeType::PNode));
        assert_eq!(data, &[]);

        let data = vec![46, 1, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpNodeType(NetBiosOverTcpIpNodeType::PNode));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_netbios_over_tcpip_scope_serialize() {
        let option = DhcpOption::NetBiosOverTcpIpScope(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![47, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn option_netbios_over_tcpip_scope_deserialize() {
        let data = vec![47, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpScope(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(data, &[]);

        let data = vec![47, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetBiosOverTcpIpScope(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
        assert_eq!(data, &[255]);
    }   

    #[test]
    fn option_x_window_system_font_server_serialize() {
        let option = DhcpOption::XWindowSystemFontServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![48, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_x_window_system_font_server_deserialize() {
        let data = vec![48, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::XWindowSystemFontServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![48, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::XWindowSystemFontServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_x_window_system_display_manager_serialize() {
        let option = DhcpOption::XWindowSystemDisplayManager(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![49, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_x_window_system_display_manager_deserialize() {
        let data = vec![49, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::XWindowSystemDisplayManager(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![49, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::XWindowSystemDisplayManager(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_network_information_service_plus_domain_serialize() {
        let option = DhcpOption::NetworkInformationServicePlusDomain("domain".to_string());
        let serialized = option.serialize();
        assert_eq!(serialized, vec![64, 6, 100, 111, 109, 97, 105, 110]);
    }

    #[test]
    fn option_network_information_service_plus_domain_deserialize() {
        let data = vec![64, 6, 100, 111, 109, 97, 105, 110];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkInformationServicePlusDomain("domain".to_string()));
        assert_eq!(data, &[]);

        let data = vec![64, 6, 100, 111, 109, 97, 105, 110, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkInformationServicePlusDomain("domain".to_string()));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_network_information_service_plus_servers_serialize() {
        let option = DhcpOption::NetworkInformationServicePlusServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![65, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_network_information_service_plus_servers_deserialize() {
        let data = vec![65, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkInformationServicePlusServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![65, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkInformationServicePlusServers(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_mobile_ip_home_agent_serialize() {
        let option = DhcpOption::MobileIpHomeAgent(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![68, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_mobile_ip_home_agent_deserialize() {
        let data = vec![68, 0];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MobileIpHomeAgent(vec![]));
        assert_eq!(data, &[]);

        let data = vec![68, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MobileIpHomeAgent(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![68, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::MobileIpHomeAgent(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_simple_mail_transport_protocol_serialize() {
        let option = DhcpOption::SimpleMailTransportProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![69, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_simple_mail_transport_protocol_deserialize() {
        let data = vec![69, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::SimpleMailTransportProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![69, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::SimpleMailTransportProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_post_office_protocol_server_serialize() {
        let option = DhcpOption::PostOfficeProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![70, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_post_office_protocol_server_deserialize() {
        let data = vec![70, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PostOfficeProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![70, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::PostOfficeProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }

    #[test]
    fn option_network_news_transport_protocol_serialize() {
        let option = DhcpOption::NetworkNewsTransportProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
        let serialized = option.serialize();
        assert_eq!(serialized, vec![71, 8, 192, 168, 0, 1, 192, 168, 0, 2]);
    }

    #[test]
    fn option_network_news_transport_protocol_deserialize() {
        let data = vec![71, 8, 192, 168, 0, 1, 192, 168, 0, 2];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkNewsTransportProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[]);

        let data = vec![71, 8, 192, 168, 0, 1, 192, 168, 0, 2, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::NetworkNewsTransportProtocolServer(vec![Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]));
        assert_eq!(data, &[255]);
    }
}
