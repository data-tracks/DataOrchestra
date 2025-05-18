use std::{thread, time::Duration};
use std::time;

use crate::core::adapters::{command_func::output_command, Runner};

pub fn delete_containers(runner: Option<&Box<dyn Runner + Send>>) -> Result<(), String> {
    let command = "docker rm $(docker container ls -a -q)";
    if let Some(runner) = runner {
        runner.exec(command.to_string())?;
    }
    else {
        output_command(command);
    }

    Ok(())
}

pub fn stop_containers(runner: Option<&Box<dyn Runner + Send>>) -> Result<(), String> {
    let command = "docker stop $(docker container ls -a -q)";
    if let Some(runner) = runner {
        runner.exec(command.to_string())?;
    }
    else {
        output_command(command);
    }

    Ok(())
}

pub fn poll_container<T: Into<String>>(name: T, timout: u64, runner: Option<&Box<dyn Runner + Send>>) -> Result<(), String> {
    let name = name.into();
    let start = time::Instant::now();
    let timout = Duration::from_secs(timout);

    let command = format!("docker inspect {} -f {{{{.State.Status}}}}", name);
    
    loop {

        let result: String;
        if let Some(runner) = runner {
            result = runner.exec(command.clone())?;
        }
        else {
            result = output_command(&command);
        }

        let result = result.replace("\n", "");
        let result = result.trim();
        if result.eq("running") {
            return Ok(());
        }

        if start.elapsed() >= timout {
            return Err("Docker not running".to_string());
        }

        thread::sleep(Duration::from_secs(1))
    }
}

pub fn get_container_names(runner: Option<&Box<dyn Runner + Send>>) -> Result<Vec<String>, String> {
    let command = "docker container ls --format {{.Names}}";
    let output: String;

    if let Some(runner) = runner {
        output = runner.exec(command.to_string())?;
    }
    else {
        output = output_command(command);
    }

    let containers: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(containers)
}
