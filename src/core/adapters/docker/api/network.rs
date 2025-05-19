use crate::core::adapters::Runner;

pub fn get_networks(runner: &Box<dyn Runner + Send>) -> Result<Vec<String>, String> {
    let command = "docker network ls -a --format {{.Name}}";

    let output: String;

    output = runner.exec(command.to_string())?;

    let networks: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(networks)
}

pub fn create_network<T: Into<String>>(name: T, runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = format!("docker network create -d bridge {}", name.into());
    runner.exec(command)?;

    Ok(())
}

pub fn delete_networks(runner: &Box<dyn Runner + Send>) -> Result<(), String> {
    let command = "docker network prune -f";
    runner.exec(command.to_string())?;

    Ok(())
}

