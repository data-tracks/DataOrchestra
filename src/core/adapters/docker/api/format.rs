use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct Format {
    #[builder(default)]
    id: bool,
    #[builder(default)]
    image: bool,
    #[builder(default)]
    name: bool
} 

impl Format {
    pub fn parse(&self) -> String {
        let mut format = "--format".to_string();

        if self.id {
            format = format!("{format} {{.ID}}");
        }

        if self.image {
            format = format!("{format} {{.Image}}")
        }

        format
    }
}

pub fn test() {
    let _ = FormatBuilder::default()
        .id(true)
        .build();
}
