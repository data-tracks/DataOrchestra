use std::net::IpAddr;

use super::command::command_func::output_command;

pub fn ping(ip: &IpAddr) -> Result<(), String> {
    let result = output_command(format!("ping {} -q -c5 -w5 | awk -F '/' 'END{{ print (/^rrt/? \"OK\":\"FAIL\") }}'" , ip));
    
    match result.as_str() {
        "OK" => Ok(()),
        _ => Err("Unable to ping remote ip".to_string())
    }
}
