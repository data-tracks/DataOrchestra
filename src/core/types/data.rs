#[derive(Debug)]
pub struct Data {
    pub name: String,
    pub path: String,
    pub destination: String,
    pub start: String,
    pub dependency: Option<String>
}

impl Default for Data {
    fn default() -> Self {
        Data 
        {
            name: String::new(),
            path: String::new(),
            destination: "/".to_string(),
            start: String::new(),
            dependency: None
        }
    }
}

impl Data {
    pub fn new(name: String, path: String, destination: String, start: String, dependency: Option<String>) -> Self {
        Data 
        {
            name,
            path,
            destination,
            start,
            dependency
        }
    }
}
