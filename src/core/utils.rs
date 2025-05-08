use std::path::Path;
use super::adapters::command::command_func::spawn_command;
use super::adapters::Executor;
use super::types::Data;
use super::adapters::ssh::Ssh;

/// Start starting script on remote object
pub fn start_script(ssh: &Ssh, data: &Data) -> Result<(), String> {
    if data.start.ends_with(".sh") {
        ssh.exec(format!("sh {}", data.start))?;
    }
    else {
        ssh.exec(format!("nohup {}", data.start))?;
    }

    Ok(())
}

/// Start ansible on remote object
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

/// Upload data to remote object via ssh
///
/// Error is cascaded
pub fn upload(ssh: &Ssh, data: &Data) -> Result<(), String> {
    ssh.upload_directory(&data.path, &data.destination)?;

    if let Some(ref dependency) = data.dependency {
        let dependency_path= Path::new(dependency);
        ssh.exec("mkdir /scripts")?;
        ssh.upload_file(dependency_path, Path::new(&format!("/scripts/{}", &dependency_path.file_name().unwrap().to_str().unwrap())))?;
        ssh.exec(format!("sh /scripts/{}", dependency_path.file_name().unwrap().to_str().unwrap()))?;
    }
    Ok(())
}
