mod ui;

use ui::{Point, Rectangle, Label, Screen, TerminalScreen, FromAtomicAction, AtomicAction};

fn main() {
    let mut screen = TerminalScreen::new();

    screen.push_object(Box::new(Rectangle { top_left: Point { line: 5, col: 5}, bottom_right: Point { line: 11, col: 30 } }));
    screen.push_object(Box::new(Label { at: Point { line: 8, col: 15 }, text: String::from("hello") }));
    screen.push_object(
        Box::new(
            FromAtomicAction::new(
                &AtomicAction::MoveAt {
                    point: Point {
                        line: 20, col: 1
                    }
                }
            )
        )
    );

    screen.redraw();
}
