use crate::types::address::Address;

use super::object_struct::Object;

impl Object {
    pub fn get_remote_connection(&self) -> Address {
        if let Some(ref docker) = self.docker {
            return docker.address.clone()
        }
        else if let Some(ref node) = self.node {
            if let Some(ref address) = node.address {
                return address.clone();            
            }
        }
    
        panic!("No available remote connection. Please specify a remote node connection if needed.");
    }
}
