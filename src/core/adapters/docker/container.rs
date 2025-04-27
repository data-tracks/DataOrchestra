impl ContainerParent {
    /// Initialise docker container based on specified data in [`Container`] struct.
    pub fn build(&mut self) -> Result<(), String>{
        // Hierarchy creation. compose > file > image
        if let CreateType::Compose { ref compose} = self.container_type {
            info!("Initializing docker container from docker compose");
            let _ = spawn_command(format!("docker compose -f {} up -d --build", compose)).wait();
        }
        else {
            assert!(self.config.has_one(), "Invalid amount of configurations. When using a image or dockerfile, please pass only one configuration");

            info!("Initializing docker container from docker image");
            let config = self.config.get_ref_mut_single();

            // Create network
            let network = create_network(config.network.clone());
            match network {
                Ok(_) => info!("Successfully created network"),
                Err(value) => error!("{}", value),
            }
           
            if let CreateType::Dockerfile { dockerfile, image_name, build_args } = &self.container_type {
                let mut build_args_string = String::new();
                if let Some(build) = build_args {
                    for (key, value) in build.iter() {
                        build_args_string = format!("{build_args_string} {}={}", key, value);
                    }
                }

                if build_args_string.is_empty() {
                    let _ = output_command(format!("docker build -t {} {}", image_name, dockerfile));
                }
                else {
                    let _ = output_command(format!("docker build --build-arg {} -t {} {}", build_args_string, image_name, dockerfile));
                }
            } 

            let image: &String = 
                match &self.container_type {
                    CreateType::Dockerfile { dockerfile: _, image_name, build_args: _ } => image_name,
                    CreateType::Image { image } => image,
                    _ => panic!(),
                };

            // Create image
            let id = output_command(format!("docker run {} -it {}", config.get_options(), image));
            config.set_id(id.trim().to_string());

            // get ip
            let ip = config.get_ip();
            match ip {
                Ok(ip) => config.meta.ip = Some(ip),
                Err(error) => error!("{}", error)
            }
        
            // Get ssh port
            let ports = output_command(format!("docker port {}", config.meta.id.as_ref().unwrap()));
            for port in ports.split("\n").filter(|x| !x.is_empty()) {
                let (int, ext) = port.split_once("/").unwrap();
                let int = int.parse::<u16>().unwrap();
                let ext = ext.split(":").last().unwrap().parse::<u16>().unwrap();
                config.add_port_map(ext, int);
            }

            // Install ssh server
            info!("Installing shh server on {}", config.get_id().unwrap());
            // Reformat sh script for linux distro
            if cfg!(target_os = "windows") {
                spawn_command(&"dos2unix src/docker/docker_ssh_init.sh".to_string());
            }

            let _ = spawn_command(&format!("docker cp src/docker/docker_ssh_init.sh {}:/", config.meta.id.as_ref().unwrap())).wait();
            let _ = status_command(&format!("docker exec {} sh ../docker_ssh_init.sh", config.meta.id.as_ref().unwrap()));
            // Start ssh server
            let _ = spawn_command(&format!("docker exec -d {} /usr/sbin/sshd -D", config.meta.id.as_ref().unwrap())).wait();

            // Sleep to wait for ssh server to properly start
            sleep(Duration::from_secs(1));
        } 
        Ok(())
    }
}

