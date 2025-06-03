use std::{net::{IpAddr, TcpStream}, time::Duration};

pub fn ping_node(ip: &IpAddr) -> Result<(), String> {
    dbg!(&ip);
    let socket_addr = format!("{}:{}", ip, 22);
    let rt = tokio::runtime::Runtime::new().unwrap(); 
    let _ = rt.block_on(async {
        let timeout = Duration::from_secs(5);
        TcpStream::connect_timeout(&socket_addr.parse().unwrap(), timeout).map_err(|err| err.to_string())
    });

    Ok(())
}
