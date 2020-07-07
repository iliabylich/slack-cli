use ui_abstract::{Point, AtomicAction, Screen};
use ui_primitives::{Rectangle, Label, FromAtomicAction};
use ui_terminal::TerminalScreen;
use slack_data::{SlackResult, SlackWorker, SlackState};
use slack_worker::{Message, Worker, Spawn, WorkerImplementation, Sender, Receiver};

use std::sync::mpsc;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct Render {
    slack_state: Arc<Mutex<SlackState>>
}
impl Render {
    pub fn redraw(&mut self) {
        let mut screen = TerminalScreen::new().unwrap();
        println!("screen_size: {:#?}", &screen.size);

        let uptime = {
            let state = self.slack_state.lock().unwrap();
            state.uptime
        };

        screen.push_object(Box::new(Rectangle { top_left: Point { line: 5, col: 5}, bottom_right: Point { line: 11, col: 30 } }));
        screen.push_object(Box::new(Label { at: Point { line: 8, col: 15 }, text: format!("hello {}", uptime) }));
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

        screen.redraw().unwrap_or_else(|err| panic!("Failed to draw, {}", err));
    }
}
impl WorkerImplementation for Render {
    type State = Arc<Mutex<SlackState>>;

    fn new(state: Arc<Mutex<SlackState>>) -> Self {
        Self { slack_state: state }
    }

    fn tick(&mut self, _receiver: &Receiver, _subscribers: &Vec<Sender>) {
        thread::sleep(Duration::from_millis(1000));
        self.redraw();
    }
}
type RenderWorker = Worker<Render>;

fn main() -> SlackResult<()> {
    let state = Arc::new(Mutex::new(SlackState::default()));

    let mut handles = vec![];

    let mut render_worker = RenderWorker::new("Render", Arc::clone(&state), vec![]);
    handles.push(render_worker.spawn());

    let mut subscribers = vec![];
    subscribers.push(render_worker.sender.clone());

    // 5 subscribers
    for i in 1..5 {
        let (sender, receiver) = mpsc::channel::<Message>();
        subscribers.push(sender);

        let handle = thread::spawn(move || {
            for received in receiver {
                match received {
                    Message::Ping => println!("thread {}: ping", i),
                    Message::Updated => println!("thread {}: updated ({:#?})", i, received),
                    Message::Exit => break
                }
            }
        });

        handles.push(handle);
    }

    // 1 slack worker
    let mut slack_worker = SlackWorker::new("Slack", Arc::clone(&state), subscribers);
    handles.push(slack_worker.spawn());

    for handle in handles {
        handle.join().unwrap();
    }


    Ok(())
}
