#[derive(Debug)]
pub struct NodeData {
    pub path: String,
    pub destination: String,
}

impl Default for NodeData {
    fn default() -> Self {
        NodeData { path: String::new(), destination: String::from("data/") }
    }
}

impl NodeData {
    pub fn new<T: Into<String>, S: Into<String>>(path: T, destination: S) -> Self {
        NodeData { path: path.into(), destination: destination.into() }
    }
}

#[derive(Debug)]
pub struct DockerData {
    pub name: String,
    pub path: String,
    pub destination: String,
    pub start: String,
    pub dependency: Option<String>
}

impl Default for DockerData {
    fn default() -> Self {
        DockerData 
        {
            name: String::new(),
            path: String::new(),
            destination: "/".to_string(),
            start: String::new(),
            dependency: None
        }
    }
}

impl DockerData {
    pub fn new<T: Into<String>, S: Into<String>, V: Into<String>, W: Into<String>>(name: T, path: S, destination: V, start: W, dependency: Option<String>) -> Self {
        DockerData 
        {
            name: name.into(),
            path: path.into(),
            destination: destination.into(),
            start: start.into(),
            dependency: dependency.map(|item| item.into())
        }
    }
}
