use std::time::Duration;
use std::thread;

use std::sync::{Arc, Mutex};
use slack_data::{SlackState};
use ui_terminal::TerminalScreen;
use ui_primitives::{Rectangle, Label, FromAtomicAction};
use ui_abstract::{Point, AtomicAction, Screen};
use slack_worker::{Worker, WorkerImplementation, Sender, Receiver};

#[derive(Debug)]
pub struct Render {
    slack_state: Arc<Mutex<SlackState>>,
    screen: TerminalScreen
}
impl Render {
    pub fn redraw(&mut self) {
        self.screen.push_object(Box::new(Label { at: Point { line: 8, col: 15 }, text: format!("hello") }));
        self.screen.redraw().unwrap_or_else(|err| panic!("Failed to draw, {}", err));
    }
}
impl WorkerImplementation for Render {
    type State = Arc<Mutex<SlackState>>;

    fn new(state: Arc<Mutex<SlackState>>) -> Self {
        let mut screen = TerminalScreen::new().unwrap();
        println!("screen_size: {:#?}", &screen.size);

        screen.push_object(Box::new(Rectangle { top_left: Point { line: 5, col: 5}, bottom_right: Point { line: 11, col: 30 } }));
        screen.push_object(Box::new(Label { at: Point { line: 8, col: 15 }, text: format!("hello") }));
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

        Self { slack_state: state, screen }
    }

    fn tick(&mut self, _receiver: &Receiver, _subscribers: &Vec<Sender>) {
        thread::sleep(Duration::from_millis(1000));
        self.redraw();
    }
}

pub type RenderWorker = Worker<Render>;
