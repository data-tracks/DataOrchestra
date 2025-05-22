use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Variables {
    #[serde(default)]
    variables: HashMap<String, Value>
}

/// Replace all variables in string with true value.
///
/// Variables are defined as `${<variable>}`. The variable can either be a simple string, array or hashmap.
pub fn set_variables(variables: Variables, mut config: String) -> String {
    let variables = variables.variables;
    for (key, value) in variables {
        // "${var}" -> "val" or val or or [ ... ] or { ... }
        let variable = format!("\"${{{}}}\"", &key);
        // Replace all mentions of the `variable`
        while config.contains(&variable) {
            config = config.replace(&variable, value.to_string().as_str());
        }
    }

    config
}
