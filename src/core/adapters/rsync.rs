use std::path::Path;
use derive_builder::Builder;
use crate::core::adapters::{Local, Runner, Ssh, Uploader, UploaderError};

#[derive(Debug, Builder)]
pub struct Rsync {
    user: String,
    remote: String,
    #[builder(default)]
    port: Option<u16>,
    #[builder(default = "default_checksum()")]
    checksum: bool,
    #[builder(default = "default_compress()")]
    compress: bool,
    #[builder(default = "default_recursive()")]
    recursive: bool
}

impl Rsync {
    fn upload(&self, src: &Path, dst: &Path) -> Result<String, String> 
    {
        let runner = Local::new();
        
        let src = src.display();
        let dst = dst.display();

        let mut parameters = Vec::new();

        if self.checksum {
            parameters.push("--checksum".to_string());
        }

        if self.compress {
            parameters.push("--compress".to_string())
        }

        if self.recursive {
            parameters.push("--recursive".to_string());
        }

        if let Some(port) = self.port {
            parameters.push(format!("--port={port}"));
        }

        let parameters = parameters.join(" ");

        let command = format!("rsync {parameters} {src} {}@{}:{dst}",
                              self.user,
                              self.remote,
        );
        
        runner.exec(command)
            .map_err(|err| err.into())
    }
}

pub fn default_checksum() -> bool {
    true
}

pub fn default_compress() -> bool {
    true
}

pub fn default_recursive() -> bool {
    true
}

impl<T, S> Uploader<T, S> for Rsync where
    T: AsRef<Path>,
    S: AsRef<Path>
{
    fn upload_file(&self, src: T, dst: S) -> Result<(), UploaderError> {
        let src = src.as_ref();
        let dst = dst.as_ref();

        if !src.is_file() {
            return Err(UploaderError::InvalidFile(src.display().to_string()))
        }

        if !src.exists() {
            return Err(UploaderError::NoSuchFile(src.display().to_string()));
        }

        let result = self.upload(src, dst);
        if let Err(error) = result {
            return Err(UploaderError::UnableToUpload(error))
        }
        
        Ok(())
    }

    fn upload_directory(&self, src: T, dst: S) -> Result<(), UploaderError> {
        let src = src.as_ref();
        let dst = dst.as_ref();

        if !src.is_dir() {
            return Err(UploaderError::InvalidDirectory(src.display().to_string()))
        }

        if !src.exists() {
            return Err(UploaderError::NoSuchDirectory(src.display().to_string()));
        }

        let result = self.upload(src, dst);
        if let Err(error) = result {
            return Err(UploaderError::UnableToUpload(error))
        }

        Ok(())
    }
}