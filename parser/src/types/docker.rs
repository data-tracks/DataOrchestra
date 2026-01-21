use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use data_orchestra_engine::adapters::{ComposeBuilder, ContainerBuilder, Mount, RestartTypes};
use crate::amount::Amount;
use crate::traits::ToInternal;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtDocker {
    // Name of container
    pub name: Option<String>,
    // Network of container
    pub network: Option<String>,
    // Environment variables of container
    pub environment: Option<HashMap<String, String>>,
    // Publish all ports
    #[serde(default)]
    pub publish_all: bool,
    // How container(s) are created
    pub image: Option<String>,
    pub dockerfile: Option<String>,
    pub compose: Option<String>,
    // Building arguments for dockerfile
    pub build_args: Option<HashMap<String, String>>,
    // Interpolation variables (Environment variables) for compose file
    pub interpolation_variables: Option<HashMap<String, String>>,
    // Container restart policy
    #[serde(default)]
    pub restart: RestartTypes,
    // Volumes attached to container
    #[serde(default)]
    pub volumes: Amount<String>,
    // Mounts attached to container
    #[serde(default)]
    pub mounts: Amount<Mount>,
    #[serde(default)]
    pub publish: Amount<u16>,
    #[serde(default)]
    pub publish_map: Amount<String>,
}

impl Default for ExtDocker {
    fn default() -> Self {
        ExtDocker {
            name: None,
            network: None,
            environment: None,
            volumes: Amount::None,
            publish_all: false,
            image: None,
            dockerfile: None,
            build_args: None,
            compose: None,
            interpolation_variables: None,
            restart: RestartTypes::default(),
            mounts: Amount::None,
            publish: Amount::None,
            publish_map: Amount::None,
        }
    }
}

impl ToInternal<ComposeBuilder> for ExtDocker {
    fn to_internal(self) -> ComposeBuilder {
        let mut builder = ComposeBuilder::default();
        if let Some(compose) = self.compose {
            builder.compose(compose);
        }

        if let Some(variables) = self.interpolation_variables {
            for (key, value) in variables {
                builder.interpolation_variable(key, value);
            }
        }

        builder
    }
}

impl ToInternal<ContainerBuilder> for ExtDocker {
    fn to_internal(self) -> ContainerBuilder {
        let mut builder = ContainerBuilder::default();

        if let Some(name) = self.name {
            builder.name(name);
        }
        if let Some(image) = self.image {
            builder.image(image);
        }
        if let Some(dockerfile) = self.dockerfile {
            builder.dockerfile(dockerfile);
        }
        if let Some(build_args) = self.build_args {
            for (key, value) in build_args {
                builder.build_arg(key, value);
            }
        }
        if let Some(network) = self.network {
            builder.network(network);
        }
        if let Some(env) = self.environment {
            for (key, value) in env {
                builder.environment(key, value);
            }
        }

        for volume in self.volumes {
            builder.volume(volume);
        }

        for mount in self.mounts {
            builder.mount(mount);
        }

        for publish in self.publish {
            builder.publish(publish);
        }

        for publish_map in self.publish_map {
            let split = publish_map.split_once(":");
            if let Some(split) = split {
                let (ext, int) = split;
                if ext.is_empty() || int.is_empty() || int.contains(":") {
                    panic!("Invalid port mapping {}", publish_map);
                }

                let ext: u16 = ext.parse().unwrap();
                let int: u16 = int.parse().unwrap();

                builder.publish_map(ext, int);
            }
        }

        builder.publish_all(self.publish_all);
        builder.restart(self.restart);

        builder
    }
}
