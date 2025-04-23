use std::{path::Path, thread};

use crate::{common::common_trait::Start, types::address::Address};

use super::object_struct::Object;

use std::thread::JoinHandle;

impl Start<()> for Object {
    fn start(self) -> JoinHandle<()> {
        thread::Builder::new().name("object".to_string()).spawn(move || {
        
        }).unwrap()
    }
} 

impl Object {
    pub fn upload_data(&self) -> String {
        let mut upload_directory = String::from("/");
        if let Some(ref data) = self.data {
            let upload = self.ssh.as_ref().unwrap().upload_directory(&Path::new(&data), &Path::new("/"));
            if let Ok(dir) = upload {
                upload_directory = dir;
            }
        }

        upload_directory
    }

    
}
