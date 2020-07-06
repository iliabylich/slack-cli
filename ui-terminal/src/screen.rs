
use ui_abstract::{VisualObject, AtomicAction, Printer, IoResult, Screen};
use crate::{TerminalPrinter, TerminalScreenSize};

type Visual = Box<dyn VisualObject>;

pub struct TerminalScreen {
    pub size: TerminalScreenSize,
    objects: Vec<Visual>,
    printer: TerminalPrinter
}

impl TerminalScreen {
    pub fn new() -> Self {
        let size = TerminalScreenSize::new().unwrap();
        Self { size, objects: vec![], printer: TerminalPrinter {} }
    }

    pub fn push_object(&mut self, object: Visual) {
        self.objects.push(object)
    }

    pub fn update_size(&mut self) {
        self.size.update().unwrap();
    }
}

impl Screen for TerminalScreen {
    fn draw(&mut self) -> IoResult {
        for object in self.objects.iter() {
            for action in object.to_actions() {
                self.printer.print(&action)?
            }
        }
        Ok(())
    }

    fn clear(&mut self) -> IoResult {
        self.printer.print(&AtomicAction::ClearScreen)
    }
}
