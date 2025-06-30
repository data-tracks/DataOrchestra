use std::fmt::Display;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum FormatFields {
    Id,
    Image,
    Name,
    Json
}

impl Display for FormatFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FormatFields::Id => "ID",
            FormatFields::Image => "Image",
            FormatFields::Name => "Name",
            FormatFields::Json => "json",
        };

        write!(f, "{value}")
    }
}

#[derive(Debug, Builder)]
#[builder(build_fn(name = "build_internal", private))]
pub struct Format {
    #[builder(default, setter(custom))]
    attributes: Vec<FormatFields> 
} 

impl From<FormatBuilder> for String {
    fn from(value: FormatBuilder) -> Self {
        value.build()        
    }
}


impl FormatBuilder {
    pub fn build(&self) -> String {
        let format = self
            .build_internal()
            .expect("Unable to build format");

        let mut format_string = "--format".to_string();
        for attribute in format.attributes.iter() {
            format_string = format!("{format_string} {{{{.{attribute}}}}}");
        }

        format_string
    }

    pub fn id(&mut self) -> &mut Self {
        let vec = self.attributes.get_or_insert_default();
        vec.push(FormatFields::Id);
        self
    } 

    pub fn image(&mut self) -> &mut Self {
        let vec = self.attributes.get_or_insert_default();
        vec.push(FormatFields::Image);
        self
    }

    pub fn name(&mut self) -> &mut Self {
        let vec = self.attributes.get_or_insert_default();
        vec.push(FormatFields::Name);
        self
    } 

    pub fn json(&mut self) -> &mut Self {
        let vec = self.attributes.get_or_insert_default();
        vec.push(FormatFields::Json);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::core::adapters::FormatBuilder;

    #[test]
    pub fn format_name() {
        let output = FormatBuilder::default()
            .name()
            .build();

        assert_eq!(output, r#"--format {{.Name}}"#)
    }

    #[test]
    pub fn format_id() {
        let output = FormatBuilder::default()
            .id()
            .build();

        assert_eq!(output, r#"--format {{.ID}}"#)
    }

    #[test]
    pub fn format_image() {
        let output = FormatBuilder::default()
            .image()
            .build();

        assert_eq!(output, r#"--format {{.Image}}"#)
    }

    /*
    #[test]
    pub fn format_json() {
        let output = FormatBuilder::default()
            .id()
            .build();

        assert_eq!(output, r#"--format {{.ID}}"#)
    }
    */
}
