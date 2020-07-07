use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use slack_worker::{WorkerImplementation, Message, Worker as WorkerT};
use crate::SlackState;

type Sender = mpsc::Sender<Message>;
type Receiver = mpsc::Receiver<Message>;

#[derive(Debug)]
pub struct Implementation {
    state: Arc<Mutex<SlackState>>,
}

impl Default for Implementation {
    fn default() -> Self {
        let state: SlackState = Default::default();
        Self { state: Arc::new(Mutex::new(state)) }
    }
}

impl WorkerImplementation for Implementation {
    fn tick(&self, receiver: &Receiver, subscribers: &Vec<Sender>) {
        thread::sleep(Duration::from_secs(1));

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

pub type Worker = WorkerT<Implementation>;
