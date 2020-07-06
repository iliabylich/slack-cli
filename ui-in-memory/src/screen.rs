use ui_abstract::{AtomicAction, VisualObject, Screen, Printer, IoResult};
use crate::InMemoryPrinter;

type Visual = Box<dyn VisualObject>;

pub struct InMemoryScreen {
    objects: Vec<Visual>,
    printer: InMemoryPrinter
}

impl InMemoryScreen {
    pub fn new(lines: i32, cols: i32) -> Self {
        InMemoryScreen {
            objects: vec![],
            printer: InMemoryPrinter::new(lines, cols)

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

#[macro_export]
macro_rules! assert_prints {
    ($size: expr, $visual: expr, $expected: expr) => {
        use ui_in_memory::InMemoryScreen;
        use ui_abstract::Screen;

        let (lines, cols) = $size;
        let mut screen = InMemoryScreen::new(lines, cols);
        screen.push_object(Box::new($visual));

        if let Err(err) = screen.draw() {
            assert!(false, format!("Failed to redraw: {}", err))
        }

        let output = $expected.into_iter().collect::<String>();

        assert_eq!(
            screen.to_string(),
            output
        );
    };
}
