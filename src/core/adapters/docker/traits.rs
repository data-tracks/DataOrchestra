pub trait Run<R, E> {
    fn run(&mut self) -> Result<R, E>;
} 
