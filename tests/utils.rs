use std::sync::{Arc, Mutex};
use data_orchestra::core::adapters::{Executor, ExecutorError};

#[derive(Debug, Clone)]
pub struct DummyExecutor {
    output: Arc<Mutex<String>>,
}

impl Default for DummyExecutor {
    fn default() -> Self {
        let arc = Arc::new(Mutex::new(String::new()));
        DummyExecutor {output: arc}
    }
}

impl DummyExecutor {
    #[allow(dead_code)]
    pub fn new(mutex: Arc<Mutex<String>>) -> Self {
        DummyExecutor {
            output: mutex,
        }
    }

    #[allow(dead_code)]
    pub fn get_output(&self) -> String {
        self.output.lock().unwrap().clone()
    }
}

impl Executor for DummyExecutor {
    fn exec(&self, command: String) -> Result<String, ExecutorError> {
        let mut output = self.output.lock().unwrap();
        *output = command.clone();
        Ok(command)
    }

    fn clone_box(&self) -> Box<dyn Executor + Send + Sync> {
        panic!()
    }
}