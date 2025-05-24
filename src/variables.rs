use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

#[derive(Debug, Deserialize, Serialize)]
pub struct Variables {
    #[serde(default)]
    variables: Map<String, Value>
}

// TODO: Try to see if this improvent is possible / viable
/*
impl Variables {
    pub fn parse(&self, config: String) {
        let mut tree: Tree<String, Value> = Tree::new(Some("variables"));
        tree.add_node(Node::new("variables", Some(serde_json::Value(self.variables.clone()))), None);
    }

    fn create_tree() {
    
    }


}
*/

/// Replace all variables in string with true value.
///
/// Variables are defined as `${<variable>}`. The variable can either be a simple string, array or map.
pub fn set_variables(variables: Variables, mut config: String) -> String {
    let variables = variables.variables;
    for (key, value) in variables.iter() {
        if value.is_object() {
            let map = value.as_object().unwrap();
            for (key_map, value_map) in map {
                // ${var.var} -> "val" or val or [ ... ] or { ... }
                let variable = format!("\"${{{}.{}}}\"", &key, key_map);
                config = replace_variables(config, &variable, &value_map.to_string());
            }
        }

        // "${var}" -> "val" or val or [ ... ] or { ... }
        let variable = format!("\"${{{}}}\"", &key);
        config = replace_variables(config, &variable, &value.to_string());
    }

    config
}

fn replace_variables(mut config: String, variable: &String, value: &String) -> String {
    while config.contains(variable) {
        config = config.replace(variable, value);
    } 

    config
}
