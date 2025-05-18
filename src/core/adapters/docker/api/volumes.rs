use crate::core::adapters::{command_func::output_command, Runner};

pub fn get_all_volumes(runner: Option<&Box<dyn Runner + Send>>) -> Result<Vec<String>, String> {
    let command = "docker volume ls -q --format {{.Name}}";
    let output: String;

    if let Some(runner) = runner {
        output = runner.exec(command.to_string())?;
    }
    else {
        output = output_command(command);
    }

    let volumes: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(volumes)
}
