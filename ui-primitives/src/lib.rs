extern crate ui;

mod from_action;
mod horizontal_line;
mod vertical_line;
mod rectangle;
mod label;

pub use from_action::FromAtomicAction;
pub use horizontal_line::HorizontalLine;
pub use vertical_line::VerticalLine;
pub use rectangle::Rectangle;
pub use label::Label;

#[cfg(test)]
#[macro_use]
extern crate in_memory_ui;
