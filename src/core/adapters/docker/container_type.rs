use super::{Container, MultiContainer};

#[derive(Debug)]
pub enum ContainerType {
    Single(Container),
    Multiple(MultiContainer)
}
