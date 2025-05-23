use crate::core::adapters::Runner;

/// Get all docker networks running on the location of the runner
pub fn get_networks(runner: &Box<dyn Runner + Send>) -> Result<Vec<String>, String> {
    let command = "docker network ls --format {{.Name}}";

    let output: String;

    output = runner.exec(command.to_string())?;

    let networks: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(networks)
}


/// Create docker network on the location of the runner
pub fn create_network<T: Into<String>>(name: T, runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = format!("docker network create -d bridge {}", name.into());
    runner.exec(command)?;

    Ok(())
}

/// Delete all docker networks on the location of the runner
pub fn delete_networks(runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = "docker network prune -f";
    runner.exec(command.to_string())?;

    Ok(())
}


/// Delete specific docker network on the location of the runner
pub fn delete_network<T: Into<String>>(network: T, runner: &Box<dyn Runner + Send>) -> Result<(), String>{
    let command = format!("docker network rm {}", network.into());
    runner.exec(command)?;

    Ok(())
}
