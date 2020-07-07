use ui_abstract::{Point, AtomicAction, Screen};
use ui_primitives::{Rectangle, Label, FromAtomicAction};
use ui_terminal::TerminalScreen;
use slack_http::{SlackResult, Worker as SlackWorker};
use slack_worker::{Message, Worker, WorkerImplementation, Sender, Receiver};

use std::sync::mpsc;
use std::time::Duration;
use std::thread;

#[derive(Debug, Default)]
struct Render {
}
impl Render {
    pub fn redraw(&self) {
        thread::sleep(Duration::from_millis(100));
        let mut screen = TerminalScreen::new().unwrap();
        println!("screen_size: {:#?}", &screen.size);

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

        screen.redraw().unwrap_or_else(|err| panic!("Failed to draw, {}", err));
    }
}
impl WorkerImplementation for Render {
    fn tick(&self, _receiver: &Receiver, _subscribers: &Vec<Sender>) {
        thread::sleep(Duration::from_secs(1));
        self.redraw();
    }
}
type RenderWorker = Worker<Render>;

fn main() -> SlackResult<()> {
    let mut handles = vec![];

    let render_worker = RenderWorker::new("Render", vec![]);

    {
        let mut render_worker = render_worker.lock().unwrap();
        handles.push(render_worker.spawn());
    }

    let mut subscribers = vec![];

    {
        let render_worker = render_worker.lock().unwrap();
        subscribers.push(render_worker.sender.clone());
    }

    // 5 subscribers
    for i in 1..5 {
        let (sender, receiver) = mpsc::channel::<Message>();
        subscribers.push(sender);

        let handle = thread::spawn(move || {
            for received in receiver {
                match received {
                    Message::Ping => println!("thread {}: ping", i),
                    Message::Updated => println!("thread {}: updated ({:#?}), uptime", i, received),
                    Message::Exit => break
                }
            }
        });

        handles.push(handle);
    }

    // 1 slack worker
    let slack_worker = SlackWorker::new("Slack", subscribers);
    {
        let mut slack_worker = slack_worker.lock().unwrap();
        handles.push(slack_worker.spawn());
    }

    for handle in handles {
        handle.join().unwrap();
    }


    Ok(())
}
