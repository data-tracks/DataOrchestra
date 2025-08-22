use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::core::adapters::{Rsync, Uploader};
use crate::shared::ToInternal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UploadTypes {
    Ssh,
    Rsync(ExtRsync)
}

impl Default for UploadTypes {
    fn default() -> Self {
        UploadTypes::Rsync(ExtRsync::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ExtRsync {
    #[serde(default = "default_checksum")]
    #[builder(default = "default_checksum()")]
    checksum: bool,
    #[serde(default = "default_compress")]
    #[builder(default = "default_compress()")]
    compress: bool,
    #[serde(default = "default_recursive")]
    #[builder(default = "default_recursive()")]
    recursive: bool,
    #[serde(default = "default_delete")]
    #[builder(default = "default_delete()")]
    delete: bool
}

impl ToInternal<Rsync> for ExtRsync {
    fn to_internal(self) -> Rsync {
        Rsync
        {
            user: "".to_string(),
            remote: "".to_string(),
            port: Some(5000),
            checksum: self.checksum,
            compress: self.compress,
            recursive: self.recursive,
            delete: self.delete
        }
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

pub fn default_delete() -> bool {
    true
}

impl Default for ExtRsync {
    fn default() -> Self {
        ExtRsync 
        {
            checksum: default_checksum(), 
            compress: default_compress(),
            recursive: default_recursive(),
            delete: default_delete()
        }
    }
}