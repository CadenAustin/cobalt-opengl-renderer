use std::error::Error;

pub trait App {
    fn run(&mut self) -> Result<(), Box<dyn Error>>;
}