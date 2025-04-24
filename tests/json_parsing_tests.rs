mod tests {
    use core::panic;

    use DataOrchester::{docker::docker_struct::{Docker, DockerType, DockerTypeContainer}, types::{amount::Amount, config}};

    #[test]
    fn json_image_parse() {
        let json = r#"{ "image": "ubuntu" }"#;
        let parsed: DockerType = serde_json::from_str(json).unwrap();
        match parsed {
            DockerType::Image { image } => assert_eq!(image, "ubuntu"),
            _ => panic!()
        }
    }

    #[test]
    fn json_dockerfile_parse() { 
        let json = r#"{ "dockerfile": "/path/to/dockerfile" }"#;
        let parsed: DockerType = serde_json::from_str(json).unwrap();
        match parsed {
            DockerType::Dockerfile { dockerfile } => assert_eq!(dockerfile, "/path/to/dockerfile"),
            _ => panic!()
        }
    }

    #[test]
    fn json_compose_parse() {
        let json = r#"{ "compose": "/path/to/compose" }"#;
        let parsed: DockerType = serde_json::from_str(json).unwrap();
        match parsed {
            DockerType::Compose { compose } => assert_eq!(compose, "/path/to/compose"),
            _ => panic!()
        }
    }

    #[test]
    fn json_docker_image_parse() {
        let json = 
        r#"
            {
                "image": "ubuntu",
                "config": { "name": "test" }
            }
        "#;
        let parsed: Docker = serde_json::from_str(json).unwrap();
        match parsed.docker_type {
            DockerType::Image { image } => assert_eq!(image, "ubuntu"),
            _ => panic!()
        }

        match parsed.config {
            Some(DockerTypeContainer::Default(container)) => assert_eq!(container.name, Some(String::from("test"))),
            _ => panic!()
        }
    }

    #[test]
    fn json_docker_dockerfile_parse() {
        let json = 
        r#"
            {
                "dockerfile": "/path/to/dockerfile",
                "config": { "name": "test" }
            }
        "#;
        let parsed: Docker = serde_json::from_str(json).unwrap();
        match parsed.docker_type {
            DockerType::Dockerfile { dockerfile } => assert_eq!(dockerfile, "/path/to/dockerfile"),
            _ => panic!()
        }

        match parsed.config {
            Some(DockerTypeContainer::Default(container)) => assert_eq!(container.name, Some(String::from("test"))),
            _ => panic!()
        }
    }

    #[test]
    fn json_docker_compose_parse() {
        let json = 
        r#"
            {
                "compose": "/path/to/compose",
                "config": { "name": "test" }
            }
        "#;
        let parsed: Docker = serde_json::from_str(json).unwrap();
        dbg!(parsed);
        panic!();
        match parsed.docker_type {
            DockerType::Compose { compose } => assert_eq!(compose, "/path/to/compose"),
            _ => panic!()
        }

        match parsed.config {
            Some(DockerTypeContainer::Compose(config)) => {
                assert!(config.get_amount() == 1);
                match config {
                    Amount::Single(container) => assert_eq!(container.name, Some(String::from("test"))),
                    _ => panic!()
                }
            },
            _ => panic!()
        }
    }

    #[test]
    fn json_docker_compose_multiple_parse() {
        let json = 
        r#"
            {
                "compose": "/path/to/compose",
                "config": [ 
                    { "name": "test-1" },
                    { "name": "test-2" }
                ]
            }
        "#;
        let parsed: Docker = serde_json::from_str(json).unwrap();
        match parsed.docker_type {
            DockerType::Compose { compose } => assert_eq!(compose, "/path/to/compose"),
            _ => panic!()
        }
        match parsed.config {
            Some(DockerTypeContainer::Compose(config)) => {
                assert!(config.get_amount() == 2);

                match config {
                    Amount::Multiple(containers) => {
                        let container = containers.get(0).unwrap();
                        assert_eq!(container.name, Some(String::from("test-1")));

                        let container = containers.get(1).unwrap();
                        assert_eq!(container.name, Some(String::from("test-2")));
                    },
                    _ => panic!()
                }

                
            },
            _ => panic!()
        }
    }

}
