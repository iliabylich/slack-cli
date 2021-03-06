mod from_action;
mod horizontal_line;
mod vertical_line;
mod rectangle;
mod label;
mod fixed_width_label;

pub use from_action::FromAtomicAction;
pub use horizontal_line::HorizontalLine;
pub use vertical_line::VerticalLine;
pub use rectangle::Rectangle;
pub use label::Label;
pub use fixed_width_label::FixedWidthLabel;

#[cfg(test)]
#[macro_use]
extern crate ui_in_memory;
