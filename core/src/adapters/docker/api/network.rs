use crate::adapters::Executor;

/// Get all docker networks running on the location of the executor
pub fn get_networks(executor: &dyn Executor) -> Result<Vec<String>, String> {
    let command = "docker network ls --format {{.Name}}";

    let output: String;

    output = executor.exec(command.to_string())?;

    let networks: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(networks)
}

/// Create docker network on the location of the executor
pub fn create_network<T: Into<String>>(name: T, executor: &dyn Executor) -> Result<(), String> {
    let command = format!("docker network create -d bridge {}", name.into());
    executor.exec(command)?;

    Ok(())
}

/// Delete all docker networks on the location of the executor
pub fn delete_networks(executor: &dyn Executor) -> Result<(), String> {
    let command = "docker network prune -f";
    executor.exec(command.to_string())?;

    Ok(())
}

/// Delete specific docker network on the location of the executor
pub fn delete_network<T: Into<String>>(network: T, executor: &dyn Executor) -> Result<(), String> {
    let command = format!("docker network rm {}", network.into());
    executor.exec(command)?;

    Ok(())
}
