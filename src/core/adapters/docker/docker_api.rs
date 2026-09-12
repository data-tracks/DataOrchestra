use std::net::{IpAddr, Ipv4Addr};

use bollard::{
    API_DEFAULT_VERSION, ClientVersion, Docker, errors::Error,
    query_parameters::BuildImageOptionsBuilder,
};

use crate::shared::Address;

#[derive(Debug)]
pub enum ConnectionType {
    Socket,
    Http,
    Ssl,
}

impl Default for ConnectionType {
    fn default() -> Self {
        Self::Socket
    }
}

#[derive(Debug)]
pub struct DockerAPI {
    address: Option<Address>,
    connection_type: ConnectionType,
    connection: Option<Docker>,
    version: ClientVersion,
}

impl Default for DockerAPI {
    fn default() -> Self {
        Self {
            address: None,
            connection_type: ConnectionType::default(),
            connection: None,
            version: API_DEFAULT_VERSION.clone(),
        }
    }
}

impl DockerAPI {
    pub fn default_address() -> Address {
        Address::new(Self::default_host(), Self::default_port())
    }

    pub fn default_host() -> IpAddr {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }

    pub fn default_port() -> u16 {
        2375
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        match self.connection_type {
            ConnectionType::Http => {
                if let Some(address) = self.address.as_ref() {
                    self.connection = Some(Docker::connect_with_http(
                        &address.as_string(),
                        timeout,
                        client_version,
                    ))
                } else {
                    self.connection = Some(Docker::connect_with_http_defaults()?);
                }
            }
            ConnectionType::Socket => {
                self.connection = Some(Docker::connect_with_socket_defaults()?);
            }
            ConnectionType::Ssl => {
                todo!();
            }
        }

        Ok(())
    }

    pub fn build_image(&self, options: BuildImageOptions) {
        if let Some(connection) = self.connection.as_ref() {
            connection.build_image(options, None, None);
        }
    }

    pub fn create(&self) {
        if let Some(connection) = self.connection.as_ref() {}
    }
}
