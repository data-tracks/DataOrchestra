use std::{net::{IpAddr, TcpStream}, time::Duration};

use log::error;

/// Ping a remote node. 
///
/// Ping is done via the creation of a tcp stream to the ssh socket. A node is
/// considered operational if the one is able to create the tcp stream to the ssh socket.
///
/// # Example
///
/// ```
/// ```
pub fn ping_node(ip: &IpAddr) -> Result<(), String> {
    let socket_addr = format!("{}:{}", ip, 22);
    let rt = tokio::runtime::Runtime::new().unwrap(); 
    let _ = rt.block_on(async {
        let timeout = Duration::from_secs(5);
        TcpStream::connect_timeout(&socket_addr.parse().unwrap(), timeout).map_err(|err| err.to_string())
    })?;

    Ok(())
}

/// Ping a remote node. 
///
/// Ping is done via the creation of a tcp stream to the ssh socket. A node is
/// considered operational if the one is able to create the tcp stream to the ssh socket.
///
/// # Example
///
/// ```
/// ```
pub async fn async_ping_node(ip: &IpAddr) -> Result<(), String> {
    let socket_addr = format!("{}:{}", ip, 22);
    let timeout = Duration::from_secs(5);
    let _ = TcpStream::connect_timeout(&socket_addr.parse().unwrap(), timeout).map_err(|err| err.to_string())?;

    Ok(())
}
