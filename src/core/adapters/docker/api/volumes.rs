use crate::core::adapters::Executor;

pub fn get_all_volumes(executor: &dyn Executor) -> Result<Vec<String>, String> {
    let command = "docker volume ls -q --format {{.Name}}";
    let output: String;

    output = executor.exec(command.to_string())?;

    let volumes: Vec<String> = output
        .split("\n")
        .filter(|x| x.ne(&""))
        .map(|x| x.to_string())
        .collect();

    Ok(volumes)
}
