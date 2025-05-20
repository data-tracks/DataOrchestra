use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use ssh2::{Session, Channel};
use walkdir::{DirEntry, WalkDir};
use std::{fs, io};
use log::{debug, error};
use crate::core::adapters::ssh::ssh::Ssh;
use crate::core::adapters::traits::{Runner, Uploader};
use crate::shared::arguments::ARGS;

impl Ssh {
    pub fn new() -> Ssh {
        Ssh { session: Session::new().unwrap()  }
    }

    /// Connect to Ssh server
    ///
    /// # Note
    /// 
    /// See <https://github.com/libssh2/libssh2/blob/master/include/libssh2.h> for relevant error
    /// codes
    pub fn connect(&mut self, host: &String, port: u16, username: &String, password: Option<&String>) -> Result<(), String> {
        let address: String = format!("{}:{}", host, port);
        debug!("Connecting to Ssh client {} with {}@{}", &address, &username, password.unwrap_or(&String::new()));
        let tcp: Result<TcpStream, io::Error> = TcpStream::connect(address);
    
        if let Err(ref error) = tcp {
            error!("Unable to setup tcp stream {}", error);
        } 

        let tcp = tcp.unwrap();
        self.session.set_tcp_stream(tcp);
        
        let handshake: Result<(), ssh2::Error> = self.session.handshake();

        if let Err(ref error) = handshake {
            return Err(format!("Unsuccessful handshake {}", error));
        }
        let authentication: Result<(), ssh2::Error>;

        if let Some(password) = password {
            authentication = self.session.userauth_password(username, password);
        }
        else if let Some(ssh_key) = ARGS.get().unwrap().ssh_key.as_ref() {
            authentication = self.session.userauth_pubkey_file(username, None, Path::new(ssh_key), None);
        }
        else {
            panic!("No valid connection type given. Either provide a ssh key or a password");
        }
        
        if let Err(ref error) = authentication {
            return Err(format!("Unsuccessful authentication {}", error));
        }

        if !self.session.authenticated() {
            return Err("Session not authenticated".to_string());
        } 

        Ok(())
    }

    pub fn to_box_runner(&self) -> Box<dyn Runner + Send> {
        let runner = Box::new(self.clone()) as Box<dyn Runner + Send>;
        runner
    }
}


impl Runner for Ssh {
    /// Execute command over Ssh connection
    fn exec(&self, command: String) -> Result<String, String> {
        debug!("Executing command [{}]", &command);
        let channel: Result<Channel, ssh2::Error> = self.session.channel_session();

        if let Err(ref error) = channel {
            error!("Unable to open Ssh channel {}", error);
        }

        let mut channel = channel.unwrap();
        let exec: Result<(), ssh2::Error> = channel.exec(&command);

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

        Ok(result)
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

    /// Upload directory to remote server via Ssh. 
    ///
    /// # Return
    ///
    /// [`Result`] type with the parent directory of the files on success or error message.
    fn upload_directory(&self, dir: T, destination: S) -> Result<(), String> {
        let dir = dir.as_ref();
        let destination = destination.as_ref();
        assert!(dir.is_dir());

        let _ = self.exec(format!("mkdir {}", destination.to_str().unwrap()));

        for entry in WalkDir::new(dir) {
            if let Ok(ref entry) = entry {
                // Apply filter for DirEntry to ignore unneeded files
                if !ignore(entry) {
                    let path = format!("{}", entry.path().display()); 
                    let stripped_remote_path = path.strip_prefix(dir.to_str().unwrap());

                    let remote_path = stripped_remote_path.unwrap_or("/");

                    // Copy to / directory
                    let remote_path = format!("{}{}", destination.to_str().unwrap(), remote_path);
                    if entry.file_type().is_dir() {
                        let _ = self.exec(format!("mkdir /{}", remote_path));
                    }
                    else {
                        let result = self.upload_file(entry.path(), &Path::new(&remote_path));
                        if let Err(ref error) = result {
                            return Err(format!("Unable to upload file from directory {}", error));
                        }
                    }
                }
            }
        }
        
        Ok(())
    } 
}

/// Ignore a directory entry based on predefined filtering for folders and files
fn ignore(obj: &DirEntry) -> bool {
    let file_filter = vec!["so", "rmeta", "d", "rlib", "TAG"];
    let folder_filter = vec!["target"];
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
