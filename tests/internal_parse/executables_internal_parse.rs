#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use data_orchestra::core::adapters::TmuxBuilder;
    use data_orchestra::core::types::{DataTypes, Executables};
    use data_orchestra::interface::execute::{ExtExecutables, ExtScript, ExtTmux};
    use data_orchestra::interface::location::Location;
    use data_orchestra::shared::ToInternal;

    #[test]
    pub fn script_node() {
        let script = ExtScript { location: Location::Node, name: Some("NAME".to_string()), destination: "/destination".to_string() };
        let script = ExtExecutables::Script(script);
        let (internal_script, data_type) = script.to_internal();

        matches!(internal_script, Executables::Script(_));

        let script = internal_script.get_script();

        assert_eq!(script.path, "/destination");
        assert_eq!(script.name, Some("NAME".to_string()));

        matches!(data_type, None);
    }

    #[test]
    pub fn script_docker() {
        let script = ExtScript { location: Location::Container, name: Some("NAME".to_string()), destination: "/destination".to_string() };
        let script = ExtExecutables::Script(script);
        let (internal_script, data_type) = script.to_internal();

        matches!(internal_script, Executables::Script(_));

        let script = internal_script.get_script();

        assert_eq!(script.path, "/destination");
        assert_eq!(script.name, Some("NAME".to_string()));

        matches!(data_type, None);
    }

    #[test]
    pub fn tmux_node() {
        let tmux_builder = TmuxBuilder::default()
            .session("")
            .command("pwd")
            .to_owned();

        let tmux = ExtTmux { location: Location::Node, name: Some("NAME".to_string()), destination: "/destination".to_string(), tmux: tmux_builder };
        let script = ExtExecutables::Tmux(tmux);
        let (internal_script, data_type) = script.to_internal();

        matches!(internal_script, Executables::Script(_));

        let script = internal_script.get_script();

        assert_eq!(script.path, "/destination");
        assert_eq!(script.name, Some("NAME".to_string()));

        matches!(data_type, Some(DataTypes::VolatileNodeData(_)));

        let volatile = data_type.unwrap();
        let volatile = volatile.get_volatile_node_data_ref();

        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.dst, PathBuf::from("/destination"));
        assert!(volatile.content.contains("pwd"));
    }

    #[test]
    pub fn tmux_docker() {
        let tmux_builder = TmuxBuilder::default()
            .session("")
            .command("pwd")
            .to_owned();

        let tmux = ExtTmux { location: Location::Container, name: Some("NAME".to_string()), destination: "/destination".to_string(), tmux: tmux_builder };
        let script = ExtExecutables::Tmux(tmux);
        let (internal_script, data_type) = script.to_internal();

        matches!(internal_script, Executables::Script(_));

        let script = internal_script.get_script();

        assert_eq!(script.path, "/destination");
        assert_eq!(script.name, Some("NAME".to_string()));

        matches!(data_type, Some(DataTypes::VolatileDockerData(_)));

        let volatile = data_type.unwrap();
        let volatile = volatile.get_volatile_docker_data_ref();

        assert_eq!(volatile.name, Some("NAME".to_string()));
        assert_eq!(volatile.dst, PathBuf::from("/destination"));
        assert!(volatile.content.contains("pwd"));
    }
}