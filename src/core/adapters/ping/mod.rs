use std::{net::{IpAddr, TcpStream}, time::Duration};

/// Ping a remote node. 
///
/// Ping is done via the creation of a tcp stream to the ssh socket. A node is
/// considered operational if the one is able to create the tcp stream to the ssh socket.
pub fn ping_node(ip: &IpAddr) -> Result<(), String> {
    ping(ip, 22) 
}

/// Ping a remote node. 
///
/// Ping is done via the creation of a tcp stream to the ssh socket. A node is
/// considered operational if the one is able to create the tcp stream to the ssh socket.
pub async fn async_ping_node(ip: &IpAddr) -> Result<(), String> {
    async_ping(ip, 22).await
}

/// Ping a remote host. 
///
/// Ping is done via the creation of a tcp stream to the given port socket. A node is
/// considered operational if the one is able to create the tcp stream to the port socket.
pub fn ping(ip: &IpAddr, port: u16) -> Result<(), String> {
    let socket_addr = format!("{}:{}", ip, port);
    let rt = tokio::runtime::Runtime::new().unwrap(); 
    let _ = rt.block_on(async {
        let timeout = Duration::from_secs(5);
        TcpStream::connect_timeout(&socket_addr.parse().unwrap(), timeout).map_err(|err| err.to_string())
    })?;

    Ok(())
}

/// Ping a remote host. 
///
/// Ping is done via the creation of a tcp stream to the given port socket. A node is
/// considered operational if the one is able to create the tcp stream to the port socket.
pub async fn async_ping(ip: &IpAddr, port: u16) -> Result<(), String> {
    let socket_addr = format!("{}:{}", ip, port);
    let timeout = Duration::from_secs(5);
    let _ = TcpStream::connect_timeout(&socket_addr.parse().unwrap(), timeout).map_err(|err| err.to_string())?;

    Ok(())
}
