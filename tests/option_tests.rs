use dhcp::option::DhcpOption;

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

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
        assert_eq!(option, DhcpOption::SwapServer(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(data, &[]);

        let data = vec![16, 4, 192, 168, 0, 1, 255];
        let (option, data) = DhcpOption::deserialize(&data).unwrap();
        assert_eq!(option, DhcpOption::SwapServer(Ipv4Addr::new(192, 168, 0, 1)));
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
}
