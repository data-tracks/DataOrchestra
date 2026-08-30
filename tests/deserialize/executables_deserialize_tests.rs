#[cfg(test)]
mod tests {
    use data_orchestra::interface::execute::{ExtExecutables, ExtScript};
    use data_orchestra::interface::location::Location;
    use serde_json::json;

    pub fn get_executable(json: serde_json::Value) -> ExtExecutables {
        let executable: ExtExecutables =
            serde_json::from_value(json).expect("Unable to parse json to Executables");
        executable
    }

    #[test]
    #[should_panic]
    pub fn no_type() {
        let json = json!(
        {
           "name": "NAME"
        });

        let _ = get_executable(json);
    }

    #[test]
    #[should_panic]
    pub fn script_no_path() {
        let json = json!(
        {
            "type": "script",
            "location": "node",
            "name": "script",
        });

        let _ = get_executable(json);
    }

    #[test]
    #[should_panic]
    pub fn script() {
        let json = json!(
        {
            "type": "script",
            "location": "node",
            "name": "script",
            "path": "/path"
        });

        let script = get_executable(json);
        let script_struct = ExtScript {
            location: Location::Node,
            name: Some("name".to_string()),
            destination: "/path".to_string(),
        };
        assert_eq!(script.get_script_ref(), &script_struct);
    }

    #[test]
    #[should_panic]
    pub fn tmux_no_path() {
        let json = json!(
        {
            "type": "tmux",
            "location": "node",
            "name": "tmux",
        });

        let _ = get_executable(json);
    }

    #[test]
    #[should_panic]
    pub fn tmux() {
        let json = json!(
        {
            "type": "tmux",
            "location": "node",
            "name": "tmux",
            "session": "tmux",
            "commands": [ "pwd", "ls" ]
        });

        let script = get_executable(json);
        let tmux = script.get_tmux_ref();
        assert_eq!(tmux.name, Some("tmux".to_string()));
        assert_eq!(tmux.location, Location::Node);
    }

    //TODO: Add tests for tmux commands etc.
}
