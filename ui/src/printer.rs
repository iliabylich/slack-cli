pub use std::io::Error as PrintError;
use crate::AtomicAction;

pub trait Printer {
    fn print(&mut self, action: &AtomicAction) -> Result<(), PrintError>;
}
