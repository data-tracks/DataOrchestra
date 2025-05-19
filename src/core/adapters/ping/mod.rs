use std::net::IpAddr;

use super::{Local, Runner};

pub fn ping(ip: &IpAddr) -> Result<(), String> {
    let local = Local::new();
    let result = local.exec(format!("ping -c1 -w5 {} >/dev/null && echo \"true\"", ip))?;
   
    match result.as_str().trim() {
        "true" => Ok(()),
        _ => Err("Unable to ping remote ip".to_string())
    }
}
