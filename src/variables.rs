use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Variables {
    #[serde(default)]
    variables: HashMap<String, ValueType>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ValueType {
    Single(String),
    Structured(HashMap<String, ValueType>)
}

impl ValueType {
    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    pub fn is_structured(&self) -> bool {
        matches!(self, Self::Structured(_))
    }
}

impl ToString for ValueType {
    fn to_string(&self) -> String {
        match self {
            ValueType::Single(value) => value.to_string(),
            ValueType::Structured(values) => serde_json::to_string(values).unwrap()
        }
    }
}

pub fn set_variables(variables: Variables, mut config: String) -> String {
    let variables = variables.variables;
    for (key, value) in variables {
        let variable: String;
        if value.is_single() {
            variable = format!("${{{}}}", &key);
        } 
        else {
            variable = format!("\"${{{}}}\"", &key); 
        }
        while config.contains(&variable) {
            config = config.replace(&variable, &value.to_string().as_str());

        }
    }

    config
}
