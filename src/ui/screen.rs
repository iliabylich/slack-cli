
use crate::primitives::VisualObject;
use super::AtomicAction;
use super::{Printer, StdoutPrinter, PrintError};

type Visual = Box<dyn VisualObject>;

pub trait Screen {
    fn draw(&mut self) -> Result<(), PrintError>;

    fn clear(&mut self) -> Result<(), PrintError>;

    fn redraw(&mut self) -> Result<(), PrintError> {
        self.clear()?;
        self.draw()
    }
}

pub struct TerminalScreen {
    objects: Vec<Visual>,
    printer: StdoutPrinter
}

impl TerminalScreen {
    pub fn new() -> Self {
        Self { objects: vec![], printer: StdoutPrinter {} }
    }

    pub fn push_object(&mut self, object: Visual) {
        self.objects.push(object)
    }
}

impl Screen for TerminalScreen {
    fn draw(&mut self) -> Result<(), PrintError> {
        for object in self.objects.iter() {
            for action in object.to_actions() {
                self.printer.print(&action)?
            }
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<(), PrintError> {
        self.printer.print(&AtomicAction::ClearScreen)
    }
}

#[cfg(test)]
pub mod test_helper {
    use super::Visual;
    use crate::primitives::Point;
    use crate::ui::{AtomicAction, Screen, Printer, PrintError};
    use crate::ui::printer_helper::InMemoryPrinter;

    pub struct InMemoryScreen {
        objects: Vec<Visual>,
        printer: InMemoryPrinter
    }

    impl InMemoryScreen {
        pub fn new(lines: i32, cols: i32) -> Self {
            InMemoryScreen {
                objects: vec![],
                printer: InMemoryPrinter {
                    lines,
                    cols,
                    state: vec![vec![' '; cols as usize]; lines as usize],
                    currently_at: Point { line: 0, col: 0 },
                    saved_cursor: Point { line: 0, col: 0 }
                }

            }
        }

        pub fn push_object(&mut self, object: Visual) {
            self.objects.push(object)
        }

        pub fn to_string(&self) -> String {
            self.printer.to_string()
        }
    }

    impl Screen for InMemoryScreen {
        fn draw(&mut self) -> Result<(), PrintError> {
            for object in self.objects.iter() {
                for action in object.to_actions() {
                    self.printer.print(&action)?
                }
            }
            Ok(())
        }

        fn clear(&mut self) -> Result<(), PrintError> {
            self.printer.print(&AtomicAction::ClearScreen)
        }
    }

    pub fn assert_prints(screen_size: (i32, i32), visual: Visual, expected: Vec<char>) {
        let (lines, cols) = screen_size;
        let mut screen = InMemoryScreen::new(lines, cols);
        screen.push_object(visual);

        if let Err(err) = screen.draw() {
            assert!(false, format!("Failed to redraw: {}", err))
        }

        let output = expected.into_iter().collect::<String>();

        assert_eq!(
            screen.to_string(),
            output
        );
    }
}
