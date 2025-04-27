use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {
}

impl Redis {
    pub fn new() -> Redis {
        Redis {  }
    }

    pub fn setup_container(&self, docker: &mut Data) {
        docker.set_image("redis");
    }
}
