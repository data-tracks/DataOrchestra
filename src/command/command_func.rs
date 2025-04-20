use std::process::{Child, Command, ExitStatus, Stdio};
use log::debug;

pub fn spawn_command<T: Into<String>>(arg: T) -> Child {
    let command = arg.into();
    debug!("{}", format!("Running command [{}]", &command));
    let spawn: Child;
    if cfg!(target_os = "windows") {
        spawn = Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn command")
    } else {
        spawn = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn command")
    }

    spawn
}

pub fn output_command(arg: &str) -> String {
    debug!("{}", format!("Running command [{}]", arg));
    let output;
    if cfg!(target_os = "windows") {
        output = Command::new("cmd")
            .arg("/C")
            .arg(arg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Unable to output command")
    } else {
        output = Command::new("sh")
            .arg("-c")
            .arg(arg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Unable to output command")
    }
    String::from_utf8(output.stdout).unwrap()
}

pub fn status_command(arg: &str) -> ExitStatus {
    debug!("{}", format!("Running command [{}]", arg));
    let status;
    if cfg!(target_os = "windows") {
        status = Command::new("cmd")
            .arg("/C")
            .arg(arg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .expect("Unable to output command")
    } else {
        status = Command::new("sh")
            .arg("-c")
            .arg(arg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .expect("Unable to output command")
    }

    status
}

