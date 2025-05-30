use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::tree::VariableTree;

/// The `Variables` struct. Holds the variables of the json file in a `serde_json::Value::Map`
/// object
#[derive(Debug, Deserialize, Serialize)]
pub struct Variables {
    #[serde(default)]
    variables: Map<String, Value>
}

impl Variables {
    /// Transform json string where variables are placed with their true value
    pub fn parse(&self, json: String) -> String {
        let mut tree = VariableTree::new(Value::Object(self.variables.clone()));
        // Start recursion to get variable tree
        Self::insert_variables(0, self.variables.clone(), &mut tree);
        let config = Self::replace_variables(tree, json);
        config
    }

    /// Recursion to create the variable tree 
    fn insert_variables(parent: usize, value: Map<String, Value>, tree: &mut VariableTree) {
        for (key, value) in value {
            let id = tree.add_child(parent, key, value.clone());
            // If value itself is a map, repeat recursion 
            if value.is_object() {
                let map = value.as_object().unwrap().to_owned();
                Self::insert_variables(id, map, tree);
            }
        }
    }

    /// Replace mentions of variables of the pattern ${<variable>} with their true value as given
    /// in the variable tree
    fn replace_variables(tree: VariableTree, mut config: String) -> String {
        // Matches the pattern ${<variable(.variable>)} where (.variable) is a possible "infinite"
        // accessing of variables inside a map 
        let pattern = Regex::new(r"\$\{([a-zA-Z0-9_.]+)\}").unwrap();

        // Get all pattern matches
        let mut variables = Vec::<(String, String)>::new();
        for variable in pattern.captures_iter(&config) {
            // Index corresponds to the <variable> inside the pattern ${<variable>}
            variables.push((variable[0].to_string(), variable[1].to_string()));
        }

        // Iterate over all pattern matches, get the value and replace all mentions of that
        // variable with its value
        for (pattern_variable, variable) in variables {
            let value = tree.get_variable_value(&variable);
            if let Some(value) = value {
                // Replace all "${<variable>}" references
                let pattern_variable_quote = format!("\"{pattern_variable}\"");
                while config.contains(&pattern_variable_quote) {
                    config = config.replace(&pattern_variable_quote, &format!("{}", value));
                }

                let mut value_string = value.to_string();
                // If value is serde_json::Value::String, then remove quotation marks because at
                // this point, the variable is concatinated with strings, therefor the quotation
                // marks are not needed
                if value.is_string() {
                    value_string = value_string.replace("\"", "");
                }
                // Replace all "...${var}..." references
                while config.contains(&pattern_variable) {
                    config = config.replace(&pattern_variable, &value_string);
                }
            }
            else {
                panic!("Variable {} called but neved defined", variable);
            }
        }

        config
    }
}
