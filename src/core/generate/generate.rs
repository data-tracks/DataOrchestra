use crate::core::object::Object;

#[derive(Debug)]
pub struct Generate {
    /*
     * Default object information
     */
    pub object: Object,
}

impl Default for Generate {
    fn default() -> Self {
        Generate 
        { 
            object: Object::default() 
        }
    }
}
