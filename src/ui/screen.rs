use std::io::{self, Write};

use super::VisualObject;
use super::AtomicAction;

type Visual =  Box<dyn VisualObject>;
impl Clone for Visual {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}

pub trait Screen {
    fn draw_action(&mut self, action: &AtomicAction);

    fn visuals(&self) -> &Vec<Visual>;

    fn draw(&mut self) {
        for visual in self.visuals().clone() {
            for action in visual.to_actions() {
                self.draw_action(&action);
            }
        }
    }

    fn clear(&mut self) {
        self.draw_action(&AtomicAction::ClearScreen)
    }

    fn redraw(&mut self) {
        self.clear();
        self.draw();
    }
}

pub struct TerminalScreen {
    objects: Vec<Visual>
}

impl TerminalScreen {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn push_object(&mut self, object: Box<dyn VisualObject>) {
        self.objects.push(object)
    }
}

impl Screen for TerminalScreen {
    fn draw_action(&mut self, action: &AtomicAction) {
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

    fn visuals(&self) -> &Vec<Visual> {
        self.objects.as_ref()
    }
}

#[cfg(test)]
pub mod test_helper {
    use super::Visual;
    use crate::ui::{AtomicAction, Screen, Point};

    pub struct InMemoryScreen {
        objects: Vec<Visual>,
        lines: i32,
        cols: i32,
        state: Vec<Vec<char>>,
        currently_at: Point,
        saved_cursor: Point,
    }

    impl InMemoryScreen {
        pub fn new(lines: i32, cols: i32) -> Self {
            InMemoryScreen {
                objects: vec![],
                lines,
                cols,
                state: vec![vec![' '; cols as usize]; lines as usize],
                currently_at: Point { line: 0, col: 0 },
                saved_cursor: Point { line: 0, col: 0 }
            }
        }

        pub fn push_object(&mut self, object: Visual) {
            self.objects.push(object)
        }

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

    impl Screen for InMemoryScreen {
        fn draw_action(&mut self, action: &AtomicAction) {
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

        fn visuals(&self) -> &Vec<Visual> {
            self.objects.as_ref()
        }
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_prints {
    ($size:expr, $visual:expr, $expected:expr) => {
        #[allow(unused_imports)]
        use crate::ui::{Screen, Point};
        use crate::ui::screen_helper::InMemoryScreen;

        let mut screen = InMemoryScreen::new($size[0], $size[1]);
        screen.push_object(Box::new($visual));
        screen.draw();

        let output: String = $expected.into_iter().collect();

        assert_eq!(
            screen.to_string(),
            output
        );
    };
}
