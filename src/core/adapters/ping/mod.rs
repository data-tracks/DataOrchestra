use std::net::IpAddr;

use super::command::command_func::output_command;

pub fn ping(ip: &IpAddr) -> Result<(), String> {
    let result = output_command(format!("ping -c1 -w5 {} >/dev/null && echo \"true\"", ip));
    
    match result.as_str().trim() {
        "true" => Ok(()),
        _ => Err("Unable to ping remote ip".to_string())
    }
}
