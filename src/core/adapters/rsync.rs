use std::path::Path;
use derive_builder::Builder;
use crate::core::adapters::{Local, Runner, Uploader, UploaderError};

/// Rsync type. Allows for the interaction with the rsync CLI. Uploading of files to remote location via SSH.
#[derive(Debug, Builder)]
pub struct Rsync {
    // Remote SSH user name
    user: String,
    // Remote address
    remote: String,
    #[builder(default)]
    // SSH port
    port: Option<u16>,
    #[builder(default = "default_checksum()")]
    checksum: bool,
    #[builder(default = "default_compress()")]
    compress: bool,
    #[builder(default = "default_recursive()")]
    recursive: bool
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

impl Uploader for Rsync
{
    fn upload_file(&self, src: &Path, dst: &Path) -> Result<(), UploaderError> {
        if !src.is_file() {
            return Err(UploaderError::InvalidFile(src.display().to_string()))
        }

        if let Ok(exists) = src.try_exists() && !exists {
            return Err(UploaderError::NoSuchFile(src.display().to_string()));
        }

        self.upload(src, dst)
            .map_err(UploaderError::UnableToUpload)?;
        
        Ok(())
    }

    fn upload_directory(&self, src: &Path, dst: &Path) -> Result<(), UploaderError> {
        if !src.is_dir() {
            return Err(UploaderError::InvalidDirectory(src.display().to_string()))
        }

        if let Ok(exists) = src.try_exists() && !exists {
            return Err(UploaderError::NoSuchDirectory(src.display().to_string()));
        }

        self.upload(src, dst)
            .map_err(UploaderError::UnableToUpload)?;

        Ok(())
    }
}