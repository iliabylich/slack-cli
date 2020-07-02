
use crate::{PrintError};

pub trait Screen {
    fn draw(&mut self) -> Result<(), PrintError>;

    fn clear(&mut self) -> Result<(), PrintError>;

    fn redraw(&mut self) -> Result<(), PrintError> {
        self.clear()?;
        self.draw()
    }
}
