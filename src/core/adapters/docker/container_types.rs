use super::{ComposeGroup, Container, Run};

#[derive(Debug)]
pub enum ContainerType {
    Compose(ComposeGroup),
    Container(Container)
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
        } 

        Ok(())
    }
} 
