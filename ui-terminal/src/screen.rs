use ui_abstract::{VisualObject, AtomicAction, Printer, IoResult, Screen};
use crate::{TerminalPrinter, TerminalScreenSize, ScreenSizeError};

type Visual = Box<dyn VisualObject>;

pub struct TerminalScreen {
    pub size: TerminalScreenSize,
    objects: Vec<Visual>,
    printer: TerminalPrinter
}

impl std::fmt::Debug for TerminalScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("TerminalScreen")
            .field("size", &self.size)
            .finish()
    }
}

impl TerminalScreen {
    pub fn new() -> Result<Self, ScreenSizeError> {
        let size = TerminalScreenSize::new()?;
        Ok(Self { size, objects: vec![], printer: TerminalPrinter {} })
    }

    pub fn push_object(&mut self, object: Visual) {
        self.objects.push(object)
    }

    pub fn update_size(&mut self) -> Result<(), ScreenSizeError> {
        self.size.update()
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
