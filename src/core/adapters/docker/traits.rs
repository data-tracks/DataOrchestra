pub trait EnvBuilder<T, S> {
    fn validate(&self) -> bool;
    fn build(&mut self) -> Result<T, S>;
}
