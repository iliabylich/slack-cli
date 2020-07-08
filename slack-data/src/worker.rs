use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use slack_worker::{WorkerImplementation, Message, Worker};
use crate::{SlackState, SlackClient};

type Sender = mpsc::Sender<Message>;
type Receiver = mpsc::Receiver<Message>;

type WorkerState = Arc<Mutex<SlackState>>;

#[derive(Debug)]
pub struct PollSlack {
    state: WorkerState,
}

impl PollSlack {
    fn bump_uptime(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.uptime += 1;
    }

    fn process_incoming_messages(&self, receiver: &Receiver) {
        loop {
            match receiver.try_recv() {
                Ok(message) => println!("Got message {:#?}", message),
                Err(_) => break
            }
        }
    }

    fn ping_subscribers(&self, subscribers: &Vec<Sender>) {
        for subscriber in subscribers {
            subscriber.send(Message::Ping).unwrap();
        }
    }

    fn update_conversations_list(&self) {
        let slack = SlackClient::new_from_env().unwrap();
        let conversations = slack.list_conversations().unwrap();
        let mut state = self.state.lock().unwrap();
        state.all_conversations = conversations;
    }
}

impl WorkerImplementation for PollSlack {
    type State = WorkerState;

    fn new(state: Self::State) -> Self {
        Self { state }
    }

    fn tick(&mut self, receiver: &Receiver, subscribers: &Vec<Sender>) {
        thread::sleep(Duration::from_secs(2));

        self.bump_uptime();
        self.process_incoming_messages(receiver);
        self.ping_subscribers(subscribers);

        self.update_conversations_list();

        // println!("[SlackWorker] running, send ping, state = {:#?}", self.state);
    }
}

pub type SlackWorker = Worker<PollSlack>;
