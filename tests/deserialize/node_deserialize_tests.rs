#[cfg(test)]
mod tests {
    use serde_json::json;
    use data_orchestra::interface::node::ExtNode;

    pub fn get_node(json: serde_json::Value) -> ExtNode  {
        let node: ExtNode = serde_json::from_value(json).expect("Unable to parse json to node");
        node
    }

    #[test]
    pub fn name_empty() {
        let json = json!(
        {
            "host": "127.0.0.1"
        });

        let node = get_node(json);
        assert_eq!(node.name, None);
    }

    #[test]
    pub fn name() {
        let json = json!(
        {
            "name": "NODE",
            "host": "127.0.0.1"
        });

        let node = get_node(json);
        assert_eq!(node.name, Some("NODE".to_string()));
    }

    #[test]
    #[should_panic]
    pub fn host_empty() {
        let json = json!({});
        let node = get_node(json);
    }

    #[test]
    pub fn username_empty() {
        let json = json!(
        {
            "host": "127.0.0.1"
        });

        let node = get_node(json);
        assert_eq!(node.username, None);
    }

    #[test]
    pub fn username() {
        let json = json!(
        {
            "host": "127.0.0.1",
            "username": "ubuntu"
        });

        let node = get_node(json);
        assert_eq!(node.username, Some("ubuntu".to_string()));
    }

    #[test]
    pub fn password_empty() {
        let json = json!(
        {
            "host": "127.0.0.1"
        });

        let node = get_node(json);
        assert_eq!(node.password, None);
    }

    #[test]
    pub fn password() {
        let json = json!(
        {
            "host": "127.0.0.1",
            "password": "password"
        });

        let node = get_node(json);
        assert_eq!(node.password, Some("password".to_string()));
    }

    #[test]
    pub fn ssh_port_empty() {
        let json = json!(
        {
            "host": "127.0.0.1"
        });

        let node = get_node(json);
        assert_eq!(node.ssh_port, 22);
    }

    #[test]
    pub fn ssh_port() {
        let json = json!(
        {
            "host": "127.0.0.1",
            "ssh_port": 5000
        });

        let node = get_node(json);
        assert_eq!(node.ssh_port, 5000);
    }
}