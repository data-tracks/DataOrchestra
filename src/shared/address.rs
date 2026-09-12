use std::net::IpAddr;

#[derive(Debug, Clone, Copy)]
pub struct Address {
    host: IpAddr,
    port: u16,
}

impl Address {
    pub fn new(host: IpAddr, port: u16) -> Self {
        Address { host, port }
    }

    pub fn as_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
