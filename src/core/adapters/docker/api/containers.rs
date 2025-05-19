use std::{thread, time::Duration};
use std::time;

use crate::core::adapters::Runner;

pub fn delete_containers(runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = "docker rm $(docker container ls -a -q)";
    runner.exec(command.to_string())?;

    Ok(())
}

pub fn delete_container<T: Into<String>>(name: T, runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = format!("docker rm {}", name.into());
    runner.exec(command.to_string())?;

    Ok(())
}

pub fn stop_containers(runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = "docker stop $(docker container ls -a -q)";
    runner.exec(command.to_string())?;

    Ok(())
}

pub fn stop_container<T: Into<String>>(name: T, runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = format!("docker stop {}", name.into());
    runner.exec(command.to_string())?;

    Ok(())
}

pub fn poll_container<T: Into<String>>(name: T, timout: u64, runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let name = name.into();
    let start = time::Instant::now();
    let timout = Duration::from_secs(timout);

    let command = format!("docker inspect {} -f {{{{.State.Status}}}}", name);
    
    loop {
        let result: String;
        result = runner.exec(command.clone())?;
        

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

pub fn get_container_names(runner: &Box<dyn Runner + Send>) -> Result<Vec<String>, String> {
    let command = "docker container ls --format {{.Names}}";
    let output: String;
    output = runner.exec(command.to_string())?;

    let containers: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(containers)
}
