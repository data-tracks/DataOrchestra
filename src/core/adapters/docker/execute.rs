use crate::core::adapters::Runner;

pub trait DockerExecute where Self: Runner {
    fn docker_exec(&self) {
         
    }
}
