use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::core::adapters::{ComposeBuilder, ContainerBuilder};
use crate::shared::{traits::ToInternal, Amount};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtDocker {
    // Name of container
    pub name: Option<String>,
    // Network of container
    pub network: Option<String>,
    // Additional options of container
    pub environment: Option<HashMap<String, String>>,
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
    #[serde(default)]
    pub interpolation_variables: HashMap<String, String>
}

impl Default for ExtDocker {
    fn default() -> Self {
        ExtDocker 
        { 
            name: None, 
            network: None, 
            environment: None,
            mount: Amount::None, 
            publish_all: true, 
            image: None, 
            dockerfile: None, 
            build_args: None, 
            compose: None, 
            interpolation_variables: HashMap::new() 
        }
    }
}

impl ToInternal<ComposeBuilder> for ExtDocker {
    fn to_internal(self) -> ComposeBuilder {
        let mut builder = ComposeBuilder::default();
        if let Some(compose) = self.compose {
            builder.compose(compose);
        }

        for (key, value) in self.interpolation_variables {
            builder.interpolation_variable(key, value);
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

        for mount in self.mount.to_vec() {
            builder.volume(mount);
        }

        builder.publish_all(self.publish_all); 

        builder
    }
}
