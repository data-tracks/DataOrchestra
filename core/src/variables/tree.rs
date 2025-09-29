use serde_json::Value;

/// The variable tree. Represents the tree structured variable system
#[derive(Debug)]
pub struct VariableTree {
    pub root: usize,
    pub nodes: Vec<Node>
}

/// The node object. Is a item in the [`VariableTree`] object
#[derive(Debug)]
pub struct Node {
    pub variable: String,
    pub value: Value,
    pub children: Vec<usize>
}

impl VariableTree {
    pub fn new(root: Value) -> Self {
        VariableTree { root: 0, nodes: vec![Node {variable: "variables".to_string(), value: root, children: Vec::new()}] } 
    }

    pub fn get_root(&self) -> Option<&Node> {
        self.nodes.get(0)
    }

    pub fn add_child(&mut self, parent: usize, variable: String, value: Value) -> usize {
        let id = self.nodes.len();

        if let Some(parent) = self.nodes.get_mut(parent) {
            parent.children.push(id); 
        }

        let node = Node { children: Vec::new(), variable, value };
        self.nodes.push(node); 

        id
    }

    pub fn get_variable_value(&self, variable: &String) -> Option<Value> {
        let variables: Vec<&str> = variable.split(".").collect();
        let last = *variables.clone().last().unwrap(); 
        let mut current_node = self.root;

        // Iterate over given variable, in the format var.var.var. ....
        for var in variables {
            // If var exists, it should be a child of the current node
            if let Some(next_node) = self.nodes.get(current_node) {

                // Reset moved checker
                let mut moved = false;

                for child in next_node.children.clone() {
                    if let Some(child_node) = self.nodes.get(child) {
                        // Check if current child is the variable searched
                        if child_node.variable.eq(last) {
                            return Some(child_node.value.clone());
                        }
                        else if child_node.variable.eq(var) {
                            moved = true;
                            current_node = child;
                            // Break out of children loop if next node is found
                            break;
                        }
                    }
                }

                // If during the checking of the children, none was found which matched the next
                // variable, then the variable doesn't exist
                if !moved {
                    return None;
                }
            }
        }

        None 
    }
}

