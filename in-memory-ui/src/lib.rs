extern crate ui;

mod printer;
pub use printer::InMemoryPrinter;

#[macro_use]
mod screen;
pub use screen::{InMemoryScreen, assert_prints};
