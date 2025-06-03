pub mod env;

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ValueType {
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<ValueType>)
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ValueType::String(string) => string.to_owned(),
            ValueType::Number(number) => number.to_string(),
            ValueType::Array(array) => {
                let mut vec_string = String::from("[");
                for value in array.iter() {
                    vec_string.push_str(value.to_string().as_str()); 
                    vec_string.push_str(",")
                }
                vec_string.pop();
                vec_string.push_str("]");

                vec_string
            },
            ValueType::Bool(bool) => bool.to_string()
        };

        write!(f, "{}", value)
    }
}

pub fn parse_env(map: &HashMap<String, ValueType>) -> String {
    let mut env = String::new();
    for (key, value) in map {
        env = format!("{env}\n{}={}", key, value); 
    }

    env
}
