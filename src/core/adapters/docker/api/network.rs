use crate::core::adapters::{command_func::output_command, Runner};

pub fn get_networks(runner: Option<&Box<dyn Runner + Send>>) -> Result<Vec<String>, String> {
    let command = "docker network ls -a --format {{.Name}}";

    let output: String;

    if let Some(runner) = runner {
        output = runner.exec(command.to_string())?;
    }
    else {
        output = output_command(command);
    }

    let networks: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(networks)
}

pub fn create_network<T: Into<String>>(name: T, runner: Option<&Box<dyn Runner + Send>>) -> Result<(), String> {
    let command = format!("docker network create -d bridge {}", name.into());
    
    if let Some(runner) = runner {
        runner.exec(command)?;
    }
    else {
        output_command(command);
    }

    Ok(())
}

pub fn delete_networks(runner: Option<&Box<dyn Runner + Send>>) -> Result<(), String> {
    let command = "docker network prune -f";
    if let Some(runner) = runner {
        runner.exec(command.to_string())?;
    }
    else {
        output_command(command);
    }

    Ok(())
}

