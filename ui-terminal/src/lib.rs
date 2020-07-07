extern crate ui_abstract;

mod printer;
pub use printer::TerminalPrinter;

mod screen;
pub use screen::TerminalScreen;

mod screen_size;
pub use screen_size::{TerminalScreenSize, ScreenSizeError};
