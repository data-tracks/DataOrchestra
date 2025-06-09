use crate::core::adapters::Runner;

pub fn get_all_volumes(runner: &dyn Runner) -> Result<Vec<String>, String> {
    let command = "docker volume ls -q --format {{.Name}}";
    let output: String;

    output = runner.exec(command.to_string())?;

    let volumes: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(volumes)
}
