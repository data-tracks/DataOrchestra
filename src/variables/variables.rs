use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

use super::tree::VariableTree;

#[derive(Debug, Deserialize, Serialize)]
pub struct Variables {
    #[serde(default)]
    variables: Map<String, Value>
}

// TODO: Try to see if this improvent is possible / viable
impl Variables {
    pub fn parse(&self, config: String) -> String {
        let mut tree = VariableTree::new(Value::Object(self.variables.clone()));
        // Start recursion to get variable tree
        Self::insert_variables(0, self.variables.clone(), &mut tree);
        let config = Self::replace_variables(tree, config);
        config
    }

    fn insert_variables(parent: usize, value: Map<String, Value>, tree: &mut VariableTree) {
        for (key, value) in value {
            let id = tree.add_child(parent, key, value.clone());
            if value.is_object() {
                let map = value.as_object().unwrap().to_owned();
                Self::insert_variables(id, map, tree);
            }
        }
    }

    fn replace_variables(tree: VariableTree, mut config: String) -> String {
        // Matches the pattern ${<variable>}
        let pattern = Regex::new(r"\$\{([a-zA-Z0-9_.]+)\}").unwrap();

        let mut variables = Vec::<(String, String)>::new();
        for variable in pattern.captures_iter(&config) {
            // Index corresponds to the <variable> inside the pattern ${<variable>}
            variables.push((variable[0].to_string(), variable[1].to_string()));
        }

        for (mut pattern_variable, variable) in variables {
            let value = tree.get_variable_value(&variable);
            if let Some(value) = value {
                pattern_variable = format!("\"{pattern_variable}\"");
                while config.contains(&pattern_variable) {
                    config = config.replace(&pattern_variable, &value.to_string());
                }
            }
            else {
                panic!("Variable {} called but neved defined", variable);
            }
        }

        config
    }
}
