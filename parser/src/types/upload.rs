use data_orchestra_engine::adapters::Rsync;
use super::amount::Amount;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::traits::ToInternal;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum UploadTypes {
    Ssh,
    Rsync(ExtRsync),
}

impl Default for UploadTypes {
    fn default() -> Self {
        UploadTypes::Rsync(ExtRsync::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ExtRsync {
    #[serde(default = "ExtRsync::default_checksum")]
    #[builder(default = "ExtRsync::default_checksum()")]
    checksum: bool,
    #[serde(default = "ExtRsync::default_compress")]
    #[builder(default = "ExtRsync::default_compress()")]
    compress: bool,
    #[serde(default = "ExtRsync::default_recursive")]
    #[builder(default = "ExtRsync::default_recursive()")]
    recursive: bool,
    #[serde(default = "ExtRsync::default_delete")]
    #[builder(default = "ExtRsync::default_delete()")]
    delete: bool,
    #[serde(default)]
    #[builder(default)]
    commands: Amount<String>,
}

impl ExtRsync {
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
}

impl ToInternal<Rsync> for ExtRsync {
    fn to_internal(self) -> Rsync {
        Rsync {
            user: "".to_string(),
            remote: "".to_string(),
            port: Some(5000),
            checksum: self.checksum,
            compress: self.compress,
            recursive: self.recursive,
            delete: self.delete,
            commands: self.commands.to_vec(),
        }
    }
}

impl Default for ExtRsync {
    fn default() -> Self {
        ExtRsync {
            checksum: ExtRsync::default_checksum(),
            compress: ExtRsync::default_compress(),
            recursive: ExtRsync::default_recursive(),
            delete: ExtRsync::default_delete(),
            commands: Amount::default(),
        }
    }
}
