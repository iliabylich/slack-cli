extern crate ui_primitives;

pub use ui_primitives::AtomicAction;

mod printer;
pub use printer::StdoutPrinter;

mod screen;
pub use screen::TerminalScreen;
