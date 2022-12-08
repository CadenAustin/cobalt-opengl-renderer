use std::error::Error;
use super::app::App;

pub struct Engine {
    pub app: Box<dyn App>,
}

impl Engine {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.app.run()
    }
}