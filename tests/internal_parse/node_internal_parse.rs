#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use data_orchestra::interface::node::ExtNode;
    use data_orchestra::interface::upload::UploadTypes;
    use data_orchestra::shared::ToInternal;

    #[test]
    pub fn node() {
        let node = ExtNode
        {
            name: Some("NODE".to_string()),
            host: IpAddr::V4(Ipv4Addr::LOCALHOST),
            username: Some("ubuntu".to_string()),
            password: Some("password".to_string()),
            ssh_port: 22,
            upload_schema: UploadTypes::Ssh,
            ssh_key: None
        };

        let (internal_node, uploader) = node.to_internal();

        assert_eq!(internal_node.ssh_port, 22);
        assert_eq!(internal_node.password, Some("password".to_string()));
        assert_eq!(internal_node.username, "ubuntu".to_string());
        assert!(internal_node.ssh.is_none());
        assert_eq!(internal_node.host, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(internal_node.name, "NODE".to_string());
    }
}