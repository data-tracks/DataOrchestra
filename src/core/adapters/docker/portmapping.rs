#[derive(Debug)]
pub struct PortMapping {
    host: u16,
    internal: u16
}

impl PortMapping {
    pub fn new(host: u16, internal: u16) -> Self {
        PortMapping { host, internal }
    }

    pub fn get_host(&self) -> u16 {
        self.host.clone()
    }

    pub fn get_internal(&self) -> u16 {
        self.internal.clone()
    }
}
