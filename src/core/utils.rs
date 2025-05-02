use super::{adapters::ssh::Ssh, data::Data};

pub fn start_script(ssh: &Ssh, data: &Data) {
    if data.start.ends_with(".sh") {
        ssh.exec(format!("sh {}/{}", data.destination, data.start));
    }
    else {
        ssh.exec(format!("{}/{}", data.destination, data.start));
    }
}
