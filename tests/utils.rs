use std::sync::{Arc, Mutex};
use data_orchestra::core::adapters::{Runner, RunnerError};

#[derive(Debug, Clone)]
pub struct DummyRunner {
    output: Arc<Mutex<String>>,
}

impl Default for DummyRunner {
    fn default() -> Self {
        let arc = Arc::new(Mutex::new(String::new()));
        DummyRunner {output: arc}
    }
}

impl DummyRunner {
    #[allow(dead_code)]
    pub fn new(mutex: Arc<Mutex<String>>) -> Self {
        DummyRunner {
            output: mutex,
        }
    }

    #[allow(dead_code)]
    pub fn get_output(&self) -> String {
        self.output.lock().unwrap().clone()
    }
}

impl Runner for DummyRunner {
    fn exec(&self, command: String) -> Result<String, RunnerError> {
        let mut output = self.output.lock().unwrap();
        *output = command.clone();
        Ok(command)
    }

    fn clone_box(&self) -> Box<dyn Runner + Send + Sync> {
        panic!()
    }
}