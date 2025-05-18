use std::collections::HashMap;
use std::path::Path;
use super::adapters::command::command_func::spawn_command;
use super::adapters::docker::{Container, DockerManager};
use super::adapters::{ContainerType, Runner, Uploader};
use super::types::Data;
use super::adapters::ssh::Ssh;

/// Start starting script on remote object
pub fn start_script(ssh: &Ssh, data: &Data) -> Result<(), String> {
    if data.start.ends_with(".sh") {
        ssh.exec(format!("sh {}", data.start))?;
    }
    else {
        ssh.exec(format!("{}", data.start))?;
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

/// Combine containers managed by manager with [`Data`] to [`Iterator`] 
///
/// If `data` is empty, returns an empty iterator.
///
/// If there is only on container, all entries in `data` get combined with that
/// specific `container`
pub fn iter_combine_data<'a>(manager: &'a DockerManager, data: &'a Vec<Data>) -> impl Iterator<Item = (&'a Container, &'a Data)> {
    let mut vec_container = Vec::<&Container>::new();
    let mut vec_data = Vec::<&Data>::new();

    // Early return for when data contains nothing
    if data.len() == 0 {
        return vec_container.into_iter().zip(vec_data);
    }

    if manager.amount() == 1 {
        let container = manager.get_container();
        if let Some(container) = container {
            for d in data.iter() {
                vec_container.push(container);
                vec_data.push(d);
            }
        }
    }
    else 
    {
        let mut mapped_all_containers = HashMap::<&String, &Container>::new();
        for (_, item) in manager.containers.iter() {
            match item {
                ContainerType::Compose(compose) => {
                    for container in compose.containers.iter() {
                        mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
                    } 
                }
                ContainerType::Container(container) => {
                    mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
                }
            }
        }

        for d in data {
            if let Some(ref mut container) = mapped_all_containers.get(&d.name) {
                vec_container.push(container);
                vec_data.push(d);
            }
        };
    }

    vec_container.into_iter().zip(vec_data)
}

pub fn upload_data(manager: &DockerManager, vec_data: &Vec<Data>) -> Result<(), String> {
    for (container, data) in iter_combine_data(manager, vec_data) {
        if let Some(ref ssh) = container.ssh {
            let path = Path::new(&data.path);
            if path.is_dir() {
                ssh.upload_directory(path, &data.destination)?;
            }
            else if path.is_file() {
                ssh.upload_file(path, &data.destination)?;
            }

            if let Some(ref dependency) = data.dependency {
                let dependency_path= Path::new(dependency);
                ssh.exec("mkdir /scripts".to_string())?;
                ssh.upload_file(dependency_path, Path::new(&format!("/scripts/{}", &dependency_path.file_name().unwrap().to_str().unwrap())))?;
                ssh.exec(format!("sh /scripts/{}", dependency_path.file_name().unwrap().to_str().unwrap()))?;
            }
        } 
        else {
            panic!("Ssh client unavailable for container {} even though upload data was specified", container.config.name.as_ref().unwrap());
        }
    }

    Ok(())
}
