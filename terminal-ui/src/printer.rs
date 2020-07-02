use std::io::{self, Write};

use ui_primitives::{Printer, PrintError};

use ui_primitives::AtomicAction;

pub struct StdoutPrinter {}

impl Printer for StdoutPrinter {
    fn print(&mut self, action: &AtomicAction) -> Result<(), PrintError> {
        let str = match action {
            AtomicAction::ClearScreen => format!("{}", "\x1b[2J"),
            AtomicAction::MoveAt { point } => format!("\x1b[{};{}H", point.line, point.col),
            AtomicAction::MoveUp { n } => format!("\x1b[{}A", n),
            AtomicAction::MoveDown { n } => format!("\x1b[{}B", n),
            AtomicAction::MoveRight { n } => format!("\x1b[{}C", n),
            AtomicAction::MoveLeft { n } => format!("\x1b[{}D", n),
            AtomicAction::Print { char } => format!("{}", char),
            AtomicAction::SaveCursor => format!("\x1b[s"),
            AtomicAction::RestoreCursor => format!("\x1b]u"),
        };
        io::stdout().write(str.as_bytes())?;
        io::stdout().flush()
    }
}
