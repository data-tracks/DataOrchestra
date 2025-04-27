mod tests {
    use da

    #[test]
    fn docker_parsing_test() {
        let string = 
        r#"
            {
                "name": "name-test"
                "network": "network-test",
                "options": 
                {
                    "key": "value"
                },
                "mount": "/path/to:/path/to",
                "publish_all": true,
                "image": "ubuntu",
                "files": {
                    "name": "name-test",
                    "path": "/path/to",
                    "destination": "/path/to",
                    "start": "setup.sh"
                }
            }
        "#;

        let json: Docker = serde_json::from_str(string);
    }
}
