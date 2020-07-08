use std::time::Duration;
use std::thread;

use std::sync::{Arc, Mutex};
use slack_data::{SlackState};
use ui_terminal::TerminalScreen;
use ui_primitives::{FromAtomicAction};
use ui_abstract::{Point, AtomicAction, Screen};
use slack_worker::{Worker, WorkerImplementation, Sender, Receiver};
use crate::ConversationsList;

#[derive(Debug)]
pub struct Render {
    slack_state: Arc<Mutex<SlackState>>,
    screen: TerminalScreen
}
impl Render {
    pub fn redraw(&mut self) {
        self.screen.redraw().unwrap_or_else(|err| panic!("Failed to draw, {}", err));
    }
}
impl WorkerImplementation for Render {
    type State = Arc<Mutex<SlackState>>;

    fn new(state: Arc<Mutex<SlackState>>) -> Self {
        let mut screen = TerminalScreen::new().unwrap();
        println!("screen_size: {:#?}", &screen.size);

        let conversations_list = ConversationsList::new(
            screen.size.clone(),
            state.clone()
        );

        screen.push_object(Box::new(conversations_list));

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
        thread::sleep(Duration::from_millis(2_000));
        self.redraw();
    }
}

pub type RenderWorker = Worker<Render>;
