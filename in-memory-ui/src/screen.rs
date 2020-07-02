use ui::{Point, AtomicAction, VisualObject, Screen, Printer, PrintError};
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

#[macro_export]
macro_rules! assert_prints {
    ($size: expr, $visual: expr, $expected: expr) => {
        use in_memory_ui::InMemoryScreen;
        use ui::{Screen};

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
