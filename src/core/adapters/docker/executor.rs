use crate::core::adapters::{Executor, ExecutorError, Local};

#[derive(Debug)]
pub struct DockerExecutor {
    pub executor: Box<dyn Executor + Send + Sync>
}


impl Default for DockerExecutor {
    fn default() -> Self {
        DockerExecutor { executor: Box::new(Local::new()) }
    }
}

impl Executor for DockerExecutor {
    fn exec(&self, command: String) -> Result<String, ExecutorError> {
        let command = format!("docker exec {command}");
        self.executor.exec(command)
    }

    fn clone_box(&self) -> Box<dyn Executor + Send + Sync> {
        Box::new(DockerExecutor { executor: self.executor.clone_box() }) as _
    }
}