use std::collections::HashMap;

#[derive(Debug)]
pub struct DockerSource {
    pub compose: Option<String>,
    pub image: Option<String>,
    pub dockerfile: Option<String>,
    pub build_args: HashMap<String, String>,
}

#[derive(Debug)]
pub struct DockerSourceBuilder {
    dockersource: DockerSource
}

impl Default for DockerSource {
    fn default() -> Self {
        DockerSource { compose: None, image: None, dockerfile: None, build_args: HashMap::new() }
    }
}

impl Default for DockerSourceBuilder {
    fn default() -> Self {
        DockerSourceBuilder 
        { 
            dockersource: DockerSource::default() 
        }
    }
}

impl DockerSourceBuilder {
    pub fn new() -> Self {
        DockerSourceBuilder 
        { 
            dockersource: DockerSource::default()        
        }
    }

    pub fn compose<T: Into<String>>(mut self, compose: T) -> Self {
        self.dockersource.compose = Some(compose.into());
        self
    }

    pub fn compose_mut<T: Into<String>>(&mut self, compose: T) -> &mut Self {
        self.dockersource.compose = Some(compose.into());
        self
    }

    pub fn image<T: Into<String>>(mut self, image: T) -> Self {
        self.dockersource.image = Some(image.into());
        self
    }
    
    pub fn image_mut<T: Into<String>>(&mut self, image: T) -> &mut Self {
        self.dockersource.image = Some(image.into());
        self
    }

    pub fn dockerfile<T: Into<String>>(mut self, dockerfile: T) -> Self {
        self.dockersource.dockerfile = Some(dockerfile.into());
        self
    }

    pub fn dockerfile_mut<T: Into<String>>(&mut self, dockerfile: T) -> &mut Self {
        self.dockersource.dockerfile = Some(dockerfile.into());
        self
    }

    pub fn build_arg<T: Into<String>, S: Into<String>>(mut self, key: T, value: S) -> Self {
        self.dockersource.build_args.insert(key.into(), value.into());
        self
    }

    pub fn build_arg_mut<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.dockersource.build_args.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> DockerSource {
        self.dockersource 
    }
}
