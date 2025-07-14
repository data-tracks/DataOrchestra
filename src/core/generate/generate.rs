use crate::core::object::Object;

use super::generate_types::{GeneratorTypeConfig}; //GeneratorType

#[derive(Debug)]
pub struct Generate {
    /*
     * Default object information
     */
    pub object: Object,
    //pub generate_type: Option<GeneratorType>,
    pub config: Option<GeneratorTypeConfig>,
}

impl Default for Generate {
    fn default() -> Self {
        Generate 
        { 
            object: Object::default(),
            //generate_type: None,
            config: None
        }
    }
}
