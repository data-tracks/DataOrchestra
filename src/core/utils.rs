use std::path::Path;

use serde::de::Error;

use super::{adapters::{command::command_func::spawn_command, ssh::Ssh}, data::Data};

pub fn start_script(ssh: &Ssh, data: &Data) {
    if data.start.ends_with(".sh") {
        ssh.exec(format!("sh {}", data.start));
    }
    else {
        ssh.exec(format!("{}", data.start));
    }
}

pub fn start_ansible(port: u16) -> Result<(), String>{
    let script_path = "scripts/ansible/ansible-setup.yml";
    if Path::new(&script_path).is_file() {
        let _ = spawn_command(&format!("ansible-playbook {} -e \"port={}\"", script_path, port)).wait();
    }
    else {
        return Err(format!("Unable to find file {}", script_path));
    }

    Ok(())
}
