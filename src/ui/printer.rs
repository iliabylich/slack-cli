use std::io::{self, Write};

use crate::ui::AtomicAction;

pub trait Printer {
    fn print(&mut self, action: &AtomicAction);
}

pub struct StdoutPrinter {}

impl Printer for StdoutPrinter {
    fn print(&mut self, action: &AtomicAction) {
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
        io::stdout().write(str.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
}

#[cfg(test)]
pub mod test_helper {
    use super::Printer;
    use crate::ui::AtomicAction;
    use crate::primitives::Point;

    pub struct InMemoryPrinter {
        pub lines: i32,
        pub cols: i32,
        pub state: Vec<Vec<char>>,
        pub currently_at: Point,
        pub saved_cursor: Point,
    }

    impl InMemoryPrinter {
        pub fn to_string(&self) -> String {
            let mut result = String::from("");

            for line in self.state.iter() {
                let line: String = line.into_iter().collect();
                result.push_str(&line[..]);
                result.push_str("\n");
            }

            result
        }
    }

    impl Printer for InMemoryPrinter {
        fn print(&mut self, action: &AtomicAction) {
            match action {
                AtomicAction::ClearScreen => {
                    self.state = vec![vec![' '; self.cols as usize]; self.lines as usize];
                },
                AtomicAction::MoveAt { point } => {
                    self.currently_at = point.clone();
                },
                AtomicAction::MoveUp { n } => {
                    self.currently_at = self.currently_at.up(*n);
                },
                AtomicAction::MoveDown { n } => {
                    self.currently_at = self.currently_at.down(*n);
                },
                AtomicAction::MoveRight { n } => {
                    self.currently_at = self.currently_at.right(*n);
                },
                AtomicAction::MoveLeft { n } => {
                    self.currently_at = self.currently_at.left(*n);
                },
                AtomicAction::Print { char } =>  {
                    self.state[self.currently_at.line as usize][self.currently_at.col as usize] = *char;
                    self.currently_at = self.currently_at.right(1);
                },
                AtomicAction::SaveCursor => {
                    self.saved_cursor = self.currently_at.clone();
                },
                AtomicAction::RestoreCursor => {
                    self.currently_at = self.saved_cursor.clone();
                },
            }
        }
    }
}
