use ui_abstract::{Printer, Point, AtomicAction, IoResult};


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
            let line = line.into_iter().collect::<String>();
            result.push_str(&line[..]);
            result.push_str("\n");
        }

        result
    }
}

impl Printer for InMemoryPrinter {
    fn print(&mut self, action: &AtomicAction) -> IoResult {
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
        Ok(())
    }
}
