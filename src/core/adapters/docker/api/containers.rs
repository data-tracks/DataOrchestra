use std::{thread, time::Duration};
use std::time;

use crate::core::adapters::{ContainerData, Executor};

/// Kill container with `name` on the location of the executor
///
/// # Examples
///
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = api::kill_container(&executor, "postgres");
/// ```
pub fn kill_container<T: Into<String>>(executor: &dyn Executor, name: T) -> Result<(), String> {
    let command = format!("docker container kill {}", name.into());
    executor.exec(command)?;

    Ok(())
}

/// Kill container all containers on the location of the executor
///
/// # Examples
///
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = api::kill_containers(&executor);
/// ```
pub fn kill_containers(executor: &dyn Executor) -> Result<(), String> {
    let containers = get_container_names(executor)?;
    for container in containers {
        let command = format!("docker container kill {container}");
        executor.exec(command)?;
    }

    Ok(())
}

/// Delete all running containers running on the location of the executor
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = api::delete_containers(&executor);
/// ```
pub fn delete_containers(executor: &dyn Executor) -> Result<(), String> {
    let containers = get_container_names(executor)?;
    for container in containers {
        let command = format!("docker rm {container}");
        executor.exec(command)?;
    }

    Ok(())
}

/// Delete specific container running on the location of the executor
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = api::delete_container("postgres", &executor);
/// ```
pub fn delete_container<T: Into<String>>(name: T, executor: &dyn Executor) -> Result<(), String> {
    let command = format!("docker rm {}", name.into());
    executor.exec(command)?;

    Ok(())
}

/// Delete all running containers running on the location of the executor
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = api::stop_containers(&executor);
/// ```
pub fn stop_containers(executor: &dyn Executor) -> Result<(), String> {
    let containers = get_container_names(executor)?;
    for container in containers {
        let command = format!("docker stop {container}");
        executor.exec(command)?;
    }

    Ok(())
}

/// Stop specific running container running on the location of the executor
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = api::stop_container("postgres", &executor);
/// ```
pub fn stop_container<T: Into<String>>(name: T, executor: &dyn Executor) -> Result<(), String> {
    let command = format!("docker stop {}", name.into());
    executor.exec(command)?;

    Ok(())
}

/// Check if container is running by continuously polling its status every second until `timeout`
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::poll_container;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = poll_container("postgres", 30, &executor);
/// ```
pub fn poll_container<T: Into<String>>(name: T, timout: u64, executor: &dyn Executor) -> Result<(), String> {
    let name = name.into();
    let start = time::Instant::now();
    let timout = Duration::from_secs(timout);

    let command = format!("docker inspect {} -f {{{{.State.Status}}}}", name);
    
    loop {
        let result: String;
        result = executor.exec(command.clone())?;
        

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

/// Get all container names running on location of executor
///
/// # Examples
/// ```
/// use data_orchestra::core::adapters::docker::api;
/// use data_orchestra::core::adapters::local::Local;
/// use data_orchestra::core::adapters::get_container_names;
/// use data_orchestra::core::adapters::traits::Executor;
///
/// let executor = Local::new();
/// let result = get_container_names(&executor);
/// ```
pub fn get_container_names(executor: &dyn Executor) -> Result<Vec<String>, String> {
    let command = "docker container ls -a --format {{.Names}}";
    let output = executor.exec(command.to_string())?;

    let containers: Vec<String> = output
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    Ok(containers)
}


/// Get metadata of all containers running on location of executor
pub fn get_container_data(executor: &dyn Executor) -> Result<Vec<ContainerData>, String> {
    let command = "docker container ls -a --format {{.ID}}";
    let result = executor.exec(command.to_string())?;

    let mut containers = Vec::new();
    for id in result.split("\n").filter(|x| !x.is_empty()) {
        let command = format!("docker container ls -a -f id={id} --format json");
        let result = executor.exec(command)?;
        if !result.is_empty() {
            let data: ContainerData = serde_json::from_str(result.as_str()).expect("Unable to parse json to struct");
            // Filter out portainer
            // TODO: Do this step before to skip unnecessary request
            if !data.names.contains("portainer") {
                containers.push(data); 
            }
        }
    }

    Ok(containers)
}
