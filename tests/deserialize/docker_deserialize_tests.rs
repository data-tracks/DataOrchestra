#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::os::unix::raw::off_t;
    use serde_json::json;
    use data_orchestra::core::adapters::{BindPropagation, Mount, RestartTypes};
    use data_orchestra::interface::docker::ExtDocker;
    use data_orchestra::shared::Amount;

    pub fn get_docker(json: serde_json::Value) -> ExtDocker {
        let docker: ExtDocker = serde_json::from_value(json).expect("Unable to parse json to docker");
        docker
    }

    #[test]
    pub fn name_empty() {
        let json = json!({});

        let docker = get_docker(json);
        assert_eq!(docker.name, None);
    }

    #[test]
    pub fn name() {
        let json = json!(
        {
            "name": "rust"
        });

        let docker = get_docker(json);
        assert_eq!(docker.name, Some("rust".to_string()));
    }

    #[test]
    pub fn network_empty() {
        let json = json!({});

        let docker = get_docker(json);
        assert_eq!(docker.network, None);
    }

    #[test]
    pub fn network() {
        let json = json!(
        {
            "network": "rust"
        });

        let docker = get_docker(json);
        assert_eq!(docker.network, Some("rust".to_string()));
    }

    #[test]
    pub fn environment_empty() {
        let json = json!({});

        let docker = get_docker(json);
        assert_eq!(docker.environment, None);
    }

    #[test]
    pub fn environment() {
        let json = json!(
        {
            "environment": {
                "KEY": "VALUE"
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.environment, Some(HashMap::from([("KEY".to_string(), "VALUE".to_string())])));
    }

    #[test]
    pub fn environment_multiple() {
        let json = json!(
        {
            "environment": {
                "KEY_1": "VALUE_1",
                "KEY_2": "VALUE_2"
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.environment, Some(HashMap::from([("KEY_1".to_string(), "VALUE_1".to_string()), ("KEY_2".to_string(), "VALUE_2".to_string())])));
    }

    #[test]
    pub fn publish_all_empty() {
        let json = json!({});

        let docker = get_docker(json);
        assert_eq!(docker.publish_all, false);
    }

    #[test]
    pub fn publish_all() {
        let json = json!(
        {
            "publish_all": true
        });

        let docker = get_docker(json);
        dbg!(&docker);
        assert_eq!(docker.publish_all, true);
    }

    #[test]
    pub fn image() {
        let json = json!(
        {
            "image": "rust"
        });

        let docker = get_docker(json);
        assert_eq!(docker.image, Some("rust".to_string()));
        assert_eq!(docker.dockerfile, None);
        assert_eq!(docker.compose, None);
    }

    #[test]
    pub fn dockerfile() {
        let json = json!(
        {
            "dockerfile": "rust"
        });

        let docker = get_docker(json);
        assert_eq!(docker.image, None);
        assert_eq!(docker.dockerfile, Some("rust".to_string()));
        assert_eq!(docker.compose, None);
    }

    #[test]
    pub fn compose() {
        let json = json!(
        {
            "compose": "rust"
        });

        let docker = get_docker(json);
        assert_eq!(docker.image, None);
        assert_eq!(docker.dockerfile, None);
        assert_eq!(docker.compose, Some("rust".to_string()));
    }

    #[test]
    pub fn build_args_empty() {
        let json = json!({});

        let docker = get_docker(json);
        assert_eq!(docker.build_args, None);
    }

    #[test]
    pub fn build_args() {
        let json = json!(
        {
            "build_args": {
                "KEY": "VALUE"
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.build_args, Some(HashMap::from([("KEY".to_string(), "VALUE".to_string())])));
    }

    #[test]
    pub fn build_args_multiple() {
        let json = json!(
        {
            "build_args": {
                "KEY_1": "VALUE_1",
                "KEY_2": "VALUE_2"
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.build_args, Some(HashMap::from([("KEY_1".to_string(), "VALUE_1".to_string()), ("KEY_2".to_string(), "VALUE_2".to_string())])));
    }

    #[test]
    pub fn interpolation_variables_empty() {
        let json = json!({});

        let docker = get_docker(json);
        assert_eq!(docker.build_args, None);
    }

    #[test]
    pub fn interpolation_variables() {
        let json = json!(
        {
            "interpolation_variables": {
                "KEY": "VALUE"
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.interpolation_variables, Some(HashMap::from([("KEY".to_string(), "VALUE".to_string())])));
    }

    #[test]
    pub fn interpolation_variables_multiple() {
        let json = json!(
        {
            "interpolation_variables": {
                "KEY_1": "VALUE_1",
                "KEY_2": "VALUE_2"
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.interpolation_variables, Some(HashMap::from([("KEY_1".to_string(), "VALUE_1".to_string()), ("KEY_2".to_string(), "VALUE_2".to_string())])));
    }

    #[test]
    pub fn restart_empty() {
        let json = json!({});

        let docker = get_docker(json);
        assert_eq!(docker.restart, RestartTypes::No);
    }

    #[test]
    pub fn restart() {
        let json = json!(
        {
            "restart": "No"
        });

        let docker = get_docker(json);
        assert_eq!(docker.restart, RestartTypes::No);

        let json = json!(
        {
            "restart": {
                "OnFailure": 1
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.restart, RestartTypes::OnFailure(1));

        let json = json!(
        {
            "restart": "Always"
        });

        let docker = get_docker(json);
        assert_eq!(docker.restart, RestartTypes::Always);

        let json = json!(
        {
            "restart": "UnlessStopped"
        });

        let docker = get_docker(json);
        assert_eq!(docker.restart, RestartTypes::UnlessStopped);
    }

    #[test]
    pub fn volume_empty() {
        let json = json!({});

        let docker = get_docker(json);
        matches!(docker.volumes, Amount::None);
    }

    #[test]
    pub fn volume() {
        let json = json!(
        {
            "volumes": "/src:/dst"
        });

        let docker = get_docker(json);
        assert_eq!(docker.volumes.get_amount(), 1);
        assert_eq!(docker.volumes.to_vec(), Vec::from(["/src:/dst".to_string()]));
    }

    #[test]
    pub fn volume_multiple() {
        let json = json!(
        {
            "volumes": [ "/src:/dst", "/dst:/src" ]
        });

        let docker = get_docker(json);
        dbg!(&docker);
        assert_eq!(docker.volumes.get_amount(), 2);
        assert_eq!(docker.volumes.to_vec(), Vec::from(["/src:/dst".to_string(), "/dst:/src".to_string()]));
    }

    #[test]
    pub fn mount_empty() {
        let json = json!({});

        let docker = get_docker(json);
        matches!(docker.mounts, Amount::None);
    }

    #[test]
    pub fn mount() {
        let json = json!(
        {
            "mounts": {
                "src": "source",
                "dst": "destination",
                "read_only": true
            }
        });

        let docker = get_docker(json);
        assert_eq!(docker.mounts.get_amount(), 1);
        let mount = Mount::new("source", "destination", true, None);
        assert_eq!(docker.mounts.to_vec(), vec![mount]);
    }

    #[test]
    pub fn mount_multiple() {
        let json = json!(
        {
            "mounts": [
                {
                    "src": "source",
                    "dst": "destination",
                    "read_only": true
                },
                {
                    "src": "destination",
                    "dst": "source",
                    "bind_propagation": "Shared"
                }
            ]
        });

        let docker = get_docker(json);
        assert_eq!(docker.mounts.get_amount(), 2);
        let mount_1 = Mount::new("source", "destination", true, None);
        let mount_2 = Mount::new("destination", "source", false, Some(BindPropagation::Shared));
        assert_eq!(docker.mounts.to_vec(), vec![mount_1, mount_2]);
    }
}