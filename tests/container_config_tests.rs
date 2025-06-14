#[cfg(test)]
mod tests {
    use data_orchestra::core::adapters::ContainerBuilder;
    use data_orchestra::core::adapters::{BindPropagation, Mount};
    use rstest::rstest;

    #[test]
    fn docker_network() {
        let container = ContainerBuilder::default()
            .network("docker_network")
            .build();

        let command = container.config.parse_options(); 
        assert!(command.contains("--network=docker_network"), "Expected docker network");
    }

    #[test]
    fn docker_name() {
        let container = ContainerBuilder::default()
            .name("rust")
            .build();

        let command = container.config.parse_options();
        assert!(command.contains("--name=rust"), "Expected docker name");
    }

    #[test]
    fn docker_expose() {
        let container = ContainerBuilder::default()
            .expose(true)
            .build();
        
        let command = container.config.parse_options();
        let command = command.split_whitespace().collect::<Vec<_>>();

        assert!(command.contains(&"--expose"), "Expected expose");
    }

    #[test]
    fn docker_publish_all() {
        let container = ContainerBuilder::default()
            .publish_all(true)
            .build();
        
        let command = container.config.parse_options();
        let command = command.split_whitespace().collect::<Vec<_>>();

        assert!(command.contains(&"-P"), "Expected publish all");
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec![10])]
    #[case(vec![10, 20])]
    fn docker_publish_ports(#[case] ports: Vec<u16>) {
        let mut builder = ContainerBuilder::default();

        for port in ports.iter() {
            builder.publish_mut(port.to_owned());
        }

        let container = builder.build();
        
        let command = container.config.parse_options();
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
        let mut builder = ContainerBuilder::default();

        for (left, right) in map.clone() {
            builder.publish_map_mut(left, right);
        }

        let container = builder.build();

        let command = container.config.parse_options();
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
        let mut builder = ContainerBuilder::default();

        for (left, right) in map.clone() {
            builder.env_var_mut(left, right);
        }

        let container = builder.build();

        let command = container.config.parse_options();
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
        let mut builder = ContainerBuilder::default();

        for volume in volumes.clone() {
            builder.volume_mut(volume);
        }

        let container = builder.build();

        let command = container.config.parse_options();
        let command = command.split_whitespace().collect::<Vec<_>>();

        for volume in volumes.iter() {
            let needle = ["-v", volume];
            let found = command.windows(2).any(|s| s == needle);
            assert!(found, "Expected volume");
        }
    }

    #[rstest]
    #[case(vec![Mount::new("/source", "/destination", false, None)], vec!["type=bind,src=/source,dst=/destination"])]
    #[case(vec![Mount::new("/source", "/destination", true, None)], vec!["type=bind,src=/source,ro,dst=/destination"])]
    #[case(vec![Mount::new("/source", "/destination", true, Some(BindPropagation::Shared))], vec!["type=bind,src=/source,ro,dst=/destination,bind-propagation=shared"])]
    #[case(vec![Mount::new("/source", "/destination", true, Some(BindPropagation::Slave))], vec!["type=bind,src=/source,ro,dst=/destination,bind-propagation=slave"])]
    #[case(vec![Mount::new("/source", "/destination", true, Some(BindPropagation::Private))], vec!["type=bind,src=/source,ro,dst=/destination,bind-propagation=private"])]
    #[case(vec![Mount::new("/source", "/destination", true, Some(BindPropagation::RShared))], vec!["type=bind,src=/source,ro,dst=/destination,bind-propagation=rshared"])]
    #[case(vec![Mount::new("/source", "/destination", true, Some(BindPropagation::RSlave))], vec!["type=bind,src=/source,ro,dst=/destination,bind-propagation=rslave"])]
    #[case(vec![Mount::new("/source", "/destination", true, Some(BindPropagation::RPrivate))], vec!["type=bind,src=/source,ro,dst=/destination,bind-propagation=rprivate"])]
    fn docker_mounts(#[case] mounts: Vec<Mount>, #[case] expected: Vec<&str>) {
        let mut builder = ContainerBuilder::default();

        for mount in mounts.clone() {
            builder.mount_mut(mount);
        }

        let container = builder.build();

        let command = container.config.parse_options();
        let command = command.split_whitespace().collect::<Vec<_>>();

        for expect in expected.iter() {
            assert!(command.contains(&expect), "Expected mount");
        }
    }
}
