use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use ssh2::{Channel, Session, Sftp};
use walkdir::{DirEntry, WalkDir};
use std::fs;
use log::{debug, error};
use crate::core::adapters::ssh::ssh::Ssh;
use crate::core::adapters::traits::{Runner, Uploader};
use crate::core::adapters::RunnerError;

use super::ssh;

impl Default for Ssh {
    fn default() -> Self {
        Ssh { session: Session::new().unwrap()  }
    }
}



impl Ssh {
    pub fn new() -> Ssh {
        Ssh { session: Session::new().unwrap()  }
    }

    /// Connect to ssh session using password 
    pub fn connect_password(&mut self, host: &String, port: u16, username: &String, password: &String) -> Result<(), ssh::SshError> {
        let address: String = format!("{host}:{port}");
        debug!("Connecting to Ssh client {} with {}@{}", &address, &username, password);
        let tcp = TcpStream::connect(address)
            .map_err(|err| ssh::SshError::Tcp(err.to_string()))?;
    
        self.session.set_tcp_stream(tcp);
        
        self.session.handshake()
            .map_err(|err| ssh::SshError::Handshake(err.to_string()))?;

        self.session.userauth_password(username, password)
            .map_err(|err| ssh::SshError::Authentication(err.to_string()))?;

        if !self.session.authenticated() {
            return Err(ssh::SshError::Authentication("Session not authenticated".to_string()));
        }

        Ok(())
    }

    /// Connect to ssh session using a private ssh key as path
    pub fn connect_ssh<T: AsRef<Path>>(&mut self, host: &String, port: u16, username: &String, ssh_key: &T) -> Result<(), ssh::SshError> {
        let address: String = format!("{host}:{port}");
        debug!("Connecting to Ssh client {} with {} and ssh_key", &address, &username);
        let tcp = TcpStream::connect(address)
            .map_err(|err| ssh::SshError::Tcp(err.to_string()))?;
    
        self.session.set_tcp_stream(tcp);
        
        self.session.handshake()
            .map_err(|err| ssh::SshError::Handshake(err.to_string()))?;

        self.session.userauth_pubkey_file(username, None, Path::new(ssh_key.as_ref()), None)
            .map_err(|err| ssh::SshError::Authentication(err.to_string()))?;

        if !self.session.authenticated() {
            return Err(ssh::SshError::Authentication("Session not authenticated".to_string()));
        }
        
        Ok(())
    }

    /// Disconnect from ssh session
    pub fn disconnect(&self) -> Result<(), String> {
        self.session.disconnect(None, "finished", None)
            .map_err(|err| format!("Unable to close session {err}"))
    }

    /// Get STFP from ssh session
    pub fn get_sftp(&self) -> Result<Sftp, String> {
        self.session.sftp().map_err(|err| err.to_string())
    } 

    /// Create and write file using STFP from ssh session
    pub fn create_sftp_file(&self, file: impl AsRef<Path>) -> Result<ssh2::File, String> {
        let sftp = self.get_sftp()?;
        let file = file.as_ref();
        debug!("Creating file {}", file.display());
        sftp.create(file.as_ref()).map_err(|err| err.to_string())
    }
}

impl Runner for Ssh {
    /// Execute command over Ssh connection
    fn exec(&self, command: String) -> Result<String, RunnerError> {
        debug!("Executing command [{}]", &command);
        let mut channel = self.session.channel_session()
            .map_err(|err| RunnerError::SessionConnect(err.to_string()))?;
        
        channel.exec(&command)
            .map_err(|err| RunnerError::CommandExecute(err.to_string(), command.clone()))?;

        let mut result = String::new();
        let _ = channel.read_to_string(&mut result)
            .map_err(|err| RunnerError::CommandRead(err.to_string(), command.clone()))?;

        channel.wait_close()
            .map_err(|err| RunnerError::SessionDisconnect(err.to_string()))?;

        Ok(result)
    }

    fn clone_box(&self) -> Box<dyn Runner + Send + Sync> {
        Box::new(self.clone()) 
    }
}   

impl<T, S> Uploader<T, S> for Ssh where 
    T: AsRef<Path>,
    S: AsRef<Path>
{
    /// Upload file to remote server via Ssh
    fn upload_file(&self, file: T, location: S) -> Result<(), String>{
        let file = file.as_ref();
        let location = location.as_ref();

        if !file.is_file() {
            panic!("File {} does not exist. Please check path", file.display());
        }

        debug!("Uploading file {} to {}", file.display(), &location.display());

        if let Some(parent) = location.parent() {
            let result = self.exec(format!("mkdir -p {}", parent.display()));
            if let Err(error) = result {
                error!("{error}");
            }
        }

        let mut local_file = File::open(file).unwrap();
        let remote_file: Result<Channel, ssh2::Error> = self.session.scp_send(location, 0o644, fs::metadata(file).unwrap().len(), None);

        if let Err(ref error) = remote_file {
            error!("Unable to upload file {error}");
        }

        let mut remote_file = remote_file.unwrap();
        
        let mut buffer = Vec::new();
        let _ = local_file.read_to_end(&mut buffer);
        remote_file.write_all(&buffer).unwrap();

        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();  

        Ok(())
    }

    /// Upload directory to remote server via Ssh. 
    ///
    /// # Return
    ///
    /// [`Result`] type with the parent directory of the files on success or error message.
    fn upload_directory(&self, dir: T, destination: S) -> Result<(), String> {
        let mut created_paths: Vec<String> = Vec::new();
        
        let dir = dir.as_ref();
        let destination = destination.as_ref();

        if !dir.is_dir() {
            panic!("Directory {} does not exist. Please check path", dir.display());
        }

        let _ = self.exec(format!("mkdir -p {}", destination.to_str().unwrap()));

        for entry in WalkDir::new(dir) {
            if let Ok(ref entry) = entry && !ignore(entry) {
                // Apply filter for DirEntry to ignore unneeded files
                let path = format!("{}", entry.path().display()); 
                let stripped_remote_path = path.strip_prefix(dir.to_str().unwrap());

                let remote_path = stripped_remote_path.unwrap_or("/");

                // Copy to / directory
                let remote_path = format!("{}{}", destination.to_str().unwrap(), remote_path);
                if entry.file_type().is_dir() && !created_paths.contains(&remote_path){
                    created_paths.push(remote_path.to_string());
                    let _ = self.exec(format!("mkdir -p {remote_path}"));
                }
                else {
                    let result = self.upload_file(entry.path(), &Path::new(&remote_path));
                    if let Err(ref error) = result {
                        return Err(format!("Unable to upload file from directory {error}"));
                    }
                }
            }
        }
        
        Ok(())
    } 
}

// TODO: Allow for user defined filters
/// Ignore a directory entry based on predefined filtering for folders and files
fn ignore(obj: &DirEntry) -> bool {
    let file_filter = ["so", "rmeta", "d", "rlib", "TAG"];
    let folder_filter = ["target", ".git"];
    let path = obj.path();
    let path_str = path.to_str().unwrap();

    // Check if file in folder 
    for f in folder_filter.iter() {
        if path_str.contains(f) {
            return true;
        }
    }

    // Check file extension
    if let Some(ext) = path.extension() {
        return file_filter.contains(&ext.to_str().unwrap_or(""));
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::core::adapters::{Runner, Ssh};

    #[test]
    #[ignore = "Should be tested manually"]
    pub fn ssh_load() {
        let mut ssh_sessions = Vec::new();
        for _ in 0..10 {
            let mut ssh = Ssh::new();
            let _ = ssh.connect_ssh(&"".to_string(), 22, &"".to_string(), &"".to_string());
            ssh_sessions.push(ssh);
        }
       
        for _ in 0..10 {
            for ssh in ssh_sessions.iter() {
                let result = ssh.exec("pwd".to_string());
                assert!(result.is_ok());
            }
        }
    }
}
