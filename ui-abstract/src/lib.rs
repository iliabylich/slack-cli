mod point;
mod atomic_action;
mod printer;
mod screen;
mod visual;

pub use point::Point;
pub use atomic_action::AtomicAction;
pub use printer::Printer;
pub use screen::Screen;
pub use visual::VisualObject;

pub use std::io::Error as PrintError;
pub type IoResult = std::result::Result<(), PrintError>;
