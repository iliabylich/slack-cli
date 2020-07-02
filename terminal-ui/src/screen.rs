
use ui_primitives::{VisualObject, AtomicAction, Printer, PrintError, Screen};
use crate::StdoutPrinter;

type Visual = Box<dyn VisualObject>;

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
