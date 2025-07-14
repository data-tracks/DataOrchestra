#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;
    use serde_json::json;
    use data_orchestra::core::types::DataTypes;
    use data_orchestra::interface::data::{ExtData, ExtDataTypes, ExtVolatile, VolatileTypes};
    use data_orchestra::interface::location::Location;
    use data_orchestra::shared::ToInternal;

    #[test]
    pub fn env_node() {
        let map = HashMap::from([("KEY_1".to_string(), "VALUE_1".to_string()), ("KEY_2".to_string(), "VALUE_2".to_string())]);
        let env = VolatileTypes::Env(map);

        let data_type = ExtDataTypes::Volatile(ExtVolatile { location: Location::Node, name: Some("NAME".to_string()), destination: "/destination".to_string(), volatile_types: env });
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::VolatileNodeData(_));
        let volatile = internal_data_type.get_volatile_node_data_ref();
        let actual_lines: HashSet<_> = volatile.content.lines().collect();
        let expected_lines: HashSet<_> = ["KEY_1=VALUE_1", "KEY_2=VALUE_2"].iter().copied().collect();
        assert_eq!(actual_lines, expected_lines);
        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.destination, PathBuf::from("/destination"));
    }

    #[test]
    pub fn env_container() {
        let map = HashMap::from([("KEY_1".to_string(), "VALUE_1".to_string()), ("KEY_2".to_string(), "VALUE_2".to_string())]);
        let env = VolatileTypes::Env(map);

        let data_type = ExtDataTypes::Volatile(ExtVolatile { location: Location::Container, name: Some("NAME".to_string()), destination: "/destination".to_string(), volatile_types: env });
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::VolatileDockerData(_));

        let volatile = internal_data_type.get_volatile_docker_data_ref();
        let actual_lines: HashSet<_> = volatile.content.lines().collect();
        let expected_lines: HashSet<_> = ["KEY_1=VALUE_1", "KEY_2=VALUE_2"].iter().copied().collect();
        assert_eq!(actual_lines, expected_lines);
        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.destination, PathBuf::from("/destination"));
    }

    #[test]
    pub fn json_container() {
        let map = json!({ "KEY": "VALUE", "MAP": { "KEY": "VALUE" } });
        let json = VolatileTypes::Json(map.as_object().unwrap().to_owned());

        let data_type = ExtDataTypes::Volatile(ExtVolatile { location: Location::Container, name: Some("NAME".to_string()), destination: "/destination".to_string(), volatile_types: json });
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::VolatileDockerData(_));
        let volatile = internal_data_type.get_volatile_docker_data_ref();
        assert_eq!(volatile.content, serde_json::to_string_pretty(&map).expect(""));
        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.destination, PathBuf::from("/destination"));
    }

    #[test]
    pub fn json_node() {
        let map = json!({ "KEY": "VALUE", "MAP": { "KEY": "VALUE" } });
        let json = VolatileTypes::Json(map.as_object().unwrap().to_owned());

            let data_type = ExtDataTypes::Volatile(ExtVolatile { location: Location::Node, name: Some("NAME".to_string()), destination: "/destination".to_string(), volatile_types: json });
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::VolatileNodeData(_));
        let volatile = internal_data_type.get_volatile_node_data_ref();
        assert_eq!(volatile.content, serde_json::to_string_pretty(&map).expect(""));
        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.destination, PathBuf::from("/destination"));
    }

    #[test]
    pub fn content_container() {
        let content = VolatileTypes::Content("CONTENT".to_string());


        let data_type = ExtDataTypes::Volatile(ExtVolatile { location: Location::Container, name: Some("NAME".to_string()), destination: "/destination".to_string(), volatile_types: content });
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::VolatileDockerData(_));
        let volatile = internal_data_type.get_volatile_docker_data_ref();
        assert_eq!(volatile.content, "CONTENT".to_string());
        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.destination, PathBuf::from("/destination"));
    }

    #[test]
    pub fn content_node() {
        let content = VolatileTypes::Content("CONTENT".to_string());

        let data_type = ExtDataTypes::Volatile(ExtVolatile { location: Location::Node, name: Some("NAME".to_string()), destination: "/destination".to_string(), volatile_types: content });
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::VolatileNodeData(_));
        let volatile = internal_data_type.get_volatile_node_data_ref();
        assert_eq!(volatile.content, "CONTENT".to_string());
        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.destination, PathBuf::from("/destination"));
    }

    #[test]
    pub fn data_node() {
        let data_type = ExtDataTypes::Data(ExtData { location: Location::Node, name: Some("NAME".to_string()), path: "/path".to_string(), destination: "/destination".to_string(), dependency: Some("DEPENDENCY".to_string()) } );
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::NodeData(_));
        let data = internal_data_type.get_node_data_ref();
        assert_eq!(data.name, Some("NAME".to_string()));
        assert_eq!(data.source, "/path".to_string());
        assert_eq!(data.destination, "/destination".to_string());
        assert_eq!(data.dependency, Some("DEPENDENCY".to_string()));
    }

    #[test]
    pub fn data_container() {
        let data_type = ExtDataTypes::Data(ExtData { location: Location::Container, name: Some("NAME".to_string()), path: "/path".to_string(), destination: "/destination".to_string(), dependency: Some("DEPENDENCY".to_string()) } );
        let internal_data_type = data_type.to_internal();

        matches!(internal_data_type, DataTypes::DockerData(_));
        let data = internal_data_type.get_docker_data_ref();
        assert_eq!(data.name, Some("NAME".to_string()));
        assert_eq!(data.source, "/path".to_string());
        assert_eq!(data.destination, "/destination".to_string());
        assert_eq!(data.dependency, Some("DEPENDENCY".to_string()));
    }
}