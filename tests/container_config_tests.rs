#[cfg(test)]
mod tests {
    use data_orchestra::core::adapters::ContainerConfigBuilder;
    use rstest::rstest;

    #[test]
    fn docker_network() {
        let config = ContainerConfigBuilder::default()
            .network("docker_network")
            .build();

        let command = config.parse(); 
        assert!(command.contains("--network=docker_network"), "Expected docker network");
    }

    #[test]
    fn docker_name() {
        let config = ContainerConfigBuilder::default()
            .name("rust")
            .build();

        let command = config.parse();
        assert!(command.contains("--name=rust"), "Expected docker name");
    }

    #[test]
    fn docker_expose() {
        let config = ContainerConfigBuilder::default()
            .expose(true)
            .build();
        
        let command = config.parse();
        let command = command.split_whitespace().collect::<Vec<_>>();

        assert!(command.contains(&"--expose"), "Expected expose");
    }

    #[test]
    fn docker_publish_all() {
        let config = ContainerConfigBuilder::default()
            .publish_all(true)
            .build();
        
        let command = config.parse();
        let command = command.split_whitespace().collect::<Vec<_>>();

        assert!(command.contains(&"-P"), "Expected publish all");
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec![10])]
    #[case(vec![10, 20])]
    fn docker_publish_ports(#[case] ports: Vec<u16>) {
        let mut config = ContainerConfigBuilder::default();

        for port in ports.iter() {
            config.publish_mut(port.to_owned());
        }

        let config = config.build();
        
        let command = config.parse();
        let command = command.split_whitespace().collect::<Vec<_>>();

        for port in ports.iter() {
            let port = port.to_string();
            let needle = ["-p", port.as_str()];
            let found = command.windows(2).any(|s| s == needle);
            assert!(found, "Expected published port");
        }
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec![(10, 20)])]
    #[case(vec![(10, 20), (30, 40)])]
    fn docker_publish_map(#[case] map: Vec<(u16, u16)>) {
        let mut config = ContainerConfigBuilder::default();

        for (left, right) in map.clone() {
            config.publish_map_mut(left, right);
        }

        let config = config.build();

        let command = config.parse();
        let command = command.split_whitespace().collect::<Vec<_>>();

        for (left, right) in map.iter() {
            let string = format!("{}:{}", left, right);
            let needle = ["-p",string.as_str()];
            let found = command.windows(2).any(|s| s == needle);
            assert!(found, "Expected port mapping")
        }
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec![("person", "name")])]
    #[case(vec![("person", "name"), ("key", "value")])]
    fn docker_enviroment_variables(#[case] map: Vec<(&str, &str)>) {
        let mut config = ContainerConfigBuilder::default();

        for (left, right) in map.clone() {
            config.env_var_mut(left, right);
        }

        let config = config.build();

        let command = config.parse();
        let command = command.split_whitespace().collect::<Vec<_>>();

        for (key, value) in map.iter() {
            let string = format!("{}={}", key, value);
            let needle = ["-e",string.as_str()];
            let found = command.windows(2).any(|s| s == needle);
            assert!(found, "Expected environment variables");
        }
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec!["/path/to:/other/path"])]
    #[case(vec!["/path/to:/other/path", "/another/path:/other/path"])]
    fn docker_volumes(#[case] volumes: Vec<&str>) {
        let mut config = ContainerConfigBuilder::default();

        for volume in volumes.clone() {
            config.volume_mut(volume);
        }

        let config = config.build();

        let command = config.parse();
        let command = command.split_whitespace().collect::<Vec<_>>();

        for volume in volumes.iter() {
            let needle = ["-v", volume];
            let found = command.windows(2).any(|s| s == needle);
            assert!(found, "Expected environment variables");
        }
    }
}
