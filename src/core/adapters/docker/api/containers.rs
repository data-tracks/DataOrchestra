use std::{thread, time::Duration};
use std::time;

use crate::core::adapters::{ContainerData, Runner};

/// Kill container with `name` on the location of the runner
///
/// # Examples
///
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = api::kill_container(&runner, "postgres");
/// ```
pub fn kill_container<T: Into<String>>(runner: &dyn Runner, name: T) -> Result<(), String> {
    let command = format!("docker container kill {}", name.into());
    runner.exec(command)?;

    Ok(())
}

/// Kill container all containers on the location of the runner
///
/// # Examples
///
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = api::kill_containers(&runner);
/// ```
pub fn kill_containers(runner: &dyn Runner) -> Result<(), String> {
    let command = "docker container kill $(docker container ls -a -q)";
    runner.exec(command.to_string())?;

    Ok(())
}

/// Delete all running containers running on the location of the runner
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = api::delete_containers(&runner);
/// ```
pub fn delete_containers(runner: &dyn Runner) -> Result<(), String> {
    let command = "docker rm $(docker container ls -a -q)";
    runner.exec(command.to_string())?;

    Ok(())
}

/// Delete specific container running on the location of the runner
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = api::delete_container("postgres", &runner);
/// ```
pub fn delete_container<T: Into<String>>(name: T, runner: &dyn Runner) -> Result<(), String> {
    let command = format!("docker rm {}", name.into());
    runner.exec(command)?;

    Ok(())
}

/// Delete all running containers running on the location of the runner
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = api::stop_containers(&runner);
/// ```
pub fn stop_containers(runner: &dyn Runner) -> Result<(), String> {
    let command = "docker stop $(docker container ls -a -q)";
    runner.exec(command.to_string())?;

    Ok(())
}

/// Stop specific running container running on the location of the runner
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = api::stop_container("postgres", &runner);
/// ```
pub fn stop_container<T: Into<String>>(name: T, runner: &dyn Runner) -> Result<(), String> {
    let command = format!("docker stop {}", name.into());
    runner.exec(command)?;

    Ok(())
}

/// Check if container is running by continously polling its status every second until `timeout`
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::poll_container;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = poll_container("postgres", 30, &runner);
/// ```
pub fn poll_container<T: Into<String>>(name: T, timout: u64, runner: &dyn Runner) -> Result<(), String> {
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

/// Get all container names running on location of runner
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::local::Local;
/// use data_orchestra::core::adapters::get_container_names;
/// use data_orchestra::core::adapters::traits::Runner;
///
/// let runner = Local::new();
/// let result = get_container_names(&runner);
/// ```
pub fn get_container_names(runner: &dyn Runner) -> Result<Vec<String>, String> {
    let command = "docker container ls -a --format {{.Names}}";
    let output = runner.exec(command.to_string())?;

    let containers: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(containers)
}


/// Get metadata of all containers running on location of runner
pub fn get_container_data(runner: &dyn Runner) -> Result<Vec<ContainerData>, String> {
    let command = "docker container ls --format {{.ID}}";
    let result = runner.exec(command.to_string())?;

    let mut containers = Vec::new();
    for id in result.split("\n") {
        let command = format!("docker container ls -f id={id} --format json");
        let result = runner.exec(command)?;
        if !result.is_empty() {
            let data: ContainerData = serde_json::from_str(result.as_str()).expect("Unable to parse json to struct");
            containers.push(data); 
        }
    }

    Ok(containers)
}
