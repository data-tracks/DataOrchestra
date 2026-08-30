use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::adapters::{ComposeBuilder, ContainerBuilder, Mount, RestartTypes};
use crate::shared::{Amount, traits::ToInternal};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExtDocker {
    Container(ExtContainer),
    Compose(ExtCompose),
    Dind(Dind),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtContainer {
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
    // Building arguments for dockerfile
    pub build_args: Option<HashMap<String, String>>,
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
    #[serde(default)]
    pub privileged: bool,
}

impl ToInternal<ContainerBuilder> for ExtContainer {
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
        builder.privileged(self.privileged);

        builder
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtCompose {
    pub file: String,
    // Environment variables
    pub interpolation_variables: Option<HashMap<String, String>>,
}

impl ToInternal<ComposeBuilder> for ExtCompose {
    fn to_internal(self) -> ComposeBuilder {
        let mut builder = ComposeBuilder::default();
        builder.file(self.file);

        if let Some(variables) = self.interpolation_variables {
            for (key, value) in variables {
                builder.interpolation_variable(key, value);
            }
        }

        builder
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dind {}

impl ToInternal<ContainerBuilder> for Dind {
    fn to_internal(self) -> ContainerBuilder {
        let mut builder = ContainerBuilder::default();
        builder
            .image("docker:dind")
            .environment("DOCKER_TLS_CERTDIR", "/certs")
            .privileged(true);
        builder
    }
}
