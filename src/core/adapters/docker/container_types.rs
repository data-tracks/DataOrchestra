use super::{ComposeGroup, Container, Run};

#[derive(Debug)]
pub enum ContainerType {
    Empty,
    Compose(ComposeGroup),
    Container(Container)
}

impl Default for ContainerType {
    fn default() -> Self {
        ContainerType::Empty
    }
}

impl Run for ContainerType {
    type Output = ();
    type Error = String;

    fn run(&mut self) -> Result<Self::Output, Self::Error> {
        match self {
            ContainerType::Container(container) => {
                container.run()?;
            },
            ContainerType::Compose(compose) => {
                compose.run()?;
            }
            ContainerType::Empty => {
                return Err("No container available".to_string());
            }
        } 

        Ok(())
    }
} 

impl<'a> ContainerType {
    pub fn containers_ref_vec(&'a self) -> Vec<&'a Container> {
        match self {
            ContainerType::Compose(compose) => compose.containers.iter().collect::<Vec<&'a Container>>(),
            ContainerType::Container(container) => vec![container],
            _ => Vec::new()
        } 
    }
}
