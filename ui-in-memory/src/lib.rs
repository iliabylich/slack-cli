extern crate ui_abstract;

mod printer;
pub use printer::InMemoryPrinter;

#[macro_use]
mod screen;
pub use screen::InMemoryScreen;
