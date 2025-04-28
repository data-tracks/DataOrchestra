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

impl DockerSourceBuilder {
    pub fn new() -> Self {
        DockerSourceBuilder 
        { 
            dockersource: DockerSource 
            { 
                compose: None, 
                image: None, 
                dockerfile: None, 
                build_args: HashMap::new() 
            } 
        }
    }

    pub fn set_compose<T: Into<String>>(&mut self, compose: T) -> &mut Self {
        self.dockersource.compose = Some(compose.into());
        self
    }
    
    pub fn set_image<T: Into<String>>(&mut self, image: T) -> &mut Self {
        self.dockersource.image = Some(image.into());
        self
    }

    pub fn set_dockerfile<T: Into<String>>(&mut self, dockerfile: T) -> &mut Self {
        self.dockersource.dockerfile = Some(dockerfile.into());
        self
    }

    pub fn add_build_arg<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.dockersource.build_args.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> DockerSource {
        self.dockersource 
    }
}
