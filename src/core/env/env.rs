use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Env {
    pub data: HashMap<String, Value>,
}

// Builder
impl Env {
    pub fn add_mut<T: Into<String>>(&mut self, key: T, value: Value) -> &mut Self {
        self.data.insert(key.into(), value);
        self
    } 

    pub fn add<T: Into<String>>(mut self, key: T, value: Value) -> Self {
        self.data.insert(key.into(), value);
        self
    }
}

impl Env {
    pub fn new() -> Self {
        Env { data: HashMap::new() }
    }

    /// Parse environment variables to valid string representing .env file
    pub fn parse(&self) -> String {
        let mut env = String::new();
        for (key, value) in self.data.iter() {
            let variable: String;
            // Add "" for complex values 
            if value.is_array() || value.is_object() {
                variable = format!("{key}=\"{value}\"\n");
            }
            else {
                variable = format!("{key}={value}\n");
            }
            env.push_str(variable.as_str());
        }

        // Remove trailing \n
        env.pop();

        env
    }
}


