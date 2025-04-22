use std::fs::File;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use ssh2::{Session, Channel};
use super::ssh_struct::ssh;
use std::io::Read;
use std::{fs, io};

use walkdir::WalkDir;

use log::{debug, error};


impl ssh {
    pub fn new() -> ssh {
        ssh { session: Session::new().unwrap()  }
    }

    /// Connect to ssh server
    ///
    /// # Example 
    ///
    /// ```
    /// use DataOrchestra::ssh::ssh_struct::ssh;
    /// let ssh = ssh::new();
    /// ```
    ///
    /// # Note
    /// 
    /// See https://github.com/libssh2/libssh2/blob/master/include/libssh2.h for relevant error
    /// codes
    pub fn connect(&mut self, host: &String, port: u16, username: &String, password: &String) {
        let address: String = format!("{}:{}", host, port);
        debug!("Connecting to ssh client {} with {}@{}", &address, &username, &password);
        let tcp: Result<TcpStream, io::Error> = TcpStream::connect(address);
    
        if let Err(ref error) = tcp {
            error!("Unable to setup tcp stream {}", error);
        } 

        let tcp = tcp.unwrap();
        self.session.set_tcp_stream(tcp);
        
        let handshake: Result<(), ssh2::Error> = self.session.handshake();

        if let Err(ref error) = handshake {
            error!("Unsuccessful handshake {}", error);
        }
        
        let authentication: Result<(), ssh2::Error> = self.session.userauth_password(username, password);

        if let Err(ref error) = authentication {
            error!("Unsuccessful authentication {}", error);
        }

        assert!(self.session.authenticated()); 
    }


    /// Execute command over ssh connection
    ///
    /// # Example
    ///
    /// ```
    /// use DataOrchestra::ssh::ssh_struct::ssh;
    /// let ssh = ssh::new();
    /// let result = ssh.exec("pwd");
    /// println!("{}", result);
    /// ```
    pub fn exec(&self, command: &str) -> String {
        debug!("Executing command [{}]", command);
        let channel: Result<Channel, ssh2::Error> = self.session.channel_session();

        if let Err(ref error) = channel {
            error!("Unable to open ssh channel {}", error);
        }

        let mut channel = channel.unwrap();
        let exec: Result<(), ssh2::Error> = channel.exec(command);
        
        if let Err(ref error) = exec {
            error!("Unable to execute command {}", error);
        }

        let mut result = String::new();
        let read: Result<usize, io::Error> = channel.read_to_string(&mut result);

        if let Err(ref error) = read {
            error!("Unable to read command output {}", error);
        }

        let close: Result<(), ssh2::Error> = channel.wait_close();
        if let Err(ref error) = close {
            error!("Unable to close channel {}", error);
        }

        result
    }

    
    /// Upload file to remote server via ssh
    ///
    /// # Example
    ///
    pub fn upload_file(&self, file: &Path, location: &Path) -> Result<(), ssh2::Error>{
        assert!(file.is_file());
        debug!("Uploading file {}", file.display());

        let mut local_file = File::open(file).unwrap();
        let remote_file: Result<Channel, ssh2::Error> = self.session.scp_send(location, 0o644, fs::metadata(file).unwrap().len(), None);

        if let Err(ref error) = remote_file {
            error!("Unable to upload file {}", error);
        }

        let mut remote_file = remote_file.unwrap();
        
        let mut buffer = Vec::new();
        let _ = local_file.read_to_end(&mut buffer);
        remote_file.write_all(&buffer).unwrap();

        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();  

        return Ok(());
    }

    /// Upload directory to remote server via ssh. 
    ///
    /// # Return
    ///
    /// [`Result`] type with the parent directory of the files on success or error message.
    pub fn upload_directory(&self, dir: &Path, location: &Path) -> Result<String, String> {
        assert!(dir.is_dir());
        //TODO: Reformat to make more safe
        let parent = format!("/{}/", dir.parent().unwrap().to_str().unwrap());
        let current_dir = dir.strip_prefix(&dir.parent().unwrap()).unwrap_or(Path::new("")).to_str().unwrap().to_string();
        for entry in WalkDir::new(dir) {
            if let Ok(ref entry) = entry {
                let remote_path = format!("{}{}", location.display(), entry.path().display()); 
                let stripped_remote_path = remote_path.strip_prefix(&parent);
                let remote_path = stripped_remote_path.unwrap_or(remote_path.as_str());
                // Copy to / directory
                let remote_path = format!("/{}", remote_path);
                if entry.file_type().is_dir() {
                    self.exec(format!("mkdir /{}", remote_path).as_str());
                }
                else {
                    let result = self.upload_file(entry.path(), &Path::new(&remote_path));
                    if let Err(ref error) = result {
                        return Err(format!("Unable to upload file from directory {}", error));
                    }
                }
            }
        }

        Ok(current_dir)
    }
}
