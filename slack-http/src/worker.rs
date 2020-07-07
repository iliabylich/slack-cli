use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use slack_worker::{WorkerImplementation, Message, Worker};
use crate::SlackState;

type Sender = mpsc::Sender<Message>;
type Receiver = mpsc::Receiver<Message>;

type WorkerState = Arc<Mutex<SlackState>>;

#[derive(Debug)]
pub struct Implementation {
    state: WorkerState
}

impl WorkerImplementation for Implementation {
    type State = WorkerState;

    fn new(state: Self::State) -> Self {
        Self { state }
    }

    fn tick(&mut self, receiver: &Receiver, subscribers: &Vec<Sender>) {
        thread::sleep(Duration::from_secs(1));

        {
            let mut state = self.state.lock().unwrap();
            state.uptime += 1;
        }

        loop {
            match receiver.try_recv() {
                Ok(message) => println!("Got message {:#?}", message),
                Err(_) => break
            }
        }

        for subscriber in subscribers {
            subscriber.send(Message::Ping).unwrap();
        }
        println!("[SlackWorker] running, send ping, state = {:#?}", self.state);
    }
}

pub type SlackWorker = Worker<Implementation>;
