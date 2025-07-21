#[cfg(test)]
mod tests {
    /*
    use std::sync::{Arc, Mutex};
    use data_orchestra::core::adapters::{ContainerBuilder, ContainerConfig, ContainerType, Runner};
    use data_orchestra::core::object::ObjectBuilder;
    use data_orchestra::core::traits::Spawner;
    use data_orchestra::core::types;
    use data_orchestra::core::types::Executables;
    use data_orchestra::core::types::Executables::Script;
    use crate::utils::DummyRunner;

    #[test]
    pub fn object_deploy() {
        let script = data_orchestra::core::types::execute::Script
        {
            name: Some("object".to_string()),
            path: "/path".to_string()
        };

        let mutex = Arc::new(Mutex::new("".to_string()));
        let dummy = DummyRunner::new(mutex.clone());

        let mut container = ContainerBuilder::default()
            .name("object")
            .image("rust")
            .build()
            .expect("Unable to build container");

        container.ssh = Some(dummy.to_box_runner());

        let executable = Executables::Script(script);
        let mut object = ObjectBuilder::default()
            .executable(executable)
            .docker_manager(ContainerType::Container(container))
            .build()
            .expect("Unable to build object");

        object.deploy();
        let output = mutex.lock().unwrap().to_string();
        assert_eq!(output, "sh /path");
    }
    */
}