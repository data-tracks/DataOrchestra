use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::{container::ContainerBuilder, ComposeGroupBuilder};
use crate::shared::{traits::ToInternal, Amount};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtDocker {
    // Name of container
    pub name: Option<String>,
    // Network of container
    pub network: Option<String>,
    // Additional options of container
    pub enviroment: Option<HashMap<String, String>>,
    // Mounts of container
    #[serde(default)]
    pub mount: Amount<String>,
    // Publish all ports
    #[serde(default)]
    pub publish_all: bool,
    // How container(s) are created
    pub image: Option<String>,
    pub dockerfile: Option<String>,
    pub build_args: Option<HashMap<String, String>>,
    pub compose: Option<String>,
    pub names: Option<Vec<String>>
}

impl ToInternal<ComposeGroupBuilder> for ExtDocker {
    fn to_internal(self) -> ComposeGroupBuilder {
        let mut builder = ComposeGroupBuilder::new();
        if let Some(compose) = self.compose {
            builder.set_compose(compose);
        }

        if let Some(names) = self.names {
            for name in names {
                builder.add_name(name);
            }
        }

        builder
    }
}

impl ToInternal<ContainerBuilder> for ExtDocker {
    fn to_internal(self) -> ContainerBuilder {
        let mut builder = ContainerBuilder::new();
        
        if let Some(name) = self.name {
            builder.set_name(name);
        }
        if let Some(image) = self.image {
            builder.set_image(image);
        }
        if let Some(dockerfile) = self.dockerfile {
            builder.set_dockerfile(dockerfile);
        }
        if let Some(build_args) = self.build_args {
            for (key, value) in build_args {
                builder.add_build_arg(key, value);
            }
        }
        if let Some(network) = self.network {
            builder.set_network(network);
        }
        if let Some(env) = self.enviroment {
            for (key, value) in env {
                builder.add_env_var(key, value);
            }
        }

        for mount in self.mount.to_vec() {
            builder.add_mount(mount);
        }

        builder.set_publish_all(self.publish_all); 

        builder
    }
}
